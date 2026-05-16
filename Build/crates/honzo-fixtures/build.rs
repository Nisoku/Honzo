#[path = "src/main.rs"]
mod fixture_generator;

fn main() {
    fixture_generator::generate_all();
}
