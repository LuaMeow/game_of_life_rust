{
  inputs = {
    # nixpkgs, i guess that's kinda obvious
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    # flake-utils is quite nice to have to simplify defining devshells and stuff,
    # because flakes always expect devshell outputs to be of the form devShells.\<system>.default
    # (e.g. devShells.x86_64-linux.default), and flake-utils allows abstracing over the system by
    # simply writing devShells.default, and it automatically creates devshell outputs for all common systems.
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs =
    {
      nixpkgs,
      flake-utils,
      ...
    }:
    # this simply takes a function that receives a system and outputs an attribute set,
    # and returns an attribute set with that system squished between the outputs
    # (e.g. converts devShells.default to devShells.\<whatever system>.default)
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.cargo
            pkgs.rustc
            pkgs.cargo-generate
            pkgs.trunk
            pkgs.rust-analyzer
            pkgs.rustfmt
          ];
        };
      }
    );
}
