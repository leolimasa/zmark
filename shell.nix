{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup
  ];

  # Runs a command after shell is started
  shellHook = ''
  '';
}
