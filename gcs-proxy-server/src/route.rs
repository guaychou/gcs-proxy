//use super::model::Response;
use super::model::RequestModel;
use crate::error::AppError;
use actix_web::{get, http::header, web, web::Bytes, HttpResponse};
use futures::stream;
use futures::StreamExt;
use gcs_proxy_deps::cloud_storage::GcsClient;

#[get("/_/download/{filename}")]
async fn download_async(
    gcs: web::Data<GcsClient>,
    request: web::Path<RequestModel>,
) -> Result<HttpResponse, AppError> {
    let file_stream = gcs.get_object_stream(request.filename.as_str()).await?;

    let stream = stream::try_unfold(file_stream, |mut file_stream| async {
        if let Some(byte) = file_stream.next().await {
            let formatted_byte = Bytes::from([byte.unwrap()].to_vec());
            Ok::<_, AppError>(Some((formatted_byte, file_stream)))
        } else {
            Ok(None)
        }
    });
    let header_disposition_value = format!("attachment; filename={}", request.filename);
    Ok(HttpResponse::Ok()
        .append_header((header::CONTENT_DISPOSITION, header_disposition_value))
        .streaming(Box::pin(stream)))
}
