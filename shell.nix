let
    nixpkgs = import ./nix/nixpkgs.nix;
    shell = nixpkgs.mkShell {
        nativeBuildInputs = [
            nixpkgs.cargo
        ];
    };
in
    shell
