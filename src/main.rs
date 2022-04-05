extern crate actix_web;
extern crate env_logger;
extern crate redis;
extern crate log;
extern crate url;
extern crate serde;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use log::info;
use url::Url;
use log::LevelFilter;
use serde::{Serialize, Deserialize};

use redis::Commands;
use redis::Client as RedisClient;

use std::env;
use std::fs;
use std::io;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct EnvConfig {
  region: String,
  environment: String,
}

impl EnvConfig {
  pub fn init() -> EnvConfig {
    EnvConfig {
      region: env::var("REGION").unwrap_or("local".into()),
      environment: env::var("ENVIRONMENT").unwrap_or("development".into()),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
struct AppConfig {
  hostname: String,
  port: u16,
  mysql_database: String,
  redis_enabled: bool,
}

impl AppConfig {
  pub fn from_file(filename: &str) -> Result<Self, io::Error> {
    let contents = fs::read_to_string(filename)?;
    let config = serde_yaml::from_str(&contents).expect("");
    Ok(config)
  }

  pub fn get_address(&self) -> String {
    format!("{}:{}", self.hostname, self.port).into()
  }
}

// NB: Ordinarily these would be secrets we *would never print*
#[derive(Serialize, Deserialize, Debug)]
struct RedisConfig {
  hostname: String,
  port: u16,
  password: String,
}

impl RedisConfig {
  pub fn from_file(filename: &str) -> Result<Self, io::Error> {
    let contents = fs::read_to_string(filename)?;
    let config = serde_yaml::from_str(&contents).expect("");
    Ok(config)
  }

  pub fn get_url(&self) -> Url {
    let url = format!("redis://{}@{}:{}", self.password, self.hostname, self.port);
    Url::parse(&url).expect("cannot parse")
  }
}

fn greet(req: HttpRequest) -> impl Responder {
  let name = req.match_info()
    .get("name")
    .unwrap_or("World");
  format!("Hello {}!", &name)
}

fn status(_req: HttpRequest) -> impl Responder {
  format!("Status: OK")
}

fn start_http_server(app_config: &AppConfig) {
  let address = app_config.get_address();
  info!("Starting server on {}...", address);

  HttpServer::new(|| {
    App::new()
      .route("/", web::get().to(greet))
      .route("/status", web::get().to(status))
      .route("/{name}", web::get().to(greet))
  })
  .bind(address)
  .expect("Can not bind to address/port")
  .run()
  .unwrap();
}

fn load_from_redis(redis_config: &RedisConfig) {
  info!("Querying redis for `test_key`...");
  let url = redis_config.get_url();
  info!("Redis URL: {}", url);
  let client = RedisClient::open(url).expect("bad config");
  let mut con = client.get_connection().expect("cannot connect");
  let value : String = con.get("test_key").unwrap_or("".into());
  info!("Value from redis: {:?}", value);
}

fn main() {
  env_logger::builder()
    .filter(None, LevelFilter::Info)
    .init();

  info!("Application init");

  let env_config = EnvConfig::init();
  info!("Environment config: {:?}", env_config);

  info!("Reading config.yaml ...");
  let app_config = AppConfig::from_file("config.yaml").expect("error");
  info!("App config: {:?}", app_config);

  info!("Reading redis secrets (redis.yaml) ...");
  let redis_config = RedisConfig::from_file("redis.yaml").expect("error");
  info!("Redis config (don't print real secrets) : {:?}", redis_config);

  if app_config.redis_enabled {
    load_from_redis(&redis_config);
  }

  info!("Waiting for dependencies...");
  thread::sleep(Duration::from_millis(5_000));

  start_http_server(&app_config);
}

