//! App state and database pool connection
use crate::{embedded_migrations::run_with_output, errors::ShopError};
use diesel::{r2d2::ConnectionManager, Connection, PgConnection};
use dotenv::dotenv;
use std::sync::Arc;

pub type PgPooledConnection = r2d2::PooledConnection<ConnectionManager<diesel::PgConnection>>;
pub type PgPoolConnection = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct StaticData {
    pub db: PgPoolConnection,
}

#[derive(Clone)]
pub struct AppState {
    pub static_data: Arc<StaticData>,
}

impl AppState {
    /// Function for getting static reference to database
    pub fn get_pg_connection(&self) -> Result<PgPooledConnection, ShopError> {
        Ok(self.static_data.db.get()?)
    }
}

/// Creating db pool and running migrations if necessary
pub fn initialize() -> AppState {
    let db_pool = get_connection_pool();

    let state = AppState {
        static_data: Arc::new(StaticData { db: db_pool }),
    };
    let connection = state
        .get_pg_connection()
        .expect("Failed to retrieve DB connection from pool");
    run_with_output(&connection, &mut std::io::stdout()).expect("Running migration error!");
    state
}

/// Establishing single postgres database connection, when pool cannot be used
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

/// Function for returning connection pool, reading from .env file
pub fn get_connection_pool() -> PgPoolConnection {
    dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool.")
}
