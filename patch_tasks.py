import re

with open('srcs/server/orchestration/tasks.go', 'r') as f:
    content = f.read()

# Add queue import
content = re.sub(
    r'"github.com/onehumancorp/mono/srcs/server/orchestration/statemachine"',
    '"github.com/onehumancorp/mono/srcs/server/orchestration/statemachine"\n\t"github.com/onehumancorp/mono/srcs/server/orchestration/queue"',
    content
)

# Add subAgentQueue to TaskManager struct
content = re.sub(
    r'stateMachine \*statemachine\.StateMachine',
    'stateMachine *statemachine.StateMachine\n\tsubAgentQueue queue.TaskQueue',
    content
)

# Initialize TaskQueue in NewTaskManager
init_replacement = """	tm := &TaskManager{
		db:           provider,
		hub:          hub,
		stateMachine: statemachine.NewStateMachine(provider, broadcast),
	}

	var redisClient rueidis.Client
	if os.Getenv("OHC_MULTITENANT") == "true" {
		redisURL := os.Getenv("REDIS_URL")
		if redisURL != "" {
			opts, err := rueidis.ParseURL(redisURL)
			if err == nil {
				c, err := rueidis.NewClient(opts)
				if err == nil {
					redisClient = c
					tm.redisClient = c
				}
			}
		}
	}

	tm.subAgentQueue = queue.NewTaskQueue(provider, redisClient)
	tm.stopChan = make(chan struct{})
	return tm"""

content = re.sub(
    r'tm := &TaskManager\{.*?\n\s+return tm',
    init_replacement,
    content,
    flags=re.DOTALL
)

with open('srcs/server/orchestration/tasks.go', 'w') as f:
    f.write(content)
