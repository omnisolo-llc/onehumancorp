package onboarding

import (
	"context"
	"encoding/json"
	"fmt"

	"onehumancorp/srcs/server/orchestration"
)

type OnboardingRequest struct {
	Name        string `json:"name"`
	Category    string `json:"category"`
	Description string `json:"description"`
}

type OnboardingResponse struct {
	TenantID string `json:"tenant_id"`
	Status   string `json:"status"`
}

type Service struct {
	tenantStore TenantStore
	taskStore   orchestration.TaskStore
}

func NewService(tenantStore TenantStore, taskStore orchestration.TaskStore) *Service {
	return &Service{
		tenantStore: tenantStore,
		taskStore:   taskStore,
	}
}

func (s *Service) StartOnboarding(ctx context.Context, req OnboardingRequest) (*OnboardingResponse, error) {
	tenant := &Tenant{
		Name:        req.Name,
		Category:    req.Category,
		Description: req.Description,
		Status:      "PROVISIONING",
	}

	if err := s.tenantStore.CreateTenant(ctx, tenant); err != nil {
		return nil, fmt.Errorf("failed to create tenant: %w", err)
	}

	// Dispatch tasks to the Teammate Mesh
	tasks := []struct {
		Title       string
		Description string
	}{
		{"Generate Storefront", "Marketing agent to create storefront layout and copy based on business details."},
		{"Setup Base Inventory/Calendar", "Operations agent to set up initial product catalog or booking calendar."},
		{"Generate Standard Policies", "Legal agent to draft terms of service and privacy policies."},
	}

	payload, _ := json.Marshal(req)
	rawPayload := json.RawMessage(payload)

	for _, taskData := range tasks {
		task := &orchestration.SharedTask{
			OrganizationID: tenant.ID,
			Title:          taskData.Title,
			Description:    &taskData.Description,
			Status:         "PENDING",
			Priority:       "P0",
			Payload:        &rawPayload,
		}
		if err := s.taskStore.CreateTask(ctx, task); err != nil {
			return nil, fmt.Errorf("failed to dispatch task %s: %w", taskData.Title, err)
		}
	}

	return &OnboardingResponse{
		TenantID: tenant.ID,
		Status:   tenant.Status,
	}, nil
}

func (s *Service) GetOnboardingStatus(ctx context.Context, tenantID string) (*OnboardingResponse, error) {
	tenant, err := s.tenantStore.GetTenant(ctx, tenantID)
	if err != nil {
		return nil, fmt.Errorf("failed to get tenant: %w", err)
	}

	if tenant.Status == "READY" {
		return &OnboardingResponse{TenantID: tenant.ID, Status: tenant.Status}, nil
	}

	tasks, err := s.taskStore.GetTasksByOrganization(ctx, tenantID)
	if err != nil {
		return nil, fmt.Errorf("failed to get tasks: %w", err)
	}

	allCompleted := true
	for _, task := range tasks {
		if task.Status != "COMPLETED" {
			allCompleted = false
			break
		}
	}

	if allCompleted && len(tasks) > 0 {
		if err := s.tenantStore.UpdateTenantStatus(ctx, tenantID, "READY"); err != nil {
			return nil, fmt.Errorf("failed to update tenant status: %w", err)
		}
		tenant.Status = "READY"
	}

	return &OnboardingResponse{
		TenantID: tenant.ID,
		Status:   tenant.Status,
	}, nil
}
