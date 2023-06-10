use std::net::TcpListener;
use sqlx::postgres::PgPoolOptions;
use zerotoproduction::{startup::run, configuration::get_configuration, telemetry::get_subscriber, telemetry::init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber(
        "zerotoproduction".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());
    
    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())
}



// Newsletter:
// - As an author,
// I want to send emails to all of my subscribers,
// so that I can notify them when i've released a new book?
// - As a subscriber,
// I want to subscribe to news from someone who's work i care about,
// So that I can receive email updates when new content is coming out.

// Glossary
// monomorphization - every time a generic function is called with a concrete set of types, the Rust compiler
// will create a copy of the function body replacing the generic type params with concrete types.
// allows for optimization of each instance of the function body. Resultant compiled code is no diff at runtime
// than if we were to implement separate functions for each concrete type ourselves. => zero-cost abstraction
// This functions a good representation of something rust does that we don't see done in other languages very often,
// types have to be known at compile time. (De)serializers in other languages tend to use runtime reflection
// as a means to get information about types you want to (de)serialize. This incurs a performance overhead, and so
// is NOT a zero-cost abstraction.

