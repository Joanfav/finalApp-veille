use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GalleryItem {
    name: String,
    image_path: String,
}


/// Récupère la liste des éléments de la galerie.
#[get("/gallery")]
async fn get_gallery() -> impl Responder {
    // Exemple de réponse simulée
    HttpResponse::Ok().json(vec!["item1", "item2"])
}

/// Ajoute un nouvel élément à la galerie.
#[post("/gallery")]
async fn create_gallery(item: web::Json<GalleryItem>) -> impl Responder {
    // Retournez l'élément reçu pour confirmation
    HttpResponse::Created().json(item.0)
}

/// Initialise les routes pour le module `image_controller`.
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_gallery).service(create_gallery);
}
