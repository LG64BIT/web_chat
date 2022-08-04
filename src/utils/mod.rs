use diesel::{r2d2::ConnectionManager, Connection, PgConnection};
use crate::embedded_migrations::run_with_output;
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
    pub fn get_pg_connection(&self) -> PgPooledConnection {
        self.static_data
            .db
            .get()
            .expect("Failed to retrieve DB connection from pool")
    }
}

pub fn initialize() -> AppState {
    let db_pool = get_connection_pool();

    let state = AppState {
            static_data: Arc::new(StaticData { db: db_pool }),
        };
    let connection = state.get_pg_connection();
    run_with_output(&connection, &mut std::io::stdout()).expect("Running migration error!");
    state
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn get_connection_pool() -> PgPoolConnection {
    dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool.")
}
