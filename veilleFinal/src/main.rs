use actix_cors::Cors;
use actix_web::{App, HttpServer};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_query;
use dotenvy::dotenv;
use std::env;
use crate::vue_mock::image_controller;
mod vue_mock;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL doit être défini dans .env");

    // Initialisation de la base de données
    let mut connection = PgConnection::establish(&database_url)
        .expect("Erreur de connexion à la base de données");

    // Supprimez et recréez la table `images`
    initialize_database(&mut connection);

    println!("Table `images` recréée avec succès !");
    println!("Démarrage du serveur sur http://127.0.0.1:8080");

    // Configuration et lancement du serveur
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["Content-Type"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .configure(image_controller::init) // Configuration des routes
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

/// Initialise la base de données : supprime et recrée la table `images`.
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
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
    "#;
    sql_query(create_table_query)
        .execute(connection)
        .expect("Erreur lors de la création de la table `images`");
}

/// Établit une connexion à la base de données.
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL doit être défini dans .env");

    PgConnection::establish(&database_url)
        .expect("Erreur de connexion à la base de données")
}
