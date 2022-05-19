// Actix web is a small, pragmatic, and extremely fast web framework for Rust.
use actix_web::{
    middleware::Logger,
    App,
    HttpServer,
};
use openssl::ssl::{
    SslAcceptor,
    SslFiletype,
    SslMethod,
};

pub async fn server() -> std::io::Result<()> {
    // It loads the .env file located in the environment's current directory or its parents in sequence.
    dotenv::dotenv().ok();
    /* A simple logger configured via environment variables which writes to stdout or stderr,
    for use with the logging facade exposed by the log crate. */
    env_logger::init();

    /* Create the application state
    String is used here, but it can be anything
    Invoke in hanlders using data: AppState<'_, String> */
    let state = crate::state::new_state::<String>();

    /* listenfd is a crate that provides support for working with externally managed and passed file descriptors.
    This lets you work with systems that support socket activation or similar. */
    let mut listenfd = listenfd::ListenFd::from_env();

    // 機能追加: load TLS keys
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(&crate::config::CONFIG.ssl_pkey_path, SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file(&crate::config::CONFIG.ssl_cert_path)
        .unwrap();

    let mut server = HttpServer::new(move || {
        App::new()
            /* Registers middleware */
            // Middleware for logging request and response info to the terminal.
            .wrap(Logger::default())
            // Cross-origin resource sharing (CORS) for Actix applications
            .wrap(actix_cors::Cors::default().supports_credentials())
            // ::
            .wrap(crate::auth::get_identity_service())
            // ::
            .configure(crate::cache::add_cache)
            // ::
            .configure(crate::database::add_pool)
            // ::
            .configure(crate::routes::routes)
            // Set application level arbitrary data item.
            .app_data(state.clone())
    });

    server = if let Some(listener) = listenfd.take_tcp_listener(0)? {
        // log::info!("listener");
        server.workers(10).listen(listener)?
    } else {
        // log::info!("bind");
        if crate::config::CONFIG.ssl_enabled {
            server
                .workers(10)
                .bind_openssl(&crate::config::CONFIG.ssl_server, builder)?
        } else {
            server.workers(10).bind(&crate::config::CONFIG.server)?
        }
    };
    server.run().await
}
