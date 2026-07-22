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

  cargoHash = "sha256-aysldp24E+NcC0XWwWp7UwL0swvc4wQ/mvUHPAlKf6s=";

  meta = with lib; {
    description = "A program";
    license = licenses.agpl3Only;
    platforms = platforms.all;
  };
}
