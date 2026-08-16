{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    clippy
    
    # additional useful tools
    pkg-config
    openssl
  ];

  shellHook = ''
    printf "\033[35mRust development environment loaded!\n\033[0m"
    rustc --version
    cargo --version
  '';
}
