import re

with open('srcs/server/orchestration/delegation.go', 'r') as f:
    content = f.read()

# Hub does not have direct access to TaskManager (s.tm)
delegation_logic = """
	if s.hub.sipDB != nil {
		s.hub.LogEvent(subAgent)
	}

	// Enqueue sub-agent task using the Hub's TaskManager queue if available.
	if s.hub.taskManager != nil && s.hub.taskManager.subAgentQueue != nil {
		job := &queue.Job{
			ParentTaskID: req.GetTaskId(),
			AgentRole:    req.GetTargetRole(),
			Payload:      req.GetInstruction(),
		}

		if err := s.hub.taskManager.subAgentQueue.Enqueue(ctx, job); err != nil {
			return nil, status.Errorf(codes.Internal, "failed to enqueue sub-agent task: %v", err)
		}
	}
"""

content = re.sub(
    r'if s\.hub\.sipDB != nil \{\n\s+s\.hub\.LogEvent\(subAgent\)\n\s+\}\n\s+// Enqueue sub-agent task using the TaskManager.*?\}',
    delegation_logic,
    content,
    flags=re.DOTALL
)

with open('srcs/server/orchestration/delegation.go', 'w') as f:
    f.write(content)
