#!/bin/bash
sed -i '/import (/a \
\t"github.com/onehumancorp/mono/srcs/server/api"' srcs/server/dashboard/server.go
