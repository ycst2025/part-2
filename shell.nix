{ pkgs ? import <nixpkgs> {} }:

{
  devShell =
    with pkgs;
    mkShell {
      buildInputs = [
        gcc-arm-embedded
        qemu
        (ruby.withPackages (ps: [ ps.racc ]))
      ];
    };
}
