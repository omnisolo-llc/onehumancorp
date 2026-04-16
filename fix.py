import sys

with open("srcs/server/orchestration/sip.go", "r") as f:
    content = f.read()

# restore DelegateMission to what the plan exactly wanted
search_str = """
func (s *SIPDB) DelegateMission(ctx context.Context, missionID, role string, task Message) error {
	_ = CheckDocumentationGate(task.Content)
"""
replace_str = """
func (s *SIPDB) DelegateMission(ctx context.Context, missionID, role string, task Message) error {
	if err := acquireThrottle(ctx); err != nil {
		return err
	}
	defer releaseThrottle()

	_ = CheckDocumentationGate(task.Content)
"""
if search_str in content:
    content = content.replace(search_str, replace_str)

with open("srcs/server/orchestration/sip.go", "w") as f:
    f.write(content)
