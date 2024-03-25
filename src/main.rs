use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Logs with Telemetry
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Configuration
    let configuration = get_configuration().expect("Failed to read configuration.");

    // DB Connection
    let connection_pool = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());

    // App listening address
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    startup::run(listener, connection_pool)?.await
}
