# save this as shell.nix
{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  packages = [
    pkgs.openssl
    pkgs.pkg-config
    pkgs.rustc
    pkgs.cargo
  ];
}
