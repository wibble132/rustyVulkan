let
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
  rust-version = "2024-11-22";
  rust = pkgs.rust-bin.nightly.${rust-version}.default.override {
    extensions = [
      "rust-src"
      "rustc-dev"
      "llvm-tools"
    ];
  };
in
pkgs.mkShell {
  buildInputs = [
    rust
  ] ++ (with pkgs; [
    llvmPackages_13.libclang
    vulkan-loader
    vulkan-validation-layers

    # Needed for glfw package
    cmake
    extra-cmake-modules
    libxkbcommon
    wayland
    wayland-protocols
    wayland-scanner
  ]);

  RUST_BACKTRACE = 1;

  LIBCLANG_PATH = "${pkgs.llvmPackages_13.libclang.lib}/lib";
  LD_LIBRARY_PATH="${with pkgs; lib.makeLibraryPath [
    wayland
    libxkbcommon
    vulkan-loader]
  }";
}

