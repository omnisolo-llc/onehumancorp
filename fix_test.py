import re

with open("srcs/server/orchestration/sync_daemon_test.go", "r") as f:
    content = f.read()

# Add a wait loop after daemon.Stop() to ensure the goroutine has finished before closing the DB
new_test = """
	daemon.Start(ctx)

	time.Sleep(50 * time.Millisecond)

	daemon.Stop()

	// Wait a moment for the goroutine to actually exit before we defer-close the DB
	time.Sleep(10 * time.Millisecond)
	// No panic implies successful shutdown via stop channel
"""

content = re.sub(r'\tdaemon\.Start\(ctx\)\n\n\ttime\.Sleep\(50 \* time\.Millisecond\)\n\n\tdaemon\.Stop\(\)\n\t// No panic implies successful shutdown via stop channel', new_test.strip("\n"), content)

with open("srcs/server/orchestration/sync_daemon_test.go", "w") as f:
    f.write(content)
