use actix::prelude::*;
use actix_web::{web, HttpResponse, Responder};
use redis_cluster_async::{Client as ClusterClient, redis::{AsyncCommands, RedisError}};
use std::sync::Arc;

pub struct RedisActor {
    client: Arc<ClusterClient>,
}

impl RedisActor {
    pub async fn new(redis_nodes: Vec<&str>) -> Self {
        let client = ClusterClient::open(redis_nodes).unwrap();
        RedisActor { client: Arc::new(client) }
    }
}

impl Actor for RedisActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<Option<String>, RedisError>")]
pub struct GetCommand {
    pub key: String,
}

#[derive(Message)]
#[rtype(result = "Result<(), RedisError>")]
pub struct SetCommand {
    pub key: String,
    pub value: String,
}

#[derive(Message)]
#[rtype(result = "Result<(), RedisError>")]
pub struct SetExCommand {
    pub key: String,
    pub value: String,
    pub ex: usize,
}

#[derive(Message)]
#[rtype(result = "Result<bool, RedisError>")]
pub struct HasCommand {
    pub key: String,
}

impl Handler<GetCommand> for RedisActor {
    type Result = ResponseFuture<Result<Option<String>, RedisError>>;
    fn handle(&mut self, msg: GetCommand, _: &mut Self::Context) -> Self::Result {
        let client = self.client.clone();
        let key = msg.key;
        Box::pin(async move {
            let mut conn = client.get_connection().await?;
            conn.get(key).await
        })
    }
}

impl Handler<SetCommand> for RedisActor {
    type Result = ResponseFuture<Result<(), RedisError>>;
    fn handle(&mut self, msg: SetCommand, _: &mut Self::Context) -> Self::Result {
        let client = self.client.clone();
        let key = msg.key;
        let value = msg.value;
        Box::pin(async move {
            let mut conn = client.get_connection().await?;
            conn.set(key, value).await
        })
    }
}

impl Handler<SetExCommand> for RedisActor {
    type Result = ResponseFuture<Result<(), RedisError>>;
    fn handle(&mut self, msg: SetExCommand, _: &mut Self::Context) -> Self::Result {
        let client = self.client.clone();
        let key = msg.key;
        let value = msg.value;
        let ex = msg.ex;
        Box::pin(async move {
            let mut conn = client.get_connection().await?;
            conn.set_ex(key, value, ex).await
        })
    }
}

impl Handler<HasCommand> for RedisActor {
    type Result = ResponseFuture<Result<bool, RedisError>>;
    fn handle(&mut self, msg: HasCommand, _: &mut Self::Context) -> Self::Result {
        let client = self.client.clone();
        let key = msg.key;
        Box::pin(async move {
            let mut conn = client.get_connection().await?;
            conn.exists(key).await
        })
    }
}

pub async fn get(
    redis: web::Data<Addr<RedisActor>>,
    key: web::Path<String>,
) -> impl Responder {
    match redis.send(GetCommand { key: key.into_inner() }).await {
        Ok(Ok(Some(val))) => HttpResponse::Ok().body(val),
        Ok(Ok(None))      => HttpResponse::NotFound().body("not found"),
        Ok(Err(e))        => HttpResponse::InternalServerError().body(format!("{:?}", e)),
        Err(_)            => HttpResponse::InternalServerError().body("actor error"),
    }
}

pub async fn set(
    redis: web::Data<Addr<RedisActor>>,
    item: web::Json<SetCommand>
) -> impl Responder {
    match redis.send(item.into_inner()).await {
        Ok(Ok(_))   => HttpResponse::Ok().body("OK"),
        Ok(Err(e))  => HttpResponse::InternalServerError().body(format!("{:?}", e)),
        Err(_)      => HttpResponse::InternalServerError().body("actor error"),
    }
}

pub async fn setex(
    redis: web::Data<Addr<RedisActor>>,
    item: web::Json<SetExCommand>
) -> impl Responder {
    match redis.send(item.into_inner()).await {
        Ok(Ok(_))   => HttpResponse::Ok().body("OK"),
        Ok(Err(e))  => HttpResponse::InternalServerError().body(format!("{:?}", e)),
        Err(_)      => HttpResponse::InternalServerError().body("actor error"),
    }
}

pub async fn has(
    redis: web::Data<Addr<RedisActor>>,
    key: web::Path<String>
) -> impl Responder {
    match redis.send(HasCommand { key: key.into_inner() }).await {
        Ok(Ok(exists)) => HttpResponse::Ok().body(exists.to_string()),
        Ok(Err(e))     => HttpResponse::InternalServerError().body(format!("{:?}", e)),
        Err(_)         => HttpResponse::InternalServerError().body("actor error"),
    }
}
