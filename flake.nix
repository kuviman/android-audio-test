{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    geng.url = "github:geng-engine/cargo-geng";
    geng.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs = { self, geng, nixpkgs, ... }@inputs:
    let
      recursiveMerge = attrList:
        let
          f = attrPath:
            pkgs.lib.zipAttrsWith (n: values:
              if builtins.tail values == [ ]
              then builtins.head values
              else if builtins.all builtins.isList values
              then builtins.unique (builtins.concatLists values)
              else if builtins.all builtins.isAttrs values
              then f (attrPath ++ [ n ]) values
              else builtins.last values
            );
        in
        f [ ] attrList;
      system = "x86_64-linux";
      overlays = [ (import inputs.rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };

      flake = geng.makeFlakeOutputs (system:
        {
          src = geng.lib.${system}.filter {
            root = ./.;
            include = [
              "src"
              "crates"
              "logicsider"
              "dependencies"
              "internal-crates"
              "android"
              "levels"
              "assets"
              "Cargo.lock"
              "Cargo.toml"
            ];
          };
          extraBuildInputs = with pkgs;
            [
              mold
              clang_14
              valgrind
              renderdoc
              hotspot
              linuxKernel.packages.linux_6_6.perf
            ];
        });
    in

    flake //
    (
      let
        nightly = pkgs.rust-bin.nightly.latest.default;
        cargo-nightly = pkgs.writeShellScriptBin "cargo-nightly" ''
          export RUSTC="${nightly}/bin/rustc";
          export CARGO="${nightly}/bin/cargo";
          exec $CARGO "$@"
        '';
      in
      {
        devShell.${system} = flake.devShell.${system}.overrideAttrs (finalAttrs: prevAttrs: {
          buildInputs = prevAttrs.buildInputs ++ [ cargo-nightly pkgs.cargo-flamegraph ];
        });
      }
    );
}
