#!/bin/bash
cat << 'INNER_EOF' > srcs/server/tools/tools.go
//go:build tools

package tools

import (
	_ "google.golang.org/grpc/cmd/protoc-gen-go-grpc"
	_ "google.golang.org/protobuf/cmd/protoc-gen-go"
	_ "github.com/onehumancorp/mono/srcs/server/tools/hybridcrdtmcp"
)
INNER_EOF
