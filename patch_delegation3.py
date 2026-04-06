import re

with open('srcs/server/orchestration/delegation.go', 'r') as f:
    content = f.read()

# Fix compilation error due to duplicate/bad regex replacement
fixed_content = re.sub(
    r'\s+if err := s\.tm\.subAgentQueue\.Enqueue\(ctx, job\); err != nil \{\n\s+return nil, status\.Errorf\(codes\.Internal, "failed to enqueue sub-agent task: %v", err\)\n\s+\}\n\s+\}',
    '',
    content,
    flags=re.DOTALL
)

with open('srcs/server/orchestration/delegation.go', 'w') as f:
    f.write(fixed_content)
