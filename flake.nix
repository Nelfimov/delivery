{
  description = "Microarch delivery";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs =
    { flake-utils, nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };
        rustfmt = pkgs.rustfmt.override { asNightly = true; };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.cmake
            pkgs.rustc
            rustfmt
            pkgs.clippy
            pkgs.go
            pkgs.postgresql
            pkgs.diesel-cli
            pkgs.openapi-generator-cli
          ];

          shellHook = ''
            if test -f ".env"; then
              set -a
              source .env
              set +a
            fi
          '';
        };
      }
    );
}
