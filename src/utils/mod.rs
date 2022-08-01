use diesel::{r2d2::ConnectionManager, Connection, PgConnection};
use dotenv::dotenv;
use std::sync::Arc;

pub type PgDbConnection = r2d2::PooledConnection<ConnectionManager<diesel::PgConnection>>;

pub struct StaticData {
    pub db: r2d2::Pool<ConnectionManager<PgConnection>>,
}

#[derive(Clone)]
pub struct AppState {
    pub static_data: Arc<StaticData>,
}

impl AppState {
    pub fn get_pg_connection(&self) -> PgDbConnection {
        self.static_data
            .db
            .get()
            .expect("Failed to retrieve DB connection from pool")
    }
}

pub fn initialize() -> AppState {
    let db_pool = get_connection_pool();
    
    AppState {
        static_data: Arc::new(StaticData {
            db: db_pool,
        }),
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn get_connection_pool() -> r2d2::Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool.")
}
