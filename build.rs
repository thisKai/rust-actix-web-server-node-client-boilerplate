#[cfg(debug_assertions)]
fn main() {}

#[cfg(not(debug_assertions))]
fn main() -> std::io::Result<()> {
    use actix_web_static_files::NpmBuild;
    NpmBuild::new(".")
        .install()?
        .run("build")?
        .target("./www")
        .to_resource_dir()
        .build()
}
