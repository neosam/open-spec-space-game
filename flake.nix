{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    openspec.url = "github:Fission-AI/OpenSpec";
  };

  outputs = { nixpkgs, rust-overlay, openspec, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
        config.allowUnfree = true;
      };
      rustToolchain = pkgs.rust-bin.stable.latest.default;
      bevyDeps = with pkgs; [
        vulkan-loader
        libxkbcommon
        wayland
        libdecor
        libx11
        libxcursor
        libxi
        libxrandr
        alsa-lib
        udev
      ];
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [
          rustToolchain
          pkgs.pkg-config
          openspec.packages.${system}.default
          pkgs.claude-code
        ] ++ bevyDeps;

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath bevyDeps;
      };
    };
}
