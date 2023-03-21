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
    nixpkgs.url = "github:nixos/nixpkgs";
  };

  outputs = {
    self,
    flake-utils,
    naersk,
    nixpkgs,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      naersk' = pkgs.callPackage naersk {};
    in {
      formatter = pkgs.alejandra;
      defaultPackage = naersk'.buildPackage {
        name = "scum";
        src = ./.;
      };
      devShell = pkgs.mkShell {
        nativeBuildInputs = [pkgs.rustc pkgs.cargo pkgs.libiconv];
      };
    });
}
