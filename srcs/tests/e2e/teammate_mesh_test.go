package e2e

import (
	"context"
	"fmt"
	"testing"
	"time"

	playwright "github.com/playwright-community/playwright-go"
	"github.com/stretchr/testify/require"
)

func TestTeammateMesh_E2E(t *testing.T) {
	page := newPage(t)
	require.NotNil(t, page)

	// 1. Login
	require.NoError(t, login(page, "admin", "admin"))

	// 2. Navigate to Dashboard
	_, err := page.Goto("/")
	require.NoError(t, err)

	// 3. Verify we are on the dashboard
	require.NoError(t, page.WaitForSelector("text=One Human Corp Dashboard", playwright.PageWaitForSelectorOptions{
		Timeout: playwright.Float(10000),
	}))

	// 4. Trigger a Mesh Broadcast via API (simulating an agent)
	// We use the 'system' role which is allowed for these APIs in the dashboard server.
	// In a real E2E, we might trigger this via a UI button if available,
	// but the requirement is to verify the mesh event bus through the UI.

	// Since we don't have a specific 'Mesh Test' button in the UI, we'll verify that
	// a broadcasted task appears or updates the UI.

	taskID := fmt.Sprintf("e2e-task-%d", time.Now().UnixNano())
	broadcastPayload := map[string]interface{}{
		"channel": "mesh:tasks",
		"agent_id": "e2e-agent",
		"action": "E2E_TEST_ACTION",
		"status": "active",
		"payload": map[string]interface{}{
			"task_id": taskID,
		},
		"event_type": "TaskBroadcast", // For ValidationMiddleware
		"data": map[string]interface{}{"task_id": taskID}, // For ValidationMiddleware
	}

	// We'll use page.Evaluate to send the POST request from the browser context to avoid CORS/auth issues
	_, err = page.Evaluate(`async (payload) => {
		const resp = await fetch('/api/mesh/broadcast', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(payload)
		});
		if (!resp.ok) throw new Error('Broadcast failed: ' + await resp.text());
	}`, broadcastPayload)
	require.NoError(t, err)

	// 5. Assert the result is visible in the UI
	// The dashboard should show the task queue length or specific task.
	// Wait for the task to appear in the dashboard's task queue or status update.
	// OHC Dashboard usually has a "Task Queue" section.

	// Verification logic depends on how the UI reflects mesh events.
	// According to design docs, Centrifuge propagates events to the dashboard.

	// We'll check for the taskID or the action string in the UI.
	require.NoError(t, page.WaitForSelector(fmt.Sprintf("text=%s", "E2E_TEST_ACTION"), playwright.PageWaitForSelectorOptions{
		Timeout: playwright.Float(15000),
	}))
}
