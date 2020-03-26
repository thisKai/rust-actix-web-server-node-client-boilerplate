use {actix_web_static_files::NpmBuild, std::io};

fn main() -> io::Result<()> {
    NpmBuild::new(".")
        .install()?
        .run("build")?
        .target("./client/dist")
        .to_resource_dir()
        .build()
}
