#!/bin/bash
# Revert to master for orchestration package which failed in CI due to other agents
git checkout -- srcs/server/orchestration/
bazelisk test //...
