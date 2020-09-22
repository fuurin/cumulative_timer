pub mod connection {
    use std::env;
    use diesel::pg::PgConnection;
    use diesel::Connection;

    pub fn establish() -> PgConnection {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    }
}