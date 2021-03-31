use std::env;

fn load_kernel_linux()
{
    let target = env::var("TARGET").unwrap();
    println!("Target={}", target);
}

fn main() {
    println!("[build.rs::Linux Headers] starting importing...");
    load_kernel_linux();
    println!("[build.rs::Linux Headers] DONE");
}
