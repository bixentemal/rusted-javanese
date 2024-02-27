use std::env;

fn main() {
    let java_home = env::var("JAVA_HOME").unwrap();
    println!("cargo:rustc-link-search={}/jre/lib/server", java_home);
    println!("cargo:rustc-link-lib=jvm");
}