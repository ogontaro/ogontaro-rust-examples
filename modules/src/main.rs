mod functions;
mod models;

mod inner {
    pub fn inner_hello_world() {
        println!("hello world by inner module");
    }
}

use inner::inner_hello_world;

fn main() {
    inner_hello_world();
    functions::functions_hello_world();
    let task = models::Task {
        name: String::from("table"),
        finished: false,
    };
    println!("{:?}", task);
    // this is not work
    // functions::hello_world_private();
}
