#[derive(clap::ValueEnum, Clone, Debug, Copy)]
pub enum SystemEnvironment {
    Development,
    Production,
}

#[derive(clap::Parser)]
pub struct Config {
    #[clap(long, env, value_enum)]
    pub sys_env: SystemEnvironment,

    #[clap(long, env, default_value = "0.0.0.0")]
    pub hostname: String,

    #[clap(long, env, default_value = "40126")]
    pub port: u16,

    #[clap(long, env)]
    pub cors_origin: String,

    #[clap(long, env)]
    pub use_root_access: bool,

    #[clap(long, env)]
    pub root_access_token: String,

    #[clap(long, env)]
    pub database_url: String,

    #[clap(long, env)]
    pub run_migrations: bool,
}