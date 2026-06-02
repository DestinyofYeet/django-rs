{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    { self, nixpkgs }@inputs:
    let
      pkgs = import nixpkgs { system = "x86_64-linux"; };

      deps = import ./nix/dependencies.nix { inherit pkgs; };
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        nativeBuildInputs = deps.packages;
        # uncomment this is you get some kind of ssl error, usually on anything networking related using reqwest
        # PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
      };

      packages.x86_64-linux.default = pkgs.callPackage ./nix/pkg.nix { };
    };
}
