with open('srcs/server/agents/builtin/cost_tracker_e2e_test.go', 'r') as f:
    content = f.read()

import_time = """	"time"
"""
content = content.replace('"strings"', '"strings"\n' + import_time)

wait_code = """	// wait for completion
	// In our fake it's very fast, we can just poll the state
	for i := 0; i < 100; i++ {
		if state.Status() == TaskStatusCompleted || state.Status() == TaskStatusFailed {
			break
		}
		// wait a bit
		time.Sleep(10 * time.Millisecond)
	}
"""
content = content.replace("""	// wait for completion
	// In our fake it's very fast, we can just poll the state
	for i := 0; i < 100; i++ {
		if state.Status() == TaskStatusCompleted || state.Status() == TaskStatusFailed {
			break
		}
		// wait a bit
	}""", wait_code)

with open('srcs/server/agents/builtin/cost_tracker_e2e_test.go', 'w') as f:
    f.write(content)
