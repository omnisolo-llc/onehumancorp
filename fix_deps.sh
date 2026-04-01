#!/bin/bash
go get github.com/sony/gobreaker
bazelisk run //:gazelle -- update-repos -from_file=go.mod -to_macro=repositories.bzl%go_repositories
bazelisk run //:gazelle
