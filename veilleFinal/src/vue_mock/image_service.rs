use serde_json::{json};
use diesel::prelude::*;
use crate::vue_mock::image_model::{Image, NewImage};
use crate::establish_connection;
use crate::vue_mock::schema::images::dsl::*;
use image::{DynamicImage, ImageFormat};
use base64::{encode};
use std::io::Cursor;
use chrono::Utc;
use diesel::associations::HasTable;
use diesel::sql_query;
use diesel::sql_types::Integer;
use crate::vue_mock::schema::images;

pub fn save_image(
    path_file: &str,
    file_content_param: Vec<u8>,
    rotation_param: i32,
    brightness_param: i32,
    crop_x_param: Option<i32>,
    crop_y_param: Option<i32>,
) {
    let connection = &mut establish_connection();

    let new_image = NewImage {
        filepath: path_file.to_string(),
        file_content: file_content_param,
        rotation: rotation_param,
        brightness: brightness_param,
        crop_x: crop_x_param,
        crop_y: crop_y_param,
        created_at: chrono::Utc::now().naive_utc(),
    };

    // Insertion dans la table 'images'
    diesel::insert_into(images)
        .values(&new_image)
        .execute(connection)
        .expect("Erreur lors de l'insertion dans `images`");
}

pub fn test_image(
    path_file: &str,
    file_content_param: Vec<u8>,
    rotation_param: i32,
    brightness_param: i32,
    crop_x_param: Option<i32>,
    crop_y_param: Option<i32>,
) -> Result<String, image::ImageError> {

    let new_image = Image {
        id: 0,
        filepath: path_file.to_string(),
        file_content: file_content_param,
        rotation: rotation_param,
        brightness: brightness_param,
        crop_x: crop_x_param,
        crop_y: crop_y_param,
        created_at: chrono::Utc::now().naive_utc(),
    };

    Ok(apply_image_transformations(&new_image)?)
}

pub fn apply_image_transformations(image: &Image) -> Result<String, image::ImageError> {
    let mut img = image::load_from_memory(&image.file_content)?;

    img = match image.rotation % 360 {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => img,
    };

    img = reduce_brightness(&img, image.brightness);

    if let (Some(cx), Some(cy)) = (image.crop_x, image.crop_y) {
        img = img.resize_exact(cx as u32, cy as u32, image::imageops::FilterType::Lanczos3);
    }

    let mut transformed_content = Vec::new();
    let mut cursor = Cursor::new(&mut transformed_content);
    img.write_to(&mut cursor, ImageFormat::Jpeg)?;
    Ok(encode(&transformed_content))
}

pub fn fetch_images_from_db() -> Result<Vec<serde_json::Value>, image::ImageError> {
    let connection = &mut establish_connection();

    let fetched_images = images
        .load::<Image>(connection)
        .expect("Erreur lors du chargement des images");

    let transformed_images: Vec<serde_json::Value> = fetched_images
        .iter()
        .map(|image| {
            match apply_image_transformations(image) {
                Ok(transformed_image) => json!({
                    "id": image.id,
                    "name": image.filepath,
                    "image": transformed_image
                }),
                Err(e) => {
                    eprintln!("Erreur lors de l'application des transformations: {}", e);
                    json!({
                        "id": image.id,
                        "name": image.filepath,
                        "image": null
                    })
                }
            }
        })
        .collect();

    Ok(transformed_images)
}

fn reduce_brightness(image: &DynamicImage, value: i32) -> DynamicImage {
    image.brighten(value)
}

pub fn fetch_image_from_db(image_id: i32) -> Result<serde_json::Value, diesel::result::Error> {
    let connection = &mut establish_connection();

    let image = images.find(image_id).first::<Image>(connection)?;

    match apply_image_transformations(&image) {
        Ok(transformed_image) => Ok(json!({
            "id": image.id,
            "name": image.filepath,
            "image": transformed_image,
            "rotation": image.rotation,
            "brightness": image.brightness,
            "crop_x": image.crop_x,
            "crop_y": image.crop_y,
            "created_at": image.created_at
        })),
        Err(e) => {
            eprintln!("Erreur lors des transformations: {}", e);
            Ok(json!({
                "id": image.id,
                "name": image.filepath,
                "image": null,
                "rotation": image.rotation,
                "brightness": image.brightness,
                "crop_x": image.crop_x,
                "crop_y": image.crop_y,
                "created_at": image.created_at
            }))
        }
    }
}

pub fn fetch_images_by_size(
    crop_x_param: i32,
    crop_y_param: i32,
) -> Result<Vec<serde_json::Value>, diesel::result::Error> {
    let connection = &mut establish_connection();

    let fetched_images = sql_query(
        "SELECT id, filepath, file_content, rotation, brightness, crop_x, crop_y, created_at
         FROM images WHERE (crop_x < 300) AND (crop_y < 300)"
    )
        .bind::<Integer, _>(crop_x_param)
        .bind::<Integer, _>(crop_y_param)
        .load::<Image>(connection)?;

    let transformed_images: Vec<serde_json::Value> = fetched_images
        .iter()
        .map(|image| {
            match apply_image_transformations(image) {
                Ok(transformed_image) => json!( {
                    "id": image.id,
                    "name": image.filepath,
                    "image": transformed_image,
                }),
                Err(e) => {
                    eprintln!("Erreur lors de l'application des transformations: {}", e);
                    json!( {
                        "id": image.id,
                        "name": image.filepath,
                        "image": null
                    })
                }
            }
        })
        .collect();

    Ok(transformed_images)
}


pub fn fetch_images_by_size_petite() -> Result<Vec<serde_json::Value>, diesel::result::Error> {
    fetch_images_by_size(300, 300)
}


pub fn fetch_images_by_date() -> Result<Vec<serde_json::Value>, diesel::result::Error> {
    let connection = &mut establish_connection();
    let current_date = Utc::now().naive_utc().date();
    let start_of_day = current_date.and_hms_opt(0, 0, 0).unwrap();
    let end_of_day = current_date.and_hms_opt(23, 59, 59).unwrap();

    let filtered_images: Vec<Image> = images::table
        .filter(images::created_at.ge(start_of_day))
        .filter(images::created_at.le(end_of_day))
        .load::<Image>(connection)?;

    let transformed_images: Vec<serde_json::Value> = filtered_images
        .iter()
        .map(|image| {
            match apply_image_transformations(image) {
                Ok(transformed_image) => json!({
                    "id": image.id,
                    "name": image.filepath,
                    "image": transformed_image,
                }),
                Err(e) => {
                    eprintln!("Erreur lors de l'application des transformations: {}", e);
                    json!({
                        "id": image.id,
                        "name": image.filepath,
                        "image": null
                    })
                }
            }
        })
        .collect();

    Ok(transformed_images)
}
