#!/bin/bash
export OHC_STANDALONE=true
bazelisk test //srcs/server/dashboard:dashboard_test
