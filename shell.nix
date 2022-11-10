let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> {
    overlays = [ moz_overlay ];
  };
  ruststable = (nixpkgs.latest.rustChannels.nightly.rust.override {
    extensions = [ "rust-src" "rust-analysis" ];}
  );
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "rust";
    buildInputs = [
      rustup ruststable


      # for sn_node_gui experiments

      xorg.libX11
      xorg.libXcursor
      xorg.libXrandr
      xorg.libXi
      pkg-config
    ];

    # For sn_node_gui experiments

    APPEND_LIBRARY_PATH = "${lib.makeLibraryPath [ libGL ]}";
    shellHook = ''
      export LD_LIBRARY_PATH="$APPEND_LIBRARY_PATH:$LD_LIBRARY_PATH"
    '';
  }
