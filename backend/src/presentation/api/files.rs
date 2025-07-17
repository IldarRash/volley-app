use rocket::{post, Data};
use rocket::data::{ToByteUnit};
use rocket::http::{ContentType, Status};
use std::fs;
use uuid::Uuid;

const UPLOAD_DIR: &str = "static/uploads";

#[post("/upload", data = "<data>")]
pub async fn upload(data: Data<'_>, content_type: &ContentType) -> Result<String, Status> {
    fs::create_dir_all(UPLOAD_DIR).map_err(|_| Status::InternalServerError)?;

    let ext = match content_type.extension().map(|s| s.as_str()) {
        Some("png") => "png",
        Some("jpg") | Some("jpeg") => "jpg",
        _ => return Err(Status::BadRequest),
    };

    let file_name = format!("{}.{}", Uuid::new_v4(), ext);
    let file_path = format!("{}/{}", UPLOAD_DIR, file_name);
    
    data.open(10.mebibytes()).into_file(&file_path).await
        .map_err(|e| {
            eprintln!("Failed to save file: {:?}", e);
            Status::InternalServerError
        })?;

    Ok(format!("/static/uploads/{}", file_name))
} 