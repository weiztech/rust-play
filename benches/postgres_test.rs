use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::env;
use deadpool_postgres::{GenericClient, PoolConfig};

use tokio_postgres::{NoTls, Error};
use tokio_postgres::row::Row;
// This is a struct that tells Criterion.rs to use the "futures" crate's current-thread executor
use tokio::runtime::Runtime;

use sqlx;
use sqlx::postgres::PgPoolOptions;
use postgres_from_row::FromRow;




#[derive(sqlx::FromRow, Debug, FromRow)]
struct User {
    id: i32,
    name: String,
    address: String
}


async fn sqlx_postgres()  {
    let db_config = env::var("RUST_POSTGRESQL_URI").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&db_config).await.unwrap();

    // return as default Row
    // let rows = sqlx::query("SELECT * from users")
    //    .fetch_all(&pool).await;

    // return as User struct
    let rows = sqlx::query_as::<_, User>("SELECT * from users")
        .fetch_all(&pool).await;

    // println!("Rows, {:?}", rows.unwrap().len());
}


fn sqlx_postgres_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("sqlx_postgres_benchmark", |b| {
        {
            // Insert a call to `to_async` to convert the bencher to async mode.
            // The timing loops are the same as with the normal bencher.
            b.to_async(&rt).iter(|| sqlx_postgres());
        }
    }
    );
}



// By default, tokio_postgres uses the tokio crate as its runtime.
async fn tokio_postgres() {
    use deadpool_postgres::{Config, Manager, ManagerConfig, Pool, RecyclingMethod, Runtime, PoolConfig};

    let mut cfg = Config::new();
    cfg.dbname = Some("rust".to_string());
    cfg.user = Some("user_rust".to_string());
    cfg.password = Some("user_rust".to_string());
    cfg.pool = Some(PoolConfig{ max_size: 1, timeouts: Default::default() });
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    let client = pool.get().await.unwrap();
    /*let db_config = env::var("RUST_POSTGRESQL_URI").unwrap();
    // Connect to the database.
    let (client, connection) = tokio_postgres::connect(
        db_config.as_str(), NoTls
    ).await.unwrap();

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });*/

    // Now we can execute a simple statement that just returns its parameter.
    let rows = client
        .query("SELECT * from users", &[])
        .await;

    let struct_rows: Vec<User> = rows.unwrap().into_iter().map(
        |row| User::from_row(&row)
    ).collect();
    // println!("Rows, {:?}", rows.unwrap()[0]);
}

fn tokio_postgres_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("tokio_postgres_benchmark", |b| {
         {
            // Insert a call to `to_async` to convert the bencher to async mode.
            // The timing loops are the same as with the normal bencher.
            b.to_async(&rt).iter(|| tokio_postgres());
        }
    }
    );
}



criterion_group!(benches, sqlx_postgres_benchmark, tokio_postgres_benchmark);
criterion_main!(benches);