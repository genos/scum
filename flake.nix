{
  description = "It's about time I made a Lisp of some sort…";

  inputs = {
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11";
  };

  outputs = {
    self,
    flake-utils,
    naersk,
    rust-overlay,
    nixpkgs,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
      rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      nativeBuildInputs = [rustToolchain] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [pkgs.libiconv];
      buildPackage = pname:
        (pkgs.callPackage naersk {}).buildPackage {
          inherit nativeBuildInputs pname;
          src = ./.;
        };
    in {
      formatter = pkgs.alejandra;
      packages = rec {
        scum-lib = buildPackage "scum-lib";
        scum-repl = buildPackage "scum-repl";
        default = scum-repl;
      };
      devShells.default = pkgs.mkShell {
        inherit nativeBuildInputs;
      };
    });
}
