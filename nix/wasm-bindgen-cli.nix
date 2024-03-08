{ lib
, rustPlatform
, fetchCrate
, nodejs
, pkg-config
, openssl
, stdenv
, curl
, darwin
}:

rustPlatform.buildRustPackage rec {
  pname = "wasm-bindgen-cli";
  version = "0.2.91";

  src = fetchCrate {
    inherit pname version;
    hash = "sha256-f/RK6s12ItqKJWJlA2WtOXtwX4Y0qa8bq/JHlLTAS3c=";
  };

  cargoHash = "sha256-3vxVI0BhNz/9m59b+P2YEIrwGwlp7K3pyPKt4VqQuHE=";

  nativeBuildInputs = [ pkg-config ];

  buildInputs = [ openssl ] ++ lib.optionals stdenv.isDarwin [ curl darwin.Security ];

  nativeCheckInputs = [ nodejs ];

  # tests require it to be ran in the wasm-bindgen monorepo
  doCheck = false;
}
