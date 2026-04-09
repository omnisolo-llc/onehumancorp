#!/bin/bash
ls srcs/server/db/migrations/*.sql | sed 's/.*\/0*\([0-9]*\)_.*\.sql/\1/' | sort -n | tail -1
