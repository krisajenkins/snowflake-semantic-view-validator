{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/25.05";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            config.allowUnfree = true;
            overlays = [ ];
          };
        in
        {
          packages = {
            ssvv = pkgs.rustPlatform.buildRustPackage {
              pname = "ssvv";
              version = "0.1.0";
              src = ./.;

              cargoLock = {
                lockFile = ./Cargo.lock;
              };

              meta = with pkgs.lib; {
                description = "Snowflake Semantic View Validator";
                homepage = "https://github.com/krisajenkins/snowflake-semantic-view-validator";
                license = licenses.mit;
                maintainers = [ ];
              };
            };

            default = self.packages.${system}.ssvv;
          };

          devShells.default =
            with pkgs;
            mkShell {
              buildInputs =
                [
                  # Rust
                  cargo
                  rustc
                  rustfmt
                  rust-analyzer
                  clippy

                  cargo-generate
                  cargo-edit
                  cargo-watch

                  iconv
                ];
            };
        });
}

