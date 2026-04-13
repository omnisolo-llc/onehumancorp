#!/bin/bash
sed -i 's/orchestration.NewHub("test-org")/orchestration.NewHub()/g' srcs/server/api/kairos_stream_test.go
