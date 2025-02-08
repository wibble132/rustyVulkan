use std::path::PathBuf;

fn main() {
    if let Ok(path) = std::env::var("GLFW_PATH") {
        println!("cargo::rustc-link-search={path}/lib/");
        println!("cargo::rustc-link-lib=glfw");
    } else {
        println!("cargo::error=Failed to find GLFW_PATH");
        return;
    }

    if let Ok(path) = std::env::var("VULKAN_SDK") {
        println!("cargo::rustc-link-search={path}/include/vulkan/");
        println!("cargo::rustc-link-lib=vulkan");
    } else {
        println!("cargo::error=Failed to find VULKAN_SDK");
        return;
    }

    // TODO find a way to properly not-include too much stuff recursively?
    //      ideally would only grab the glfw stuff here, but this includes all of Vulkan and OpenGL too...
    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
