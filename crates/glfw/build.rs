fn main() {
    if let Ok(path) = std::env::var("GLFW_PATH") {
        print!("cargo:rustc-link-search={path}/lib/");
    } else {
        println!("cargo::error=Failed to find GLFW_PATH")
    }
}
