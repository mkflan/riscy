{
  description = "shell environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, rust-overlay, nixpkgs }: {
    devShells.x86_64-linux.default = let
      pkgs = nixpkgs.legacyPackages.x86_64-linux.pkgsCross.riscv64-embedded;
      rust-bin = rust-overlay.lib.mkRustBin { } pkgs.buildPackages;
      rust = rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
    in    
      pkgs.mkShell {
        nativeBuildInputs = [ rust ];
        depsBuildBuild = [ pkgs.pkgsBuildBuild.qemu ];

        env = {
          CARGO_TARGET_RISCV64IMAC_UNKNOWN_NONE_ELF_RUNNER = "qemu-system-riscv64";
        };
      };
  };
}
