{
  description = "class_get's devshell";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    cloudtide = {
      url = "github:parsifa1/nixpkg";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    # cloudtide,
    fenix,
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      rustoolchains = fenix.packages.${system};
      # ct = cloudtide.packages.${system};
    in {
      devShells.default = pkgs.mkShell {
        packages =  [
          (rustoolchains.stable.withComponents ["cargo" "clippy" "rust-docs" "rust-std" "rustc" "rust-src"])
          (rustoolchains.complete.withComponents ["rustfmt"])
        ];
        # export OPENSSL_DEV=${pkgs.openssl.dev}
        # export PKG_CONFIG_PATH=${pkgs.openssl.dev}/lib/pkgconfig
        # export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.openssl.out}/lib
        # export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.stdenv.cc.cc.lib}/lib
        # shellHook = ''
        #   export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${ct.onnxruntime}/lib
        # '';
      };
    });
}
