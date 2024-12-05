use actix_web::{web, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures_util::stream::StreamExt;
use crate::vue_mock::image_model::{NewImage};
use crate::vue_mock::image_service::{save_image, fetch_images_from_db, test_image, fetch_image_from_db};
use base64::{decode};

// Structure pour contenir les données du formulaire
#[derive(serde::Deserialize)]
pub struct ImageForm {
    pub name: String,
    pub rotation: i32,
    pub brightness: i32,
    pub crop_x: Option<i32>,
    pub crop_y: Option<i32>,
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/upload", web::post().to(upload_image))
        .route("/testImage", web::post().to(test_image_controller))
        .route("/images", web::get().to(get_images))
        .route("/getImages/{id}", web::get().to(get_image_detail));
}

async fn read_image(mut payload: Multipart) -> Result<NewImage, HttpResponse> {
    let mut file_content = Vec::new();
    let mut form_data = ImageForm {
        name: String::new(),
        rotation: 0,
        brightness: 100,
        crop_x: None,
        crop_y: None,
    };

    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|_| HttpResponse::BadRequest().body("Error reading file"))?;

        match field.name() {
            "file" => {
                while let Some(chunk) = field.next().await {
                    let data = chunk.map_err(|_| HttpResponse::BadRequest().body("Error reading chunk"))?;
                    if let Ok(decoded) = decode(&data) {
                        file_content.extend(decoded);
                    } else {
                        file_content.extend_from_slice(&data);
                    }
                }
            }
            "name" => {
                let data = field.next().await
                    .ok_or_else(|| HttpResponse::BadRequest().body("Missing name field"))?
                    .map_err(|_| HttpResponse::BadRequest().body("Error reading name"))?;
                form_data.name = String::from_utf8_lossy(&data).to_string();
            }
            "rotation" => {
                let data = field.next().await
                    .ok_or_else(|| HttpResponse::BadRequest().body("Missing rotation field"))?
                    .map_err(|_| HttpResponse::BadRequest().body("Error reading rotation"))?;
                form_data.rotation = String::from_utf8_lossy(&data).parse().unwrap_or(0);
            }
            "brightness" => {
                let data = field.next().await
                    .ok_or_else(|| HttpResponse::BadRequest().body("Missing brightness field"))?
                    .map_err(|_| HttpResponse::BadRequest().body("Error reading brightness"))?;
                form_data.brightness = String::from_utf8_lossy(&data).parse().unwrap_or(100);
            }
            "crop_x" => {
                let data = field.next().await
                    .ok_or_else(|| HttpResponse::BadRequest().body("Missing crop_x field"))?
                    .map_err(|_| HttpResponse::BadRequest().body("Error reading crop_x"))?;
                form_data.crop_x = String::from_utf8_lossy(&data).parse().ok();
            }
            "crop_y" => {
                let data = field.next().await
                    .ok_or_else(|| HttpResponse::BadRequest().body("Missing crop_y field"))?
                    .map_err(|_| HttpResponse::BadRequest().body("Error reading crop_y"))?;
                form_data.crop_y = String::from_utf8_lossy(&data).parse().ok();
            }
            _ => continue,
        }
    }

    Ok(NewImage {
        filepath: form_data.name,
        file_content,
        rotation: form_data.rotation,
        brightness: form_data.brightness,
        crop_x: form_data.crop_x,
        crop_y: form_data.crop_y,
        created_at: chrono::Utc::now().naive_utc(),
    })
}

async fn test_image_controller(part_file: Multipart) -> impl Responder {
    let image_form = read_image(part_file).await.unwrap();
    match test_image(
        &*image_form.filepath,
        image_form.file_content.clone(),
        image_form.rotation,
        image_form.brightness,
        image_form.crop_x,
        image_form.crop_y,
    ) {
        Ok(base64_image) => HttpResponse::Ok().body(base64_image),
        Err(e) => {
            eprintln!("Error processing image: {:?}", e);
            HttpResponse::InternalServerError().body("Error processing image")
        }
    }
}

async fn upload_image(part_file: Multipart) -> impl Responder {
    let image_form = read_image(part_file).await.unwrap();
    save_image(
        &*image_form.filepath,
        image_form.file_content.clone(),
        image_form.rotation,
        image_form.brightness,
        image_form.crop_x,
        image_form.crop_y,
    );
    HttpResponse::Ok().body("Image uploaded")
}
pub async fn get_images() -> impl Responder {
    let images =  fetch_images_from_db();

    match images {
        Ok(images) => HttpResponse::Ok().json(images),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
pub async fn get_image_detail(id: web::Path<i32>) -> impl Responder {
    let image =  fetch_image_from_db(id.into_inner());

    match image {
        Ok(image) => HttpResponse::Ok().json(image),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}






