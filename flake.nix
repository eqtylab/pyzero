{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ { self, ... }:
    (inputs.flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let

        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [ inputs.rust-overlay.overlays.default ];
        };

        inherit (pkgs) callPackage;

        rust-config = {
          extensions = [ "rust-src" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        wasm-bindgen-cli = callPackage ./nix/wasm-bindgen-cli.nix { };

        rust = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain).override rust-config;

        # rustfmt from rust-nightly used for advanced options in rustfmt
        rustfmt-nightly = pkgs.rust-bin.nightly.latest.rustfmt;

        shellPkgs = [
          rustfmt-nightly
          rust
          wasm-bindgen-cli
        ] ++ (with pkgs; [
          bc
          clang
          ets
          just
          nixpkgs-fmt
          openssl
          pkg-config
          present-cli
          rustup
          wasm-pack
        ]);

      in
      rec {

        devShells = {
          default = pkgs.mkShell {
            nativeBuildInputs = shellPkgs;
            CC = "${pkgs.clang}/bin/clang";
            CXX = "${pkgs.clang}/bin/clang++";
          };
        };

      }));
}
