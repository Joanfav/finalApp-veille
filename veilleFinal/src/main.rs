use actix_web::{App, HttpServer};
use actix_cors::Cors;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_query;
use dotenvy::dotenv;
use std::{env, fs};
use chrono::Utc;
use vue_mock::image_model::NewImage;
use crate::vue_mock::schema::images;

use diesel::insert_into;

mod vue_mock;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Chargement de l'URL de la base de données depuis le fichier .env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL doit être défini dans .env");

    // Initialisation de la connexion à la base de données
    let mut connection = PgConnection::establish(&database_url)
        .expect("Erreur de connexion à la base de données");

    // Supprime et recrée la table images
    initialize_database(&mut connection);
    println!("Table `images` recréée avec succès !");

    println!("Démarrage du serveur sur http://127.0.0.1:8080");

    // Configuration et lancement du serveur HTTP avec CORS
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:3001")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["Content-Type", "Accept", "Authorization"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .configure(vue_mock::image_controller::init)  // Configure les routes pour les images
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

/// Fonction pour initialiser la base de données : supprime et recrée la table images.
fn initialize_database(connection: &mut PgConnection) {
    let drop_query = "DROP TABLE IF EXISTS images;";
    sql_query(drop_query)
        .execute(connection)
        .expect("Erreur lors de la suppression de la table `images`");

    let create_table_query = r#"
        CREATE TABLE images (
            id SERIAL PRIMARY KEY,
            filepath VARCHAR NOT NULL,
            file_content BYTEA NOT NULL,
            rotation INTEGER NOT NULL DEFAULT 0,
            brightness INTEGER NOT NULL DEFAULT 100,
            crop_x INTEGER,
            crop_y INTEGER,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
    "#;
    sql_query(create_table_query)
        .execute(connection)
        .expect("Erreur lors de la création de la table `images`");

    let image_path3 = "image/pp2.jpeg";

    let file_content3 = match fs::read(image_path3) {
        Ok(content3) => content3,
        Err(err) => panic!("Erreur lors de la lecture du fichier image : {:?}", err),
    };

    let mut connection = establish_connection();


    let image3 = NewImage {
        filepath: image_path3.to_string(),
        file_content: file_content3,
        created_at: Utc::now().naive_utc() - chrono::Duration::days(5),
        rotation: 0,
        brightness: 0,
        crop_x: Option::from(150),
        crop_y: Option::from(150),
    };

    insert_into(images::table)
        .values(&[image3])
        .execute(&mut connection)
        .expect("Error inserting images into database");

}

/// Fonction pour établir la connexion à la base de données.
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL doit être défini dans .env");
    PgConnection::establish(&database_url)
        .expect("Erreur de connexion à la base de données")
}
