use concoct::render::run;
use saturn::app;

#[cfg(not(target_os = "android"))]
#[tokio::main]
async fn main() {
    run(app)
}
