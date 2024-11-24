use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::sql_query;
use dotenvy::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL doit être défini dans .env");

    let mut connection = PgConnection::establish(&database_url)
        .expect("Erreur de connexion à la base de données");

    let drop_query = "DROP TABLE IF EXISTS images;";
    sql_query(drop_query)
        .execute(&mut connection)
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
        .execute(&mut connection)
        .expect("Erreur lors de la création de la table `images`");

    println!("Table `images` recréée avec succès !");
}
