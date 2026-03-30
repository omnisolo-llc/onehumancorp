import re

with open('srcs/orchestration/sip.go', 'r') as f:
    content = f.read()

new_retry = """func withRetry(ctx context.Context, op func() error) error {
	var err error
	for i := 0; i < maxRetries; i++ {
		err = op()
		if err == nil {
			return nil
		}

		// If context is done, abort retries
		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
		}

		// Only retry on busy or locked errors, otherwise break early
		errStr := err.Error()
		if !(strings.Contains(errStr, "database is locked") || strings.Contains(errStr, "SQLITE_BUSY")) {
			return err
		}

		slog.Warn("sipdb: operation failed, retrying", "attempt", i+1, "error", err)
		time.Sleep(retryInterval * time.Duration(1<<i))
	}
	return err
}"""

content = re.sub(r'func withRetry\(ctx context\.Context, op func\(\) error\) error \{.*?\n\}', new_retry, content, flags=re.DOTALL)

with open('srcs/orchestration/sip.go', 'w') as f:
    f.write(content)
