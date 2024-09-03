#[derive(clap::ValueEnum, Clone, Debug, Copy)]
pub enum OperatingEnvironment {
    Development,
    Production,
}

#[derive(clap::Parser)]
pub struct Config {
    #[clap(long, env, value_enum)]
    pub op_env: OperatingEnvironment,

    #[clap(long, env, default_value = "40126")]
    pub port: u16,

    #[clap(long, env)]
    pub cors_origin: String,

    #[clap(long, env)]
    pub database_path: String,
}