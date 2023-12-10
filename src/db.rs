use actix_web::web::Data;
use diesel::{r2d2::{ConnectionManager, self}, PgConnection};

pub type Pool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_pool() -> Data<Pool> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env var must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool: Pool = r2d2::Pool::builder().build(manager).expect("Failed to create DB Pool");
    Data::new(pool.clone())
}