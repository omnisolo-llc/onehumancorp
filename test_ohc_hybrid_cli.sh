#!/bin/bash
# Test for ohc_hybrid_cli.sh

echo "Running test..."
export OHC_STANDALONE=true
export DATABASE_URL=""

# Mock sqlite3
sqlite3() {
    if [ "$2" = "PRAGMA integrity_check;" ]; then
        echo "ok"
    else
        echo "agent_missions"
        echo "meeting_rooms"
        echo "teammate_mesh"
        echo "shared_tasks"
        echo "autodream_pipeline"
    fi
}
export -f sqlite3

TEST_HOME=$(mktemp -d)
export HOME="$TEST_HOME"

mkdir -p "$HOME/.ohc-local-data"
touch "$HOME/.ohc-local-data/standalone.db"
OUTPUT=$(./ohc_hybrid_cli.sh << CMD
7
q
CMD
)

if echo "$OUTPUT" | grep -q "Migrations appear successful" && echo "$OUTPUT" | grep -q "DB Integrity Check Passed."; then
    echo "Test passed."
else
    echo "Test failed."
    echo "$OUTPUT"
    rm -rf "$TEST_HOME"
    kill -SIGINT $$
fi
rm -rf "$TEST_HOME"
