pub use inner::static_files;

#[cfg(debug_assertions)]
mod inner {
    use actix_files::Files;

    pub fn static_files() -> Files {
        Files::new("/", "./client/dist").index_file("index.html")
    }
}

#[cfg(not(debug_assertions))]
mod inner {
    use {actix_web_static_files::ResourceFiles, std::collections::HashMap};

    include!(concat!(env!("OUT_DIR"), "/generated.rs"));

    pub fn static_files() -> ResourceFiles {
        let generated = generate();

        ResourceFiles::new("/", generated)
    }
}
