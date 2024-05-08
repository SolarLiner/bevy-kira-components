{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    cargo-smart-release-src = {
      url = "github:Byron/cargo-smart-release/v0.21.3";
      flake = false;
    };
  };

  outputs = { self, flake-utils, naersk, nixpkgs, cargo-smart-release-src }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };
        inherit (pkgs) lib;

        naersk' = pkgs.callPackage naersk {};

        nativeBuildInputs = with pkgs; [ cmake ninja pkg-config ];
        buildInputs = with pkgs; [
          udev alsa-lib vulkan-loader
          xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
          libxkbcommon wayland # To use the wayland feature
        ];

        cargo-smart-release = naersk'.buildPackage {
          inherit nativeBuildInputs;
          buildInputs = with pkgs; [openssl];
          src = cargo-smart-release-src;
          singleStep = true;
        };

        bevy-kira-components = naersk'.buildPackage {
          inherit nativeBuildInputs;
          buildInputs = with pkgs; [pkg-config alsa-lib];
          src = ./.;
        };
      in {

        packages = {
          inherit cargo-smart-release bevy-kira-components;
          default = bevy-kira-components;
        };

        # For `nix develop`:
        devShell = pkgs.mkShell {
          nativeBuildInputs = nativeBuildInputs ++ buildInputs ++ [cargo-smart-release];
          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        };
      }
    );
}
