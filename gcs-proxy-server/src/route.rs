//use super::model::Response;
use super::model::RequestModel;
use crate::error::AppError;
use actix_web::body::SizedStream;
use actix_web::{get, http::header, web, HttpResponse};
use gcs_proxy_deps::cloud_storage::GcsClient;

#[get("/_/download/{filename}")]
async fn download_async(
    gcs: web::Data<GcsClient>,
    request: web::Path<RequestModel>,
) -> Result<HttpResponse, AppError> {
    let file_stream = gcs.get_object_stream(request.filename.as_str()).await?;
    let stream = SizedStream::new(
        gcs.get_file_size(request.filename.as_str()).await?,
        Box::pin(file_stream),
    );
    let header_disposition_value = format!("attachment; filename={}", request.filename);
    Ok(HttpResponse::Ok()
        .append_header((header::CONTENT_DISPOSITION, header_disposition_value))
        .body(stream))
}
