#!/bin/bash
sed -i '283,296d' srcs/server/main.go
sed -i '/var redisClient rueidis.Client/i \	var teammateMesh mesh.TeammateMesh' srcs/server/main.go
sed -i '/redisClient, _ = rueidis.NewClient(opts)/a \			if os.Getenv("OHC_STANDALONE") != "true" {\n\t\t\t\tteammateMesh = mesh.NewRedisMesh(redisClient)\n\t\t\t}' srcs/server/main.go
sed -i '/	if redisURL := os.Getenv("REDIS_URL"); redisURL != "" {/,+5a \	if teammateMesh == nil {\n\t\tteammateMesh = mesh.NewLocalMesh()\n\t}' srcs/server/main.go
