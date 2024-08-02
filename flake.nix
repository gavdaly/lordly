{
  description = "A component library for building web applications in Leptos";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }: {
    overlay = final: prev: {
      rust = rust-overlay.overlays.default(final, prev);
    };

    packages = {
      default = nixpkgs.lib.mkShell {
        buildInputs = [
          nixpkgs.pkgs.rust-bin.stable.latest.rustc
          nixpkgs.pkgs.rust-bin.stable.latest.cargo
          nixpkgs.nodejs
        ];

        shellHook = ''
          export RUSTUP_HOME=$(pwd)/.rustup
          export CARGO_HOME=$(pwd)/.cargo
          export PATH=$CARGO_HOME/bin:$PATH
          echo "Development environment for Leptos project is ready."
        '';
      };
    };

    defaultPackage = self.packages.default;

    devShell = self.packages.default;
  };
}
