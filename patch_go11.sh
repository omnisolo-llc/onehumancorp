#!/bin/bash
sed -i '/"github.com\/onehumancorp\/mono\/srcs\/server\/api"/d' srcs/server/dashboard/server.go
sed -i 's/api.NewKairosStreamHandler(server.hub.GetTeammateMesh()).ServeWS//' srcs/server/dashboard/server.go
sed -i '/mux.HandleFunc("\/api\/kairos\/stream", )/d' srcs/server/dashboard/server.go
