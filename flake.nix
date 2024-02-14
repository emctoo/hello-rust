{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix }:
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    rustToolchain = fenix.packages.${system}.minimal.toolchain;
    rustPlatform = pkgs.makeRustPlatform {
      cargo = rustToolchain;
      rustc = rustToolchain;
    };
  in {
    foo = "bar";

    packages."${system}" = {
      default = rustToolchain.minimal;

      t = rustPlatform.buildRustPackage {
        pname = "t";
        version = "0.1.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
      };

      # t =
      #   with import nixpkgs { system = system; };
      #   stdenv.mkDerivation {
      #     name = "hello";
      #     src = self;
      #     buildPhase = "cargo build --release";
      #     installPhase = "mkdir -p $out/bin; install -t $out/bin ./target/release/t";
      #   };
    };

    devShell."${system}" = pkgs.mkShell {
      nativeBuildInputs = with pkgs; [
        clang
        # Use mold when we are runnning in Linux
        # (lib.optionals stdenv.isLinux mold)
      ];
      buildInputs = with pkgs; [
        # rustToolchain.${pkgs.system}.default
        rust-analyzer-unwrapped
        pkgs.cargo
        pkgs.rustc
        pkg-config
        openssl

        cowsay
        ripgrep
        fzf
        # tmux
        watchexec

        # self.packages.${system}.t
      ];
      # RUST_SRC_PATH = "${rustToolchain.${pkgs.system}.rust-src}/lib/rustlib/src/rust/library";

      shellHook = ''
        export http_proxy=http://192.168.8.34:1081
        echo 'Hello, world!'
        # tmux new -s cli
      '';
    };
  };
}
