#!/bin/bash
export PATH=$PATH:$HOME/go/bin
bazelisk test //srcs/server/dashboard/... --test_filter=TestHandleRAGSync
