package dashboard

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/billing"
	"github.com/onehumancorp/mono/src/server/domain"
	"github.com/onehumancorp/mono/src/server/orchestration"
)

type mockTracker struct {
	billing.TokenTracker
}

func (m *mockTracker) Summary(orgID string) billing.Summary {
	time.Sleep(10 * time.Millisecond)
	return billing.Summary{}
}
func (m *mockTracker) TrackAgentTokenUsage(ctx context.Context, organizationID, agentID, model string, promptTokens, completionTokens, cachedTokens int, isAction bool) error {
	return nil
}

type mockHubPerf struct {
	orchestration.Hub
	tm *mockTaskManagerPerf
}

func (m *mockHubPerf) Agents() []orchestration.Agent {
	time.Sleep(10 * time.Millisecond)
	return []orchestration.Agent{{ID: "agent-1", OrganizationID: "org-1"}}
}

func (m *mockHubPerf) Meetings() []orchestration.MeetingRoom {
	time.Sleep(10 * time.Millisecond)
	return []orchestration.MeetingRoom{{ID: "meeting-1", Name: "room"}}
}

func (m *mockHubPerf) TaskManager() orchestration.TaskManager {
	return m.tm
}

type mockTaskManagerPerf struct {
	orchestration.TaskManager
}

func (m *mockTaskManagerPerf) PeekTasks(ctx context.Context, limit int) ([]*orchestration.SharedTask, error) {
	time.Sleep(10 * time.Millisecond)
	return []*orchestration.SharedTask{{ID: "task-1"}}, nil
}

func BenchmarkSnapshotLocked(b *testing.B) {
	s := &Server{
		org:     domain.Organization{ID: "org-1"},
		hub:     &mockHubPerf{tm: &mockTaskManagerPerf{}},
		tracker: &mockTracker{},
	}

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_ = s.snapshotLocked()
	}
}
