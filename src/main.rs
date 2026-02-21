use sea_orm::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::connect("sqlite://database.db").await.unwrap();

    let ping = db.ping().await;

    if ping.is_err() {
        print!("Error in database connection");
    }

    Ok(())
}
