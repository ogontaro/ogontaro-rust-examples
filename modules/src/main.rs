mod functions;
mod inner {
    pub fn inner_hello_world() {
        println!("hello world by inner module");
    }
}

use inner::inner_hello_world;
use functions::functions_hello_world;

fn main() {
    inner_hello_world();
    functions_hello_world();

    // this is not work
    // functions::hello_world_private();
}
