#!/bin/bash
# Test for ohc_hybrid_cli.sh

echo "Running test..."
export OHC_STANDALONE=true
export DATABASE_URL=""
export HOME=$(pwd)/test_home

# Mock sqlite3
sqlite3() {
    echo "agent_missions"
    echo "meeting_rooms"
}
export -f sqlite3

mkdir -p test_home/.ohc-local-data
touch test_home/.ohc-local-data/standalone.db

OUTPUT=$(./ohc_hybrid_cli.sh << CMD
7
q
CMD
)

if echo "$OUTPUT" | grep -q "Migrations appear successful"; then
    echo "Test passed."
else
    echo "Test failed."
    echo "$OUTPUT"
    rm -rf test_home
    kill -SIGINT $$
fi
rm -rf test_home
