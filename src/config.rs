#[derive(clap::ValueEnum, Clone, Debug, Copy)]
pub enum OperatingEnvironment {
    Development,
    Production,
}

#[derive(clap::Parser)]
pub struct Config {
    #[clap(long, env("KOTRAN_OP_ENV"), value_enum)]
    pub op_env: OperatingEnvironment,

    #[clap(long, env("KOTRAN_HOST"), default_value = "0.0.0.0")]
    pub host: String,

    #[clap(long, env("KOTRAN_PORT"), default_value = "40126")]
    pub port: u16,

    #[clap(long, env("KOTRAN_CORS_ORIGIN"))]
    pub cors_origin: String,

    #[clap(long, env("KOTRAN_DATABASE_URL"))]
    pub database_url: String,
}