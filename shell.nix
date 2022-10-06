{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  nativeBuildInputs = [
    pkg-config
    fontconfig
    gcc
    libGL
    libxkbcommon
    wayland
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
  ];
  propagatedBuildInputs = [ vulkan-loader ];
  buildInputs = [ ];
  shellHook = "";
  LD_LIBRARY_PATH = lib.makeLibraryPath [ vulkan-loader ];
}
