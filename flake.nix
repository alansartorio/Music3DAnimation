{
  description = "Animusic director";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        runtimeDependencies = [ ];

        buildDependencies = with pkgs; [ ];
        rpath = with pkgs; lib.makeLibraryPath runtimeDependencies;

      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs =
            with pkgs;
            [
              cargo
              #rustc
              pkg-config
            ]
            ++ buildDependencies;
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildDependencies;
        };
      }
    );
}
