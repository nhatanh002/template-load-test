with import <nixpkgs> { };

pkgs.mkShell { 
  buildInputs = [ darwin.apple_sdk.frameworks.Security pkgconfig openssl iconv ];
  shellHook = ''
    alias nv=nvim
  '';
}