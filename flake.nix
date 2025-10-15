{
	description = "A Computer Algebra System";

	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
		hooks = {
			url = "github:cachix/git-hooks.nix";
			inputs.nixpkgs.follows = "nixpkgs";
		};
		fenix = {
			url = "github:nix-community/fenix";
			inputs.nixpkgs.follows = "nixpkgs";
		};
	};

	outputs = {
		self,
		hooks,
		fenix,
		nixpkgs,
		...
	}: let
		systems = ["aarch64-linux" "x86_64-linux"];
		forAllSystems = f:
			nixpkgs.lib.genAttrs systems (system:
					f {
						pkgs =
							import nixpkgs {
								inherit system;
								overlays = [self.overlays.default];
							};
					});
	in {
		overlays.default = final: prev: {
			rustToolchain = with fenix.packages.${prev.stdenv.hostPlatform.system};
				combine (
					(with stable; [clippy rustc cargo rust-src])
					++ [default.rustfmt]
				);
		};

		checks =
			forAllSystems ({pkgs}: {
					pre-commit-check =
						hooks.lib.${pkgs.system}.run {
							src = ./.;
							hooks = {
								clippy = {
									enable = true;
									package = fenix.packages.${pkgs.system}.stable.clippy;
								};
								rustfmt = {
									enable = true;
									package = fenix.packages.${pkgs.system}.default.rustfmt;
								};
							};
						};
				});

		packages =
			forAllSystems ({pkgs}: {
					default =
						(pkgs.makeRustPlatform {
								cargo = pkgs.rustToolchain;
								rustc = pkgs.rustToolchain;
							}).buildRustPackage {
							pname = "abacas";
							version = "0.1.0";
							src = ./.;
							cargoLock.lockFile = ./Cargo.lock;
						};
				});

		devShells =
			forAllSystems ({pkgs}: let
					check = self.checks.${pkgs.system}.pre-commit-check;
				in {
					default =
						pkgs.mkShell {
							inherit (check) shellHook;
							buildInputs = check.enabledPackages;

							packages = with pkgs; [
								rustToolchain
								pkg-config
								cargo-deny
								cargo-edit
								cargo-semver-checks
								cargo-watch
								cargo-show-asm
								rust-analyzer
								bacon
							];

							env.RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
						};
				});
	};
}
