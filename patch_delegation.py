import re

with open('srcs/server/orchestration/delegation.go', 'r') as f:
    content = f.read()

# Add queue import
content = re.sub(
    r'"google.golang.org/protobuf/proto"',
    '"google.golang.org/protobuf/proto"\n\t"github.com/onehumancorp/mono/srcs/server/orchestration/queue"',
    content
)

# Find end of provisioning but before sub-agent message creation to enqueue the job.
# We also need to extract payload JSON creation
delegation_logic = """
	if s.hub.sipDB != nil {
		s.hub.LogEvent(subAgent)
	}

	// Enqueue sub-agent task using the TaskManager's subAgentQueue if available.
	// Since HubServiceServer has access to s.tm (TaskManager), we can queue it.
	if s.tm != nil && s.tm.subAgentQueue != nil {
		job := &queue.Job{
			ParentTaskID: req.GetTaskId(),
			AgentRole:    req.GetTargetRole(),
			Payload:      req.GetInstruction(), // Store instruction directly or encode as JSON
		}

		if err := s.tm.subAgentQueue.Enqueue(ctx, job); err != nil {
			return nil, status.Errorf(codes.Internal, "failed to enqueue sub-agent task: %v", err)
		}
	}
"""

content = re.sub(
    r'if s\.hub\.sipDB != nil \{\n\s+s\.hub\.LogEvent\(subAgent\)\n\s+\}',
    delegation_logic,
    content,
    flags=re.DOTALL
)

with open('srcs/server/orchestration/delegation.go', 'w') as f:
    f.write(content)
