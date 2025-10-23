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

