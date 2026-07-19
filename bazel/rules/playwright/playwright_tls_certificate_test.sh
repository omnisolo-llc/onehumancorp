#!/bin/bash
set -euo pipefail

generator="$1"
work_dir="$(mktemp -d)"
trap 'rm -rf "$work_dir"' EXIT

"$generator" "$work_dir"

for file in ca.crt ca.key server.crt server.key; do
  test -s "$work_dir/$file"
done

openssl x509 -in "$work_dir/ca.crt" -noout -text | grep -Eq 'Version: 3 '
openssl x509 -in "$work_dir/ca.crt" -noout -ext basicConstraints | grep -q 'CA:TRUE'
openssl x509 -in "$work_dir/server.crt" -noout -text | grep -Eq 'Version: 3 '
openssl x509 -in "$work_dir/server.crt" -noout -ext basicConstraints | grep -q 'CA:FALSE'
openssl x509 -in "$work_dir/server.crt" -noout -checkhost localhost
openssl verify -purpose sslserver -CAfile "$work_dir/ca.crt" "$work_dir/server.crt"
