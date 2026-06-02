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

  cargoHash = "sha256-ajdcftvHKUptOARRK/kMMNZH3o+xWia7oxO10JR50+0=";

  meta = with lib; {
    description = "A program";
    license = licenses.agpl3Only;
    platforms = platforms.all;
  };
}
