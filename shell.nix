with import <nixpkgs> { };

pkgs.mkShell { 
  buildInputs = [ pkgconfig openssl iconv ] 
    ++ lib.optionals stdenv.isDarwin [ darwin.apple_sdk.frameworks.Security ];
  shellHook = ''
    alias nv=nvim
  '';
}
