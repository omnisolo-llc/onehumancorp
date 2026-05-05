package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"github.com/google/uuid"
)

type AgentPool struct {
	Store TaskStore
}

func NewAgentPool(store TaskStore) *AgentPool {
	return &AgentPool{Store: store}
}

// ForkAgent spawns a new Agent instance in the background by copying the parent's context and creating a new task.
func (p *AgentPool) ForkAgent(ctx context.Context, parentID string, directive string) (string, error) {
	// Retrieve parent's state
	parentTask, err := p.Store.GetTask(ctx, parentID)
	if err != nil {
		return "", fmt.Errorf("failed to retrieve parent agent state: %w", err)
	}

	childID := uuid.New().String()

	// Clone the state and initialize a new child agent record
	// The prompt specifies: "feed the serialized context directly into the child agent's initialization state"
	// We will create a new task that represents the child agent

	// Create a child payload, copying from parent and appending the directive or similar.
	// For simplicity, we just copy the parent's payload entirely as the "state snapshot"
	var childPayload *json.RawMessage
	if parentTask.Payload != nil {
		copiedPayload := make(json.RawMessage, len(*parentTask.Payload))
		copy(copiedPayload, *parentTask.Payload)
		childPayload = &copiedPayload
	}

	childTask := &SharedTask{
		ID:             childID,
		OrganizationID: parentTask.OrganizationID,
		Title:          "Forked Subagent: " + directive,
		Description:    parentTask.Description, // Inherit description if any
		Status:         "PENDING",
		Priority:       parentTask.Priority,
		Payload:        childPayload,
		ParentPlanID:   &parentID, // We could use ParentPlanID to track the parent
	}

	// Implement a `<task-notification>` XML/JSON response pattern for the child
	// to report progress back to the parent asynchronously.
	notificationPayload := map[string]interface{}{
		"type":    "task-notification",
		"childID": childID,
		"status":  "forked",
		"message": "Subagent started successfully",
	}
	notificationBytes, _ := json.Marshal(notificationPayload)
	var parentPayload map[string]interface{}
	if parentTask.Payload != nil {
		json.Unmarshal(*parentTask.Payload, &parentPayload)
	} else {
		parentPayload = make(map[string]interface{})
	}

	// Add the task notification to the parent's context/memory
	// In a real implementation this would use a messaging queue or update the parent state directly.
	// Here we update the parent's payload to simulate context notification.
	parentPayload["latest_notification"] = json.RawMessage(notificationBytes)
	newParentBytes, _ := json.Marshal(parentPayload)
	newParentRaw := json.RawMessage(newParentBytes)
	parentTask.Payload = &newParentRaw
	// Note: We'd typically persist the updated parent task here, or send a pubsub message.

	err = p.Store.CreateTask(ctx, childTask)
	if err != nil {
		return "", fmt.Errorf("failed to create child agent task: %w", err)
	}

	return childID, nil
}
