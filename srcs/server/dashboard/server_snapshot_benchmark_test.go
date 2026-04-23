package dashboard

import (
	"context"
	"fmt"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/billing"
	"github.com/onehumancorp/mono/srcs/server/domain"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func BenchmarkSnapshotLocked(b *testing.B) {
	org := domain.Organization{ID: "org-1"}
	hub := orchestration.NewHub()
	tracker := billing.NewTracker(billing.DefaultCatalog)
	authStore := setupAuthStore()

	// Seed heavy load
	for i := 0; i < 1000; i++ {
		agentID := fmt.Sprintf("agent-%d", i)
		meetingID := fmt.Sprintf("meeting-%d", i)
		hub.Hire(orchestration.Agent{ID: agentID, OrganizationID: "org-1", Status: orchestration.StatusActive})
		hub.CreateMeeting(orchestration.MeetingRoom{ID: meetingID, Participants: []string{agentID}})
	}

	for i := 0; i < 1000; i++ {
		taskID := fmt.Sprintf("task-%d", i)
		hub.TaskManager().PublishTask(context.Background(), orchestration.SharedTask{ID: taskID, OrganizationID: "org-1"})
	}

	handler := NewServer(org, hub, tracker, authStore)
	server := handler.(*Server)

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		server.snapshotLocked()
	}
}
