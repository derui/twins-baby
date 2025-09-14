{
  description = "Flake for Aozora Typer";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    # Rust tool management
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

  };

  outputs =
    {
      self,
      nixpkgs,
      fenix,
      ...
    }:
    let
      # System types to support.
      supportedSystems = [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      # Helper function to generate an attrset '{ x86_64-linux = f "x86_64-linux"; ... }'.
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;

      # Nixpkgs instantiated for supported system types.
      nixpkgsFor = forAllSystems (
        system:
        import nixpkgs {
          inherit system;

          overlays = [ fenix.overlays.default ];
        }
      );
    in
    {

      # define formatter for all systems
      formatter = forAllSystems (system: nixpkgsFor.${system}.nixfmt-rfc-style);

      # define devShell for aysstem with packages
      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
          fenix-pkg = fenix.packages.${system};
        in
        {
          default = let
            _buildInputs = [
              (fenix-pkg.combine [
                fenix-pkg.stable.cargo
                fenix-pkg.stable.clippy
                fenix-pkg.stable.rust-src
                fenix-pkg.stable.rustc
                fenix-pkg.stable.rustfmt
                fenix-pkg.stable.llvm-tools-preview
                fenix-pkg.targets.wasm32-unknown-unknown.stable.rust-std
              ])
              pkgs.stdenv

              pkgs.rust-analyzer-nightly
              pkgs.openssl
              pkgs.pre-commit
              pkgs.python312
              pkgs.python312Packages.pip

              # for bevy
              pkgs.xdotool
              pkgs.glib.dev
              pkgs.gdk-pixbuf.dev
              pkgs.pango.dev
              pkgs.atk.dev
              pkgs.gtk3.dev
              pkgs.libsoup_3.dev
              pkgs.webkitgtk_4_1.dev
              pkgs.alsa-lib.dev
              pkgs.udev.dev 
              pkgs.xorg.libX11
              pkgs.xorg.libXrandr
              pkgs.xorg.libXcursor
              pkgs.xorg.libxcb
              pkgs.xorg.libXi
              pkgs.wayland
              pkgs.libxkbcommon
              pkgs.libxkbcommon.dev
              pkgs.vulkan-loader
              pkgs.vulkan-tools
              pkgs.glfw
              pkgs.xorg.xf86videoamdgpu  # notice this line might not match your needs or desires
              
              # for tailwindcss
              pkgs.nodejs_22
            ];
          in
            pkgs.mkShell {
            nativeBuildInputs = with pkgs; [ pkg-config ];
            buildInputs = _buildInputs;

            RUST_SRC_PATH = "${fenix-pkg.stable.rust-src}/lib/rustlib/src/rust/library";
            RUST_BACKTRACE = 1;
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath _buildInputs;

            shellHook = ''
              pre-commit install
              cargo install cargo-nextest
              cargo install cargo-watch
              cargo install cargo-llvm-cov
              cargo install leptosfmt
            '';
          };
        }
      );
    };
}
