#!/bin/bash
sed -i '/import (/a \	"github.com\/onehumancorp\/mono\/srcs\/server\/orchestration\/mesh"' srcs/server/orchestration/hub.go
