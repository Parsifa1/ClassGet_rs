{
  description = "A basic flake with a shell";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.cloudtide = {
    url = "github:parsifa1/nixpkg";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    nixpkgs,
    cloudtide,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.default = pkgs.mkShell {
        packages = with pkgs; [libiconv pkg-config];
        shellHook = ''
          export OPENSSL_DEV=${pkgs.openssl.dev}
          export PKG_CONFIG_PATH=${pkgs.openssl.dev}/lib/pkgconfig
          export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.stdenv.cc.cc.lib}/lib
          export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${cloudtide.packages.${system}.onnxruntime}/lib
        '';
      };
    });
}
