#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // 구성 읽기를 초기화 한다.
    let settings = config::Config::builder()
        // `configuration.yaml` 파일로부터 구성값을 추가한다.
        .add_source(
            config::File::new("configuration.yaml", config::FileFormat::Yaml, )
        )
        .build()?;

    // 읽은 구성값을 Settings 타입으로 변환한다.
    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}