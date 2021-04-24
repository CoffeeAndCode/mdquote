{
  inputs = {
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
    in rec {
      # `nix build`
      packages.mdquote = naersk-lib.buildPackage {
        pname = "mdquote";
        root = ./.;
      };
      defaultPackage = packages.mdquote;

      # `nix run`
      apps.mdquote = utils.lib.mkApp {
        drv = packages.mdquote;
      };
      defaultApp = apps.mdquote;

      # `nix develop`
      devShell = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [ cargo rustc ];
      };
    });
}
