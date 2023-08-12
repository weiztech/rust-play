use tokio_postgres::{NoTls, Error, Connection};

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("src/bin/postgre_migrations");
}


#[tokio::main]
async fn main() {
    // Connect to the database.
    let (mut client, connection) =
        tokio_postgres::connect(
            "host=localhost user=user_rust password=user_rust dbname=rust", NoTls
        ).await.unwrap();
    println!("{:?}", client);
    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = &connection.await {
            eprintln!("connection error: {}", e);
        };
    });

    embedded::migrations::runner().run_async(  &mut client).await.unwrap();
}
