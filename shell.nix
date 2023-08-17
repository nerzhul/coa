# Cargo 1.69.0, RustC 1.69.0 & Rustup 1.26.0
{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/1c62806f8992fd548b4e2331051c8eed85bc769d.tar.gz") {}}:

pkgs.mkShell {
  packages = [
    pkgs.openssl
    pkgs.pkg-config
	# rustc 1.69, cargo 1.69 & rustup 1.26
    pkgs.rustc
    pkgs.cargo
    pkgs.rustup
  ];
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
