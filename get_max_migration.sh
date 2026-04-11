#!/bin/bash
ls srcs/server/db/migrations/*.sql | sed 's/.*migrations\///' | grep -o '^[0-9]\+' | sort -n | tail -1
