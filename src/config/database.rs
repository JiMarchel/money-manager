pub struct DatabaseConfig {
    pub database_name: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}
