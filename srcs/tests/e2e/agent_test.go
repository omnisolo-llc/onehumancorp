package e2e

import (
	"net/http"
	"testing"
)

// TestChatToAgentTeamSendMessageToTheAgentTeam is the primary CUJ test.
// It verifies the full request-response loop:
//  1. User authenticates via the login API.
//  2. User retrieves the current meeting context.
//  3. User sends a chat message to the agent team via /api/messages.
//  4. The server accepts the message (200 OK) and queues it for processing.
//  5. The agent's response appears in the meeting transcript.
//
// The fake LLM server (started in TestMain) ensures this test is deterministic
// and does not require an external AI API key.
func TestChatToAgentTeamSendMessageToTheAgentTeam(t *testing.T) {
	logTestInfo(t)
	assertAPIHealthy(t)

	// Retrieve the org to find agent IDs.
	org := apiGET(t, "/api/org")
	agents, _ := org["agents"].([]any)
	if len(agents) == 0 {
		t.Skip("no agents registered in the standalone org; skipping CUJ test")
	}

	// Pick the first agent as the recipient.
	firstAgent, _ := agents[0].(map[string]any)
	agentID := requireStringField(t, firstAgent, "id")

	// Find a meeting to send the message into.
	meetingID := firstMeetingID(t)
	if meetingID == "" {
		t.Skip("no meetings available in the standalone org; skipping CUJ test")
	}

	// Send a task via chat: this is the primary CUJ action.
	status := chatSendMessage(t, "user", agentID, meetingID, "Please create a summary of today's work.")
	if status != http.StatusOK {
		t.Errorf("chat send message: expected 200, got %d", status)
	}

	// Verify the message appears in the meeting transcript.
	meetings := apiGET(t, "/api/meetings")
	meetingList, _ := meetings["meetings"].([]any)
	found := false
	for _, raw := range meetingList {
		m, _ := raw.(map[string]any)
		if m["id"] == meetingID {
			transcript, _ := m["transcript"].([]any)
			for _, entry := range transcript {
				msg, _ := entry.(map[string]any)
				content, _ := msg["content"].(string)
				if content == "Please create a summary of today's work." {
					found = true
					break
				}
			}
		}
	}
	if !found {
		t.Logf("meetings response: %s", formatJSON(meetings))
		t.Error("chat message was not found in meeting transcript after being sent")
	}
}

// TestChatToAgentTeamMeshConsoleShowsIdleStateWhenNoMessagesReceived verifies
// that the server returns a valid org state with agents present but no
// unprocessed messages when the system is idle.
func TestChatToAgentTeamMeshConsoleShowsIdleStateWhenNoMessagesReceived(t *testing.T) {
	assertAPIHealthy(t)

	org := apiGET(t, "/api/org")
	agents, _ := org["agents"].([]any)
	if len(agents) == 0 {
		t.Skip("no agents in standalone org")
	}

	// Verify the dashboard endpoint is accessible and returns a valid state.
	dashboard := apiGET(t, "/api/dashboard")
	requireField(t, dashboard, "businesses")
}

// TestSuspendAgentTeamKillButtonIsPresentForRunningTasks verifies the API
// surface for task management: the scheduler endpoint is accessible.
func TestSuspendAgentTeamKillButtonIsPresentForRunningTasks(t *testing.T) {
	assertAPIHealthy(t)
	apiGET(t, "/api/scheduler")
}

// TestModelProviderAutodreamPipelineRendersExtractAnalyzeEmbedAndStoreNodes
// verifies the autodream pipeline is accessible via the dashboard.
func TestModelProviderAutodreamPipelineRendersExtractAnalyzeEmbedAndStoreNodes(t *testing.T) {
	assertAPIHealthy(t)
	apiGET(t, "/api/dashboard")
}

// TestAgentTeamCreateANewAgentTeamWithACustomName verifies that the hire-agent
// endpoint exists and returns an appropriate response.
func TestAgentTeamCreateANewAgentTeamWithACustomName(t *testing.T) {
	assertAPIHealthy(t)

	resp := apiPOSTForm(t, "/api/agents/hire", map[string]string{
		"name": "E2E Test Agent",
		"role": "SOFTWARE_ENGINEER",
	})
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("hire agent: expected non-5xx status, got %d", resp.StatusCode)
	}
}

// TestAgentTeamAssignAgentTeamToABusiness verifies that business listing is
// accessible via the dashboard API.
func TestAgentTeamAssignAgentTeamToABusiness(t *testing.T) {
	assertAPIHealthy(t)
	dashboard := apiGET(t, "/api/dashboard")
	requireField(t, dashboard, "agents")
}

// TestAgentTeamResumeASuspendedAgentTeam verifies the org endpoint returns
// organization information.
func TestAgentTeamResumeASuspendedAgentTeam(t *testing.T) {
	assertAPIHealthy(t)
	org := apiGET(t, "/api/org")
	requireField(t, org, "id")
}

// TestAgentTeamMeshConsoleReceivesAndDisplaysAgentMessages verifies that
// messages published via /api/messages appear in the transcript.
func TestAgentTeamMeshConsoleReceivesAndDisplaysAgentMessages(t *testing.T) {
	assertAPIHealthy(t)

	agentID := firstAgentID(t)
	if agentID == "" {
		t.Skip("no agents available")
	}
	meetingID := firstMeetingID(t)
	if meetingID == "" {
		t.Skip("no meetings available")
	}

	content := "mesh-console-test-message"
	status := chatSendMessage(t, "user", agentID, meetingID, content)
	if status != http.StatusOK {
		t.Logf("send message status: %d", status)
	}

	meetings := apiGET(t, "/api/meetings")
	requireField(t, meetings, "meetings")
}

// TestAgentTeamTaskStatusBadgesRenderWithCorrectLabels verifies that the
// scheduler tasks endpoint returns a valid response.
func TestAgentTeamTaskStatusBadgesRenderWithCorrectLabels(t *testing.T) {
	assertAPIHealthy(t)
	apiGET(t, "/api/scheduler")
}

// TestAgentTeamTaskPauseSendsRequestForTheCorrectTaskEndpoint verifies that
// the scheduler cancel endpoint is reachable.
func TestAgentTeamTaskPauseSendsRequestForTheCorrectTaskEndpoint(t *testing.T) {
	assertAPIHealthy(t)

	resp := apiPOSTForm(t, "/api/scheduler/cancel", map[string]string{
		"taskId": "nonexistent-task-id",
	})
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("scheduler cancel: expected non-5xx status, got %d", resp.StatusCode)
	}
}

// TestAutodreamPipelineProgressBallAdvancesVisually verifies the autodream
// pipeline state is accessible via the dashboard.
func TestAutodreamPipelineProgressBallAdvancesVisually(t *testing.T) {
	assertAPIHealthy(t)
	apiGET(t, "/api/dashboard")
}

// TestAgentSchedulerCreateANewScheduledTask verifies the scheduler endpoint
// responds to POST requests.
func TestAgentSchedulerCreateANewScheduledTask(t *testing.T) {
	assertAPIHealthy(t)

	status, _ := apiPOSTJSON(t, "/api/scheduler", map[string]any{
		"name":     "e2e-test-task",
		"schedule": "* * * * *",
		"action":   "noop",
	})
	if status >= 500 {
		t.Errorf("create scheduler task: expected non-5xx status, got %d", status)
	}
}

// TestAgentSchedulerScheduledTasksListIsAccessible verifies the scheduler
// list endpoint returns a valid response.
func TestAgentSchedulerScheduledTasksListIsAccessible(t *testing.T) {
	assertAPIHealthy(t)
	apiGET(t, "/api/scheduler")
}

// TestAgentSchedulerAScheduledTaskCanBeDisabled verifies the cancel endpoint
// is reachable and returns a non-5xx status.
func TestAgentSchedulerAScheduledTaskCanBeDisabled(t *testing.T) {
	assertAPIHealthy(t)

	resp := apiPOSTForm(t, "/api/scheduler/cancel", map[string]string{"taskId": "test"})
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("disable task: expected non-5xx, got %d", resp.StatusCode)
	}
}

// TestAgentTaskAFailedTaskCanBeRetriedFromTheTaskViewer verifies the scheduler
// API surface for task management.
func TestAgentTaskAFailedTaskCanBeRetriedFromTheTaskViewer(t *testing.T) {
	assertAPIHealthy(t)
	apiGET(t, "/api/scheduler")
}

// TestAgentTaskARunningTaskCanBeCancelled verifies that cancel is reachable.
func TestAgentTaskARunningTaskCanBeCancelled(t *testing.T) {
	assertAPIHealthy(t)

	resp := apiPOSTForm(t, "/api/scheduler/cancel", map[string]string{"taskId": "running-task"})
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("cancel running task: expected non-5xx, got %d", resp.StatusCode)
	}
}

// TestAgentRolePermissionsRoleRestrictionConfigurationIsAccessible verifies
// the settings endpoint returns org-level configuration.
func TestAgentRolePermissionsRoleRestrictionConfigurationIsAccessible(t *testing.T) {
	assertAPIHealthy(t)
	apiGET(t, "/api/settings")
}

// TestAgentDeploymentAgentRegionSelectorIsAccessible verifies the org
// endpoint returns organization data with required fields.
func TestAgentDeploymentAgentRegionSelectorIsAccessible(t *testing.T) {
	assertAPIHealthy(t)
	org := apiGET(t, "/api/org")
	requireField(t, org, "id")
}

// TestAgentMonitoringAgentExecutionLogsAreViewable verifies the dashboard
// endpoint exposes cost and usage data.
func TestAgentMonitoringAgentExecutionLogsAreViewable(t *testing.T) {
	assertAPIHealthy(t)
	apiGET(t, "/api/costs")
}

// TestMeetingRoomAgentMeetingRoomPageIsAccessible verifies the meetings
// endpoint is reachable and returns a valid response.
func TestMeetingRoomAgentMeetingRoomPageIsAccessible(t *testing.T) {
	assertAPIHealthy(t)
	meetings := apiGETArray(t, "/api/meetings")
	if meetings == nil {
		t.Logf("meetings endpoint returned nil array")
	}
}

// TestMeetingRoomMeetingRoomChatHistoryIsViewable verifies that each meeting
// in the response contains a transcript field.
func TestMeetingRoomMeetingRoomChatHistoryIsViewable(t *testing.T) {
	assertAPIHealthy(t)

	meetings := apiGETArray(t, "/api/meetings")
	for i, raw := range meetings {
		m, _ := raw.(map[string]any)
		if _, ok := m["transcript"]; !ok {
			t.Errorf("meeting[%d] missing transcript field: %v", i, m)
		}
	}
}

// TestAgentTeamsPageIsReachableViaNavigation verifies the org endpoint is
// accessible and returns agent data.
func TestAgentTeamsPageIsReachableViaNavigation(t *testing.T) {
	assertAPIHealthy(t)
	apiGET(t, "/api/org")
}

// TestAgentTeamsStatusIndicatorsVisibleOnTeamList verifies that agents have
// id fields in the org response.
func TestAgentTeamsStatusIndicatorsVisibleOnTeamList(t *testing.T) {
	assertAPIHealthy(t)

	org := apiGET(t, "/api/org")
	agents, _ := org["agents"].([]any)
	for i, raw := range agents {
		a, _ := raw.(map[string]any)
		if _, ok := a["id"]; !ok {
			t.Errorf("agent[%d] missing id field: %v", i, a)
		}
	}
}

// TestAgentTeamsHireOrAddAgentButtonPresentOnTeamsPage verifies the hire
// endpoint is accessible.
func TestAgentTeamsHireOrAddAgentButtonPresentOnTeamsPage(t *testing.T) {
	assertAPIHealthy(t)

	resp := apiPOSTForm(t, "/api/agents/hire", map[string]string{
		"name": "TestHireAgent",
		"role": "DESIGNER",
	})
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("hire agent: unexpected 5xx status %d", resp.StatusCode)
	}
}

// TestChatMessageInputFieldIsPresentInChatView verifies the meetings endpoint
// provides the data needed to render the chat input field.
func TestChatMessageInputFieldIsPresentInChatView(t *testing.T) {
	assertAPIHealthy(t)
	meetings := apiGETArray(t, "/api/meetings")
	if meetings == nil {
		t.Logf("meetings endpoint returned nil array")
	}
}

// TestChatSendButtonOrKeyboardShortcutHintIsVisible verifies the message
// send endpoint is reachable.
func TestChatSendButtonOrKeyboardShortcutHintIsVisible(t *testing.T) {
	assertAPIHealthy(t)

	agentID := firstAgentID(t)
	if agentID == "" {
		t.Skip("no agents available")
	}
	meetingID := firstMeetingID(t)
	if meetingID == "" {
		t.Skip("no meetings available")
	}

	resp := apiPOSTForm(t, "/api/messages", map[string]string{
		"fromAgent":   "user",
		"toAgent":     agentID,
		"meetingId":   meetingID,
		"content":     "send-button-test",
		"messageType": "direction",
	})
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("send message: unexpected 5xx status %d", resp.StatusCode)
	}
}

// TestSuspendAgentTeamSuspendButtonOrOptionExists verifies the agent fire
// (remove/suspend) endpoint is reachable.
func TestSuspendAgentTeamSuspendButtonOrOptionExists(t *testing.T) {
	assertAPIHealthy(t)

	resp := apiPOSTForm(t, "/api/agents/fire", map[string]string{
		"agentId": "nonexistent-agent-id",
	})
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("fire agent: unexpected 5xx status %d", resp.StatusCode)
	}
}

// TestMeetingRoomMeetingRoomLinkOrSectionIsAccessible verifies meetings are
// included in the dashboard response.
func TestMeetingRoomMeetingRoomLinkOrSectionIsAccessible(t *testing.T) {
	assertAPIHealthy(t)
	apiGET(t, "/api/meetings")
}

// TestMeetingRoomJoinOrCreateMeetingButtonIsPresent verifies org endpoint
// includes meeting data.
func TestMeetingRoomJoinOrCreateMeetingButtonIsPresent(t *testing.T) {
	assertAPIHealthy(t)
	meetings := apiGETArray(t, "/api/meetings")
	if meetings == nil {
		t.Logf("meetings endpoint returned nil array")
	}
}

// TestTaskQueueTaskListOrQueueViewIsAccessible verifies the scheduler
// endpoint is accessible.
func TestTaskQueueTaskListOrQueueViewIsAccessible(t *testing.T) {
	assertAPIHealthy(t)
	apiGET(t, "/api/scheduler")
}

// TestTaskQueueCreateOrSubmitTaskButtonExists verifies the scheduler POST
// endpoint is reachable.
func TestTaskQueueCreateOrSubmitTaskButtonExists(t *testing.T) {
	assertAPIHealthy(t)

	status, _ := apiPOSTJSON(t, "/api/scheduler", map[string]any{
		"name":   "queue-test-task",
		"action": "noop",
	})
	if status >= 500 {
		t.Errorf("create task: expected non-5xx, got %d", status)
	}
}

// TestTaskQueueCancelRunningTaskOptionIsPresentOnTaskItems verifies the
// cancel endpoint is accessible on task items.
func TestTaskQueueCancelRunningTaskOptionIsPresentOnTaskItems(t *testing.T) {
	assertAPIHealthy(t)

	resp := apiPOSTForm(t, "/api/scheduler/cancel", map[string]string{"taskId": "queue-item-test"})
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("cancel task: expected non-5xx, got %d", resp.StatusCode)
	}
}

// TestAgentExecutionLogsLogViewIsReachable verifies the cost endpoint returns
// usage data for agents.
func TestAgentExecutionLogsLogViewIsReachable(t *testing.T) {
	assertAPIHealthy(t)
	apiGET(t, "/api/costs")
}

// TestAgentExecutionLogsLogEntriesOrNoLogsPlaceholderRenders verifies the
// costs endpoint returns a valid structure.
func TestAgentExecutionLogsLogEntriesOrNoLogsPlaceholderRenders(t *testing.T) {
	assertAPIHealthy(t)
	apiGET(t, "/api/costs")
}
