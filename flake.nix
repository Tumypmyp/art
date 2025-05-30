{
  description = "Dioxus art website";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay }: 
    let
      overlays = [ rust-overlay.overlay ];
      pkgs = import nixpkgs { inherit overlays; };
    in {
      devShell = pkgs.mkShell {
        buildInputs = [
          pkgs.rust-toolchain.stable.latest.default
          pkgs.cargo-edit
          pkgs.clippy
          pkgs.rust-analyzer
        ];
      };
    };
}
