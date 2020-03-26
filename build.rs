use actix_web_static_files::NpmBuild;

fn main() {
    NpmBuild::new(".")
        .install().unwrap()
        .run("build").unwrap()
        .target("./client/dist")
        .to_resource_dir()
        .build().unwrap();
}
