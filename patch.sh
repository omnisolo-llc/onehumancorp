#!/bin/bash
sed -i 's/VALUES ('\''m2'\'', '\''PENDING'\'', '\''{"role":"ROLE","raw":"invalid_json"}'\'')/VALUES ('\''m2'\'', '\''PENDING'\'', '\''{"role":"ROLE","raw":"invalid_json"}'\'')/g' srcs/server/orchestration/sip_extra_test.go
