#[allow(dead_code)]
mod application;

use application::Application;

fn main() {
    let mut app: Application = Application::new();
    app.run();
}
