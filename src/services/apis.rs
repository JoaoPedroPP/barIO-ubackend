use actix_multipart::Multipart;
use futures_util::StreamExt;
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