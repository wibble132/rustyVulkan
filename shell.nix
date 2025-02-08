{
  pkgs ? (import <nixpkgs> {}),
  stdenv ? pkgs.clang13Stdenv
}:
stdenv.mkDerivation {
  name = "vkTriangleRust";

  buildInputs = with pkgs; [
    # put packages here.
    glslang
    shaderc # glslc
    vulkan-tools
    vulkan-headers
    spirv-tools
    vulkan-loader
    vulkan-validation-layers
    glfw

    xorg.libX11
    xorg.libXi
    xorg.libXxf86vm
    xorg.libXrandr
    xorg.libXinerama
    xorg.libXcursor

    extra-cmake-modules
    wayland-protocols
    wayland
    wayland-scanner
    libxkbcommon

    cmake
    tinyobjloader
    stb

    jetbrains.rust-rover
  ];

  GLFW_PATH="${pkgs.glfw}";
  STB_PATH="${pkgs.stb}";
  VULKAN_SDK = "${pkgs.vulkan-headers}";
  LIBCLANG_PATH = "${pkgs.llvmPackages_13.libclang.lib}/lib";
}

