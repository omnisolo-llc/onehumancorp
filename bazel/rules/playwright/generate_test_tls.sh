#!/bin/bash
set -euo pipefail

if [[ $# -ne 1 || -z "$1" ]]; then
  echo "usage: $0 OUTPUT_DIRECTORY" >&2
  exit 2
fi

output_dir="$1"
mkdir -p "$output_dir"
umask 077

openssl req \
  -x509 \
  -newkey rsa:2048 \
  -nodes \
  -keyout "$output_dir/ca.key" \
  -sha256 \
  -days 365 \
  -out "$output_dir/ca.crt" \
  -subj "/CN=OHC E2E Test CA" \
  -addext "basicConstraints=critical,CA:TRUE" \
  -addext "keyUsage=critical,keyCertSign,cRLSign" \
  -addext "subjectKeyIdentifier=hash" \
  >/dev/null 2>&1

openssl req \
  -new \
  -newkey rsa:2048 \
  -nodes \
  -keyout "$output_dir/server.key" \
  -out "$output_dir/server.csr" \
  -subj "/CN=localhost" \
  >/dev/null 2>&1

extensions_file="$output_dir/server.ext"
{
  echo "basicConstraints=critical,CA:FALSE"
  echo "keyUsage=critical,digitalSignature,keyEncipherment"
  echo "extendedKeyUsage=serverAuth"
  echo "subjectAltName=DNS:localhost,IP:127.0.0.1"
  echo "subjectKeyIdentifier=hash"
  echo "authorityKeyIdentifier=keyid,issuer"
} > "$extensions_file"

openssl x509 \
  -req \
  -in "$output_dir/server.csr" \
  -CA "$output_dir/ca.crt" \
  -CAkey "$output_dir/ca.key" \
  -CAcreateserial \
  -out "$output_dir/server.crt" \
  -days 365 \
  -sha256 \
  -extfile "$extensions_file" \
  >/dev/null 2>&1

rm -f "$output_dir/server.csr" "$extensions_file" "$output_dir/ca.srl"
