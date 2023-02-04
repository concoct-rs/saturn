#[cfg(not(target_os = "android"))]
#[tokio::main]
async fn main() {
    concoct::render::run(saturn::app)
}
