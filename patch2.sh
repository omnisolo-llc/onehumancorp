#!/bin/bash
sed -i 's/'\''{"role":"ROLE","raw":"invalid_json"}'\''/'\''{"role":"ROLE","raw":"invalid_json"}'\''/g' srcs/server/orchestration/sip_extra_test.go
