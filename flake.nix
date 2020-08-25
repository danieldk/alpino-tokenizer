{
  description = "Alpino-based tokenizer for Dutch";

  inputs = {
    naersk = {
      url = "github:nmattia/naersk/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs-channels/nixos-unstable-small";
    utils.url = "github:numtide/flake-utils";
  };
  
  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system: {
      defaultPackage = self.packages.${system}.alpino-tokenize;

      defaultApp = self.apps.${system}.alpino-tokenize;

      apps.alpino-tokenize = {
        type = "app";
        program = "${self.defaultPackage.${system}}/bin/alpino-tokenize";
      };

      packages.alpino-tokenize = naersk.lib.${system}.buildPackage {
        name = "alpino-tokenize";
        root = ./.;
        targets = [ "alpino-tokenize" ];
        doCheck = true;
      };
    });
}
