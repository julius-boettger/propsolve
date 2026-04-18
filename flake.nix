{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    systems.url = "github:nix-systems/default"; # can run on all systems
  };

  outputs = { self, nixpkgs, systems, ... }:
  let
    eachSystem = fn: nixpkgs.lib.genAttrs (import systems) (system: fn system (import nixpkgs {
      inherit system;
    }));
  in
  {
    packages = eachSystem (system: pkgs: rec {
      default = propsolve;
      propsolve = pkgs.rustPlatform.buildRustPackage {
        name = "propsolve";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
      };
    });

    devShells = eachSystem (system: pkgs: {
      default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          rustc
          cargo
          clippy
          cargo-edit # provides `cargo upgrade` for dependencies
        ];
        # fix rust-analyzer in vscode
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    });
  };
}