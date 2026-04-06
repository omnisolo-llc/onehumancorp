import re

with open('srcs/server/orchestration/service.go', 'r') as f:
    content = f.read()

# Add SetTaskManager method to Hub
hub_methods = """
// SetTaskManager assigns the TaskManager to the Hub for delegation queue access.
func (h *Hub) SetTaskManager(tm *TaskManager) {
	h.mu.Lock()
	defer h.mu.Unlock()
	h.taskManager = tm
}
"""

content = re.sub(
    r'(func \(h \*Hub\) CheckAccess\(agentID string, secret string\) bool \{)',
    hub_methods + r'\1',
    content
)

with open('srcs/server/orchestration/service.go', 'w') as f:
    f.write(content)
