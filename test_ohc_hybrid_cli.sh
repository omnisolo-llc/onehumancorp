#!/bin/bash
# Test for ohc_hybrid_cli.sh

echo "Running test..."
export OHC_STANDALONE=true
export DATABASE_URL=""

# Mock sqlite3
sqlite3() {
    echo "agent_missions"
    echo "meeting_rooms"
}
export -f sqlite3

TEST_HOME=$(mktemp -d); export HOME="$TEST_HOME"; mkdir -p $HOME/.ohc-local-data && touch $HOME/.ohc-local-data/standalone.db
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
    rm -f $HOME/.ohc-local-data/standalone.db
    kill -SIGINT $$
fi
rm -f $HOME/.ohc-local-data/standalone.db
