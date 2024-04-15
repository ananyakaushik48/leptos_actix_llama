#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use rust_ai::app::*;

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    println!("listening on http://{}", &addr);

    // Loading in the css styles for tailwind
    #[get("/main.css")]
    async fn css() -> impl Responder {
        actix_files::NamedFile::open_async("./style/rust-ai.css")
    }

    let model= web::Data::new(get_language_model());
    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .app_data(model.clone())
            .service(css)
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", site_root))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(web::Data::new(leptos_options.to_owned()))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

cfg_if::cfg_if!{
    if #[cfg(feature = "ssr")] {
        use llm::models::Llama;
        use actix_web::*;
        use std::env;
        use dotenv::dotenv;

        fn get_language_model() -> Llama {
            use std::path::PathBuf;
            dotenv().ok();
            let model_path = env::var("MODEL_PATH").expect("MODEL_PATH must be set");

            llm::load::<Llama>(
                &PathBuf::from(&model_path),
                llm::TokenizerSource::Embedded,
                Default::default(),
                llm::load_progress_callback_stdout,
            )
            .unwrap_or_else(|err| {
                panic!("Failed to load model from {model_path:?}: {err}")
            })
        }
    }
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use rust_ai::app::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
