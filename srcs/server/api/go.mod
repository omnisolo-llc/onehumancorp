module github.com/onehumancorp/mono/srcs/server/api

go 1.24.12

require (
    github.com/gorilla/websocket v1.5.3
    github.com/onehumancorp/mono/srcs/server/orchestration v0.0.0
)

replace github.com/onehumancorp/mono/srcs/server/orchestration => ../orchestration
