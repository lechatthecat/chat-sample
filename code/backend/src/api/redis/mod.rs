use actix_web::{web, HttpResponse, Responder};
use actix::prelude::*;
use redis::{cluster_async::ClusterConnection, cluster::ClusterClient, cluster::ClusterConfig};

pub struct RedisActor {
    conn: ClusterConnection,
}

impl RedisActor {
    pub async fn new(redis_url: Vec<&str>) -> Self {
        let client = ClusterClient::new(redis_url).unwrap(); // should be in env
        let config = ClusterConfig::new()
                                        .set_connection_timeout(std::time::Duration::from_secs(1))
                                        .set_response_timeout(std::time::Duration::from_secs(1));
        let conn = client.get_async_connection_with_config(config).await.unwrap();

        RedisActor { conn }
    }
}

#[derive(Message, Debug)]
#[rtype(result = "Result<Option<String>, redis::RedisError>")]
struct InfoCommand;

#[derive(Message, Debug)]
#[rtype(result = "Result<Option<String>, redis::RedisError>")]
pub struct GetCommand {
    pub key: String,
}

#[derive(Message, Debug)]
#[rtype(result = "Result<bool, redis::RedisError>")]
pub struct HasCommand {
    pub key: String,
}

#[derive(Message, Debug)]
#[rtype(result = "Result<Option<String>, redis::RedisError>")]
pub struct SetCommand {
    pub key: String,
    pub value: String,
    pub ex: Option<usize>, // Optional expiration time in seconds
}

impl Handler<InfoCommand> for RedisActor {
    type Result = ResponseFuture<Result<Option<String>, redis::RedisError>>;

    fn handle(&mut self, _msg: InfoCommand, _: &mut Self::Context) -> Self::Result {
        let mut con = self.conn.clone();
        let cmd = redis::cmd("INFO");
        let fut = async move {
            cmd.query_async(&mut con).await
        };
        Box::pin(fut)
    }
}

impl<'a> Handler<GetCommand> for RedisActor {
    type Result = ResponseFuture<Result<Option<String>, redis::RedisError>>;

    fn handle(&mut self, msg: GetCommand, _: &mut Self::Context) -> Self::Result {
        let mut con = self.conn.clone();
        let key = msg.key.to_string();

        let fut = async move {
            redis::cmd("GET")
                .arg(key)
                .query_async(&mut con)
                .await
        };

        Box::pin(fut)
    }
}

// Implement the `HasCommand` to check if a key exists
impl Handler<HasCommand> for RedisActor {
    type Result = ResponseFuture<Result<bool, redis::RedisError>>;

    fn handle(&mut self, msg: HasCommand, _: &mut Self::Context) -> Self::Result {
        let mut con = self.conn.clone();
        let key = msg.key.to_string();

        let fut = async move {
            let exists: bool = redis::cmd("EXISTS")
                .arg(key)
                .query_async(&mut con)
                .await?;
            Ok(exists)
        };

        Box::pin(fut)
    }
}

// Modify the `SetCommand` to accept an expiration
impl Handler<SetCommand> for RedisActor {
    type Result = ResponseFuture<Result<Option<String>, redis::RedisError>>;

    fn handle(&mut self, msg: SetCommand, _: &mut Self::Context) -> Self::Result {
        let mut con = self.conn.clone();
        let key = msg.key.to_string();
        let value = msg.value.to_string();
        let ex = msg.ex;

        let fut = async move {
            if let Some(expiration) = ex {
                // If expiration is provided, use `SETEX`
                redis::cmd("SETEX")
                    .arg(key)
                    .arg(expiration)
                    .arg(value)
                    .query_async(&mut con)
                    .await
            } else {
                // If no expiration, use regular `SET`
                redis::cmd("SET")
                    .arg(key)
                    .arg(value)
                    .query_async(&mut con)
                    .await
            }
        };

        Box::pin(fut)
    }
}

impl Actor for RedisActor {
    type Context = Context<Self>;
}

pub async fn _info(redis: web::Data<Addr<RedisActor>>) -> impl Responder {
    let res = redis.send(InfoCommand).await.unwrap().unwrap().unwrap();
    HttpResponse::Ok().body(res)
}

pub async fn get<'a>(
    redis: &'a web::Data<Addr<RedisActor>>,
    key: &'a str // Take ownership of the key
) -> Result<String, Box<dyn std::error::Error>> {
    // Send a GetCommand to the RedisActor, move the key into the command
    match redis.send(GetCommand { key: key.to_string() }).await {
        // If the message was sent successfully and the response is Ok
        Ok(Ok(Some(value))) => Ok(value), // Return the value fetched from Redis
        // If the response contains an error from Redis
        Ok(Ok(None)) => Err("Key not found in Redis".into()), // Handle key not found
        Ok(Err(redis_error)) => Err(Box::new(redis_error)), // Handle Redis errors
        // If the actor communication fails
        Err(mailbox_error) => Err(Box::new(mailbox_error)), // Handle Actix mailbox errors
    }
}

pub async fn has<'a>(
    redis: &'a web::Data<Addr<RedisActor>>,
    key: &'a str // Take ownership of the key
) -> Result<bool, Box<dyn std::error::Error>> {
    // Send a GetCommand to the RedisActor, move the key into the command
    match redis.send(HasCommand { key: key.to_string() }).await {
        // If the message was sent successfully and the response is Ok
        Ok(Ok(true)) => Ok(true), // Return the value fetched from Redis
        // If the response contains an error from Redis
        Ok(Ok(false)) => Ok(false), // Handle key not found
        Ok(Err(redis_error)) => Err(Box::new(redis_error)), // Handle Redis errors
        // If the actor communication fails
        Err(mailbox_error) => Err(Box::new(mailbox_error)), // Handle Actix mailbox errors
    }
}

pub async fn setex<'a> (
    redis: &'a web::Data<Addr<RedisActor>>,
    key: &'a str,
    value: &'a str,
    ex: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Send a GetCommand to the RedisActor, move the key into the command
    match redis.send(SetCommand { key: key.to_string(), value: value.to_string(), ex }).await {
        // If the message was sent successfully and the response is Ok
        Ok(Ok(_)) => Ok(()), // Return the value fetched from Redis
        Ok(Err(redis_error)) => Err(Box::new(redis_error)), // Handle Redis errors
        // If the actor communication fails
        Err(mailbox_error) => Err(Box::new(mailbox_error)), // Handle Actix mailbox errors
    }
}