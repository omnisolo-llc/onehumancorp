package onboarding

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
	"sync"

	"onehumancorp/srcs/server/orchestration"
)

type OnboardingRequest struct {
	Name        string `json:"name"`
	Category    string `json:"category"`
	Description string `json:"description"`
}

type ChatOnboardingRequest struct {
	Message string `json:"message"`
}

type ChatOnboardingResponse struct {
	Name        string `json:"name"`
	Category    string `json:"category"`
	Description string `json:"description"`
}

type OnboardingResponse struct {
	TenantID string `json:"tenant_id"`
	Status   string `json:"status"`
}

type TenantStateRequest struct {
	State string `json:"state"`
}

type TenantStateResponse struct {
	State string `json:"state"`
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
func (s *Service) ChatOnboarding(ctx context.Context, req ChatOnboardingRequest) (*ChatOnboardingResponse, error) {
	// A mock implementation of AI processing the user's business description.
	// In production, this would call out to Gemini/GPT.

	// Default extraction
	name := "My New Business"
	category := "Other"
	description := "A business starting out on OHC."

	// Simple keyword extraction for the sake of the E2E simulation
	if req.Message != "" {
		description = req.Message

		// Very basic heuristics for demo purposes
		if len(req.Message) > 10 {
			name = "Auto-Generated Shop"
		}

		if strings.Contains(strings.ToLower(req.Message), "bake") || strings.Contains(strings.ToLower(req.Message), "cake") {
			category = "Food & Beverage"
			name = "Custom Bakery"
		} else if strings.Contains(strings.ToLower(req.Message), "handyman") || strings.Contains(strings.ToLower(req.Message), "plumb") {
			category = "Service"
			name = "Handyman Services"
		} else if strings.Contains(strings.ToLower(req.Message), "boutique") || strings.Contains(strings.ToLower(req.Message), "clothes") {
			category = "Retail"
			name = "Fashion Boutique"
		} else if strings.Contains(strings.ToLower(req.Message), "tutor") || strings.Contains(strings.ToLower(req.Message), "music") {
			category = "Service"
			name = "Music Lessons"
		}
	}

	return &ChatOnboardingResponse{
		Name:        name,
		Category:    category,
		Description: description,
	}, nil
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

	var wg sync.WaitGroup
	errs := make(chan error, len(tasks))

	for _, taskData := range tasks {
		wg.Add(1)
		go func(tTitle, tDesc string) {
			defer wg.Done()
			task := &orchestration.SharedTask{
				OrganizationID: tenant.ID,
				Title:          tTitle,
				Description:    &tDesc,
				Status:         "PENDING",
				Priority:       "P0",
				Payload:        &rawPayload,
			}
			if err := s.taskStore.CreateTask(ctx, task); err != nil {
				errs <- fmt.Errorf("failed to dispatch task %s: %w", tTitle, err)
			}
		}(taskData.Title, taskData.Description)
	}

	wg.Wait()
	close(errs)

	for err := range errs {
		if err != nil {
			return nil, err
		}
	}

	return &OnboardingResponse{
		TenantID: tenant.ID,
		Status:   tenant.Status,
	}, nil
}

func (s *Service) SaveTenantState(ctx context.Context, tenantID string, state string) error {
	return s.tenantStore.UpdateTenantState(ctx, tenantID, state)
}

func (s *Service) GetTenantState(ctx context.Context, tenantID string) (*TenantStateResponse, error) {
	tenant, err := s.tenantStore.GetTenant(ctx, tenantID)
	if err != nil {
		return nil, fmt.Errorf("failed to get tenant: %w", err)
	}

	return &TenantStateResponse{
		State: tenant.State,
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
