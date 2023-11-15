{
  description = "It's about time I made a Lisp of some sort…";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {
    flake-parts,
    rust-overlay,
    nixpkgs,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = nixpkgs.lib.systems.flakeExposed;
      perSystem = {
        self',
        system,
        ...
      }: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        nativeBuildInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin [pkgs.libiconv];
        naersk = pkgs.callPackage inputs.naersk {};
        buildCrate = pkg:
          naersk.buildPackage {
            inherit nativeBuildInputs pkg;
            src = ./.;
          };
      in {
        formatter = pkgs.alejandra;
        packages = rec {
          scum-lib = buildCrate "scum-lib";
          scum-repl = buildCrate "scum-repl";
          default = scum-repl;
        };
        apps = rec {
          scum-repl = {
            type = "app";
            program = "${self'.packages.scum-repl}/bin/scum-repl";
          };
          default = scum-repl;
        };
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs;
          buildInputs = [
            toolchain
          ];
        };
      };
    };
}
