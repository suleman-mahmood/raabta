use std::net::TcpListener;

use env_logger::Env;
use forge::{configuration::get_configuration, startup::run};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?.await
}
