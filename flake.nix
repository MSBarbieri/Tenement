{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
  };
  outputs =
    { self, nixpkgs, utils, fenix, pre-commit-hooks, ... }:
    utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          fenix.overlays.default
        ];
      };
    in
    {
      checks = {
        pre-commit-check = pre-commit-hooks.lib.${system}.run {
          src = ./.;
          hooks = {
            nixpkgs-fmt.enable = true;
            rustfmt.enable = true;
          };
        };
      };

      devShells.default = pkgs.mkShell {
        inherit (self.checks.${system}.pre-commit-check) shellHook;

        buildInputs = with pkgs; [
          (pkgs.fenix.complete.withComponents [
            "cargo"
            "clippy"
            "rust-src"
            "rustc"
            "rustfmt"
          ])
          cargo-watch
          pre-commit
          nixpkgs-fmt
          openssl
          protobuf
          tilt
          kind
        ];
        nativeBuildInputs = [ pkgs.pkg-config ];
      };
    });
}
