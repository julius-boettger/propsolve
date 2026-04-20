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

        buildInputs = [ pkgs.z3 ];
      };
    });

    devShells = eachSystem (system: pkgs: {
      default = pkgs.mkShell {
        inherit (self.packages.${system}.default) buildInputs;

        nativeBuildInputs = with pkgs; [
          rustc
          cargo
          clippy
          cargo-edit # provides `cargo upgrade` for dependencies

          # convenient command to create a linux release with statically linked z3
          cmake # for compiling z3 when statically linking
          (writeShellScriptBin "release-linux" ''
            rm -rf propsolve-linux-x86_64
            cargo build --target x86_64-unknown-linux-gnu --release --features "z3/vendored" "$@" || exit 1
            cp target/x86_64-unknown-linux-gnu/release/propsolve propsolve-linux-x86_64 || exit 1
            # should work for most linux distros
            patchelf --set-interpreter /lib64/ld-linux-x86-64.so.2 propsolve-linux-x86_64 || exit 1
          '')
        ];

        # fix rust-analyzer in vscode
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    });
  };
}