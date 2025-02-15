{ pkgs ? import <nixpkgs> {} }:

let
  	libPath = with pkgs; lib.makeLibraryPath [
		libGL
		libxkbcommon
  		wayland
  	];
in {
	devShell = with pkgs; mkShell {
		buildInputs = [
			cargo
			rustc
			rust-analyzer

			# for LUA cli
			lua

			pkg-config
		];

		RUST_LOG = "info";
		RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
		LD_LIBRARY_PATH = libPath;
	};
}
