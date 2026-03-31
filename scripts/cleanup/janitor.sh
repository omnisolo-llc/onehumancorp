#!/bin/bash
# Prune obsolete generated protobuf files and temporary scripts
find . -name '*.pb.go' -o -name '*.pb.ts' -o -name '*_pb2.py' | xargs rm -f
find . -name '*.py' | xargs rm -f
echo 'Cleanup complete'
