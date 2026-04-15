import sys

with open('srcs/server/orchestration/tasks_store_test.go', 'r') as f:
    content = f.read()

# rename mockTx, mockRow, mockDB to something unique
content = content.replace('mockTx', 'storeMockTx')
content = content.replace('mockRow', 'storeMockRow')
content = content.replace('mockDB', 'storeMockDB')

# db.CommandTag doesn't exist, remove the return type if possible, or use standard pgconn.CommandTag or similar.
# Wait, db.Tx Exec returns pgconn.CommandTag from pgx?
# The interface in mono/srcs/server/db probably returns (pgconn.CommandTag, error)
# Let's see what db.Tx interface is. We can just use (interface{}, error) if we want? Actually, it's typed.
# Let's grep db.Tx to see what it is
