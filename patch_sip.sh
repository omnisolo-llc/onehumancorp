#!/bin/bash
sed -i 's/isMultiTenant := envBoolDefault("OHC_MULTITENANT", false)//g' srcs/server/orchestration/sip.go
sed -i 's/isSQLite := s.db != nil \/\/ We know it'\''s SQLite since this is SIPDB which only uses SQLite//g' srcs/server/orchestration/sip.go
