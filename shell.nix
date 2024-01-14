let
  nixpkgs-src = builtins.fetchTarball {
    # 23.05
    url = "https://github.com/NixOS/nixpkgs/archive/nixos-23.05.tar.gz";
  };

  pkgs = import nixpkgs-src {
    config = {
      # allowUnfree may be necessary for some packages, but in general you should not need it.
      allowUnfree = false;
    };
  };

  lib-defs = with pkgs; [
    rustup
    libxkbcommon
    libGL

  # WINIT_UNIX_BACKEND=wayland
    wayland

  # Default to wayland
  # WINIT_UNIX_BACKEND=x11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    xorg.libX11
  ];

  shell = pkgs.mkShell {
    buildInputs = lib-defs;
    shellHook = ''
      export WINIT_UNIX_BACKEND=x11
      rustup update
      # Augment the dynamic linker path
      export "LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath lib-defs}"
    '';
  };

in shell
