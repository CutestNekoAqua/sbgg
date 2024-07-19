{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
    rust-flake.url = "github:juspay/rust-flake";
    rust-flake.inputs.nixpkgs.follows = "nixpkgs";
    process-compose-flake.url = "github:Platonic-Systems/process-compose-flake";
    cargo-doc-live.url = "github:srid/cargo-doc-live";

    # Dev tools
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = [
        inputs.treefmt-nix.flakeModule
        inputs.rust-flake.flakeModules.default
        inputs.rust-flake.flakeModules.nixpkgs
        inputs.process-compose-flake.flakeModule
        inputs.cargo-doc-live.flakeModule
      ];
      perSystem = { config, self', pkgs, lib, ... }: {
        rust-project.crates."rust-nix-template".crane.args = {
          buildInputs = [
            pkgs.openssl
            pkgs.graphite2
            pkgs.icu
            pkgs.freetype
            pkgs.libpng
            pkgs.harfbuzzFull
            pkgs.fontconfig
            pkgs.lato
            pkgs.texlivePackages.lato
            pkgs.curl
          ] ++ lib.optionals pkgs.stdenv.isDarwin (
            with pkgs.darwin.apple_sdk.frameworks; [
              IOKit
              ApplicationServices
              Cocoa
            ]
          );
        };

        # Add your auto-formatters here.
        # cf. https://nixos.asia/en/treefmt
        treefmt.config = {
          projectRootFile = "flake.nix";
          programs = {
            nixpkgs-fmt.enable = true;
            rustfmt.enable = true;
          };
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ self'.devShells.rust ];
          packages = [
            pkgs.cargo-watch
            pkgs.openssl
            pkgs.graphite2
            pkgs.icu
            pkgs.freetype
            pkgs.libpng
            pkgs.harfbuzzFull
            pkgs.fontconfig
            pkgs.lato
            pkgs.texlivePackages.lato
            config.process-compose.cargo-doc-live.outputs.package
          ];
        };
        packages.default = self'.packages.rust-nix-template;
      };
    };
}
