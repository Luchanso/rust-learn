#![feature(test)]

mod hello_world;

fn main() {
    hello_world::generate_file().expect("error");
}
