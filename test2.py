import re

with open("srcs/server/orchestration/state_machine.go", "r") as f:
    content = f.read()

# Add Centrifuge Hub field to TaskStateMachine
if "node Node" not in content:
    content = re.sub(
        r'type TaskStateMachine struct \{\n\s*dbProvider db\.Provider\n\s*mutexProvider MutexProvider\n\}',
        'type TaskStateMachine struct {\n\tdbProvider db.Provider\n\tmutexProvider MutexProvider\n\tnode Node\n}',
        content
    )

if "node Node" not in content:
    # also try different spacing
    pass

# Change NewTaskStateMachine to accept Node and pass it
if "NewTaskStateMachine(provider db.Provider, redisClient rueidis.Client, node Node)" not in content:
    content = re.sub(
        r'func NewTaskStateMachine\(provider db\.Provider, redisClient rueidis\.Client\) \*TaskStateMachine \{',
        'func NewTaskStateMachine(provider db.Provider, redisClient rueidis.Client, node Node) *TaskStateMachine {',
        content
    )
    content = re.sub(
        r'return &TaskStateMachine\{dbProvider: provider, mutexProvider: mp\}',
        'return &TaskStateMachine{dbProvider: provider, mutexProvider: mp, node: node}',
        content
    )

if "StateTransitionEvent struct" not in content:
    content = re.sub(
        r'const \(\n\s*TaskStatePending',
        'type StateTransitionEvent struct {\n\tTaskID   string `json:"task_id"`\n\tEvent    string `json:"event"`\n\tNewState string `json:"new_state"`\n}\n\nconst (\n\tTaskStatePending',
        content
    )

with open("srcs/server/orchestration/state_machine.go", "w") as f:
    f.write(content)
print("success")
