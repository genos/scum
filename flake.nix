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
    nixpkgs.url = "github:nixos/nixpkgs";
  };

  outputs = {
    self,
    flake-utils,
    naersk,
    rust-overlay,
    nixpkgs,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [rust-overlay.overlays.default];
      };
      naersk' = pkgs.callPackage naersk {};
      buildInputs =
        []
        ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
          pkgs.libiconv
        ];
      buildPackage = pname:
        naersk'.buildPackage {
          inherit buildInputs pname;
          src = ./.;
        };
      scum-lib = buildPackage "scum-lib";
      scum-repl = buildPackage "scum-repl";
    in {
      formatter = pkgs.alejandra;
      packages = {
        inherit scum-lib scum-repl;
        default = scum-repl;
      };
      devShells.default = pkgs.mkShell {
        nativeBuildInputs = [pkgs.rustc pkgs.cargo];
        inherit buildInputs;
      };
    });
}
