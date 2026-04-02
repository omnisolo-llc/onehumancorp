#!/bin/bash
go get github.com/gorilla/websocket
go get github.com/redis/go-redis/v9
bazelisk run //:gazelle
