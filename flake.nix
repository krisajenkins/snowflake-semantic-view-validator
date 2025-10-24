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
          packages = rec {
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

            default = ssvv;
          };

          apps.default = {
            type = "app";
            program = "${self.packages.${system}.ssvv}/bin/ssvv";
          };

          checks = {
            build = self.packages.${system}.ssvv;
            help-test = pkgs.runCommand "ssvv-help-test" { } ''
              ${self.packages.${system}.ssvv}/bin/ssvv --help
              touch $out
            '';
          };

          formatter = pkgs.nixpkgs-fmt;

          devShells.default =
            with pkgs;
            mkShell {
              nativeBuildInputs =
                [
                  # Rust
                  cargo
                  rustfmt
                  rust-analyzer
                  clippy

                  cargo-generate
                  cargo-watch
                ];
            };
        });
}

