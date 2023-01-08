use crate::models::Task;

pub fn functions_hello_world() {
    println!("hello world by functions.rs");
}

fn hello_world_private() {
    println!("hello world by functions.rs");
}

pub fn debug_tag(task: &Task) {
    println!("{:?}", task);
}
