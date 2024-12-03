use actix_web::{web, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures_util::stream::StreamExt;
use crate::vue_mock::image_service::{save_image, fetch_images_from_db, test_image};

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
        .route("/images", web::get().to(get_images));
}

async fn upload_image(
    mut payload: Multipart,  // On prend juste Multipart
) -> impl Responder {
    let mut file_content = Vec::new();
    let mut form_data = ImageForm {
        name: String::new(),
        rotation: 0,
        brightness: 100,
        crop_x: None,
        crop_y: None,
    };

    while let Some(item) = payload.next().await {
        let mut field = item.expect("Error reading file");

        if field.name() == "file" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("Error reading chunk");
                file_content.extend_from_slice(&data);
            }
        } else if field.name() == "name" {
            let data = field.next().await.expect("Error reading chunk").expect("No data");
            form_data.name = String::from_utf8_lossy(&data).to_string();
        } else if field.name() == "rotation" {
            let data = field.next().await.expect("Error reading chunk").expect("No data");
            form_data.rotation = String::from_utf8_lossy(&data).parse().unwrap_or(0);
        } else if field.name() == "brightness" {
            let data = field.next().await.expect("Error reading chunk").expect("No data");
            form_data.brightness = String::from_utf8_lossy(&data).parse().unwrap_or(100);
        } else if field.name() == "crop_x" {
            let data = field.next().await.expect("Error reading chunk").expect("No data");
            form_data.crop_x = String::from_utf8_lossy(&data).parse().ok();
        } else if field.name() == "crop_y" {
            let data = field.next().await.expect("Error reading chunk").expect("No data");
            form_data.crop_y = String::from_utf8_lossy(&data).parse().ok();
        }
    }

    if file_content.is_empty() {
        return HttpResponse::BadRequest().body("No file uploaded");
    }

    save_image(
        &form_data.name,
        file_content,
        form_data.rotation,
        form_data.brightness,
        form_data.crop_x,
        form_data.crop_y,
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
async fn test_image_controller(
    mut payload: Multipart,  // On prend juste Multipart
) -> impl Responder {
    let mut file_content = Vec::new();
    let mut form_data = ImageForm {
        name: String::new(),
        rotation: 0,
        brightness: 100,
        crop_x: None,
        crop_y: None,
    };

    while let Some(item) = payload.next().await {
        let mut field = item.expect("Error reading file");

        if field.name() == "file" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("Error reading chunk");
                file_content.extend_from_slice(&data);
            }
        } else if field.name() == "name" {
            let data = field.next().await.expect("Error reading chunk").expect("No data");
            form_data.name = String::from_utf8_lossy(&data).to_string();
        } else if field.name() == "rotation" {
            let data = field.next().await.expect("Error reading chunk").expect("No data");
            form_data.rotation = String::from_utf8_lossy(&data).parse().unwrap_or(0);
        } else if field.name() == "brightness" {
            let data = field.next().await.expect("Error reading chunk").expect("No data");
            form_data.brightness = String::from_utf8_lossy(&data).parse().unwrap_or(100);
        } else if field.name() == "crop_x" {
            let data = field.next().await.expect("Error reading chunk").expect("No data");
            form_data.crop_x = String::from_utf8_lossy(&data).parse().ok();
        } else if field.name() == "crop_y" {
            let data = field.next().await.expect("Error reading chunk").expect("No data");
            form_data.crop_y = String::from_utf8_lossy(&data).parse().ok();
        }
    }

    if file_content.is_empty() {
        return HttpResponse::BadRequest().body("No file uploaded");
    }

    // Process the image and get the transformed base64 string
    let transformed_image_base64 = test_image(
        &form_data.name,
        file_content,
        form_data.rotation,
        form_data.brightness,
        form_data.crop_x,
        form_data.crop_y,
    );

    match transformed_image_base64 {
        Ok(base64_image) => HttpResponse::Ok().body(base64_image),
        Err(e) => {
            eprintln!("Error processing image: {:?}", e);
            HttpResponse::InternalServerError().body("Error processing image")
        }
    }
}





