#!/bin/bash
export PATH=$PATH:$HOME/go/bin
go get github.com/alicebob/miniredis/v2
go mod tidy
bazelisk run //:gazelle -- update-repos -from_file=go.mod -to_macro=repositories.bzl%go_repositories -prune
bazelisk run //:gazelle
