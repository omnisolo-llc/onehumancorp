package dashboard

import (
	"encoding/json"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/domain"
	"github.com/onehumancorp/mono/srcs/server/integrations"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestHybridMCPBridge_InvokeTool_EscalationFlag(t *testing.T) {
	// 100% coverage requirement: if running standalone, set OHC_STANDALONE=true
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	org := domain.NewSoftwareCompany("test-org", "Test Org", "CEO", time.Now())
	hub := orchestration.NewHub()
	defer hub.Close()

	integReg := integrations.NewRegistry()
	srv := NewServer(org, hub, nil, integReg)

	// We'll invoke the git-mcp tool that we modified to return HybridEscalation.
	payload := mcpInvokeRequest{
		ToolID: "git-mcp",
		AgentID: "agent-1",
		Action: "execute",
		Params: json.RawMessage(`{"repository": "org/repo", "title": "Fix bug", "body": "Fixed", "sourceBranch": "feature", "targetBranch": "main", "createdBy": "user"}`),
	}

	// Create mock integration for github
	integReg.Register(&mockGitHubIntegration{})

	res, err := srv.invokeMCPTool(payload)
	if err != nil {
		t.Fatalf("invokeMCPTool failed: %v", err)
	}

	if val, ok := res["HybridEscalation"]; !ok || val != true {
		t.Errorf("Expected HybridEscalation flag to be true, got %v", val)
	}
}

type mockGitHubIntegration struct{}

func (m *mockGitHubIntegration) ID() string { return "github" }
func (m *mockGitHubIntegration) Type() integrations.IntegrationType { return integrations.TypeSourceControl }
func (m *mockGitHubIntegration) Init() error { return nil }
func (m *mockGitHubIntegration) Connect() error { return nil }
func (m *mockGitHubIntegration) Disconnect() error { return nil }
func (m *mockGitHubIntegration) Status() integrations.IntegrationStatus { return integrations.StatusConnected }
func (m *mockGitHubIntegration) Configure(config map[string]interface{}) error { return nil }

func (m *mockGitHubIntegration) ListPullRequests(repo string) ([]integrations.PullRequest, error) {
	return nil, nil
}
func (m *mockGitHubIntegration) CreatePullRequest(repo, title, body, sourceBranch, targetBranch, createdBy string, ts time.Time) (integrations.PullRequest, error) {
	return integrations.PullRequest{
		ID: "pr-1",
		Title: title,
	}, nil
}
func (m *mockGitHubIntegration) MergePullRequest(repo, id string) error {
	return nil
}

// Ensure it implements needed methods
func (m *mockGitHubIntegration) SendChatMessage(channel, fromAgent, content, threadID string, ts time.Time) (integrations.ChatMessage, error) {
	return integrations.ChatMessage{}, nil
}
func (m *mockGitHubIntegration) CreateIssue(project, title, description, createdBy string, priority integrations.IssuePriority, labels []string, ts time.Time) (integrations.Issue, error) {
	return integrations.Issue{}, nil
}
