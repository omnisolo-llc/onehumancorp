#!/bin/bash
cd srcs/server
go get github.com/golang-jwt/jwt/v5
cd ../../
bazelisk run //:gazelle -- update srcs/server/auth
bazelisk test //srcs/server/auth/...
