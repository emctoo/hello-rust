# flake.nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    nixvim = {
      url = "github:nix-community/nixvim";
      # If you are not running an unstable channel of nixpkgs, select the corresponding branch of nixvim.
      # url = "github:nix-community/nixvim/nixos-23.05";

      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, nixvim }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [rust-overlay.overlays.default];
    };
    toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
    nvim = nixvim.legacyPackages.x86_64-linux.makeNixvim {
      plugins.lsp.enable = true;
    };
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = [
        toolchain
        nvim
        pkgs.lsd
        pkgs.nushell
        pkgs.fish
        pkgs.tmux
        # pkgs.neovim
      ];

      shellHook = ''
        init() {
          echo "initialize ..."
          podman run --name pg -d --rm -e POSTGRES_PASSWORD=postgres docker.io/library/postgres:15
          # exec nu
        }

        cleanup() {
          podman stop pg
          echo "cleaned up ..."
        }

        init
        trap cleanup EXIT
      '';
    };
  };
}
