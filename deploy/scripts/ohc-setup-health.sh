#!/bin/bash
set -e
echo -e "\n[Running OHC Setup Health Check]"
echo "Verifying Bazelisk..."
bazelisk version > /dev/null
echo "✅ Bazelisk is healthy!"
echo "Verifying Go..."
go version > /dev/null
echo "✅ Go is healthy!"
echo "✅ All systems go!"
