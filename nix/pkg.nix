{
  rustPlatform,
  lib,
  pkgs,
  ...
}:
let
  deps = import ./dependencies.nix { inherit pkgs; };
in

rustPlatform.buildRustPackage {
  pname = "django-rs";
  version = "1.0";

  buildInputs = deps.packages;

  src = ../.;

  cargoHash = "sha256-kcbqcm35lsPuckE8tD3zDJJAU8hhJvMe3YZaTPMcGzE=";

  meta = with lib; {
    description = "A program";
    license = licenses.agpl3Only;
    platforms = platforms.all;
  };
}
