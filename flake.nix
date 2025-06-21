{
  description = "Animusic Blender Extension";

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
        name = "animusic";
        version = "0.0.1";
        fake-bpy-module = rec {
          pname = "fake_bpy_module";
          version = "20250616";
          format = "wheel";
          dist = "py3";
          python = "py3";
          abi = "none";
          platform = "any";
          #wheel-filename = "${pname}-${version}-${python}-${abi}-${platform}.whl";
          #wheel = pkgs.fetchPypi {
            #inherit
              #pname
              #version
              #format
              #dist
              #python
              #abi
              #platform
              #;
            #sha256 = "sha256-PpnX8iU5d7Zc8KOD1riXPQTUlsDtqWYa15oeEM9aRss=";
          #};
          src = pkgs.fetchPypi {
            inherit
              pname
              version
              ;
            sha256 = "sha256-UiHSijELzHcu0R3hIUudwIbejbXn05joc99daSi4xKg=";
          };
        };
        blender-manifest = (pkgs.formats.toml { }).generate "blender-manifest" {

          schema_version = "1.0.0";

          id = name;
          inherit version;
          name = "Animusic Player";
          tagline = "Animusic Player";
          maintainer = "Alan Sartorio <alan42ga@hotmail.com>";
          type = "add-on";

          blender_version_min = "4.2.0";

          license = [
            "SPDX:GPL-3.0-or-later"
          ];

          #wheels = [
          #"./wheels/${fake-bpy-module.wheel-filename}"
          #];
        };
      in
      {
        packages.default = pkgs.stdenv.mkDerivation {
          name = "${name}-blender";
          pname = name;
          inherit version;
          src = ./.;

          buildInputs = [ pkgs.zip ];
          buildPhase = ''
            mkdir wheels
            #cp -r $${fake-bpy-module.wheel} wheels/$${fake-bpy-module.wheel-filename}

            cp ${blender-manifest} blender_manifest.toml

            #zip animusic-blender.zip blender_manifest.toml wheels/* __init__.py
            zip animusic-blender.zip blender_manifest.toml __init__.py
          '';
          installPhase = ''cp animusic-blender.zip $out'';
        };
        devShells.default = pkgs.mkShellNoCC {
          buildInputs = with pkgs; [
            (python3.withPackages (python-pkgs: [
              (python3Packages.buildPythonPackage {
                inherit (fake-bpy-module) pname version src;
                doCheck = false;
              })
            ]))
          ];
        };
      }
    );
}
