pub use inner::{dev_tools, static_files};

#[cfg(debug_assertions)]
mod inner {
    use {actix_files::Files, actix_web::web::block, std::process::Command};

    pub fn static_files() -> Files {
        Files::new("/", "./www").index_file("index.html")
    }

    pub async fn dev_tools() {
        let cmd = block(move || {
            let mut process = Command::new("npm")
                .args(&["run", "dev"])
                .spawn()
                .expect("Cannot execute npm run dev");
            process.wait()
        });
        let error = cmd.await
            .map(|exit_status| !exit_status.success())
            .unwrap_or(true);
        if error {
            panic!("Error executing npm run dev");
        }
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

    pub async fn dev_tools() {}
}
