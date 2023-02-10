use std::env;
use actix_multipart::Multipart;
use futures_util::StreamExt;
use rdkafka::{
    producer::{BaseProducer, BaseRecord},
    ClientConfig,
};
use actix_web::{
    HttpResponse,
    Error,
    web,
    http::StatusCode
};

pub async fn upload_cos(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // let p = format!("{}", payload);
    // println!("{:?}", payload);
    while let Some(item) = payload.next().await {
        let mut field = item?;
        println!("{:?}", field);
        // let filename = content_disposition.get_filename();
        // let filepath = format!("./tmp/{filename}");
    }
    Ok(HttpResponse::Ok().body("ssss"))
}

pub async fn trigger_kafka() -> Result<HttpResponse, Error> {
    let kafka_servers = env::var("KAFKA_SERVERS").expect("KAFKA_SERVERS not set");
    let kafka_username = env::var("KAFKA_USERNAME").expect("KAFKA_USERNAME not set");
    let kafka_password = env::var("KAFKA_PASSWORD").expect("KAFKA_PASSWORD not set");
    let kafka_producer_topic = env::var("KAFKA_PRODUCER_TOPIC").expect("KAFKA_PRODUCER_TOPIC not set");

    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", kafka_servers)
        .set("sasl.username", kafka_username)
        .set("sasl.password", kafka_password)
        .set("security.protocol", "SASL_SSL")
        .set("sasl.mechanisms", "PLAIN")
        .create()
        .expect("Invalid Kafka Producer Config");

    match producer.send(BaseRecord::to(&kafka_producer_topic).key("msg").payload("data")) {
        Ok(_) => {
            println!("Submited to Kafka");
            Ok(HttpResponse::Ok().body("uuuu"))
        },
        Err(_) => {
            println!("Msg could not be sent to kafka");
            Ok(HttpResponse::InternalServerError().body("Error"))
        }
    }
}