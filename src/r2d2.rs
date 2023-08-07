use diesel::{r2d2, PgConnection};

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<r2d2::ConnectionManager<PgConnection>>;

pub type DbConnectionManger = r2d2::ConnectionManager<PgConnection>;

pub fn init_db(database_url: &str) -> DbPool {
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("could not build connection pool")
}

#[cfg(test)]
mod tests {
    use super::init_db;
    use diesel::RunQueryDsl;
    use diesel::{prelude::*, sql_query, sql_types::Text};
    #[derive(QueryableByName)]
    struct SqlVersion {
        #[diesel(sql_type = Text)]
        pub version: String,
    }

    #[tokio::main]
    #[test]
    async fn test_init_db() {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("Expected DATABASE_URL to be set");
        println!("database_url: {}", database_url.clone());
        let pool = init_db(database_url.as_str());
        let mut conn = pool.get().expect("could not get connection");
        let version = sql_query("SELECT version()").get_result::<SqlVersion>(&mut conn);

        assert!(version.is_ok());
        let version = version.unwrap();
        println!("database version {}", version.version);
    }
}
