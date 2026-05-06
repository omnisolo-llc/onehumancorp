package statesyncmcp

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"onehumancorp/srcs/server/orchestration"
	"time"
)

type DefaultProvider struct {
	localDB orchestration.TaskStore
	coreURL string
	client  *http.Client
}

func NewDefaultProvider(localDB orchestration.TaskStore, coreURL string) *DefaultProvider {
	return &DefaultProvider{
		localDB: localDB,
		coreURL: coreURL,
		client:  &http.Client{Timeout: 10 * time.Second},
	}
}

func (p *DefaultProvider) SyncUp(ctx context.Context, orgID string) (int, error) {
	tasks, err := p.localDB.GetTasksByOrganization(ctx, orgID)
	if err != nil {
		return 0, fmt.Errorf("failed to get local tasks: %w", err)
	}

	var batch []*orchestration.SharedTask
	for _, task := range tasks {
		batch = append(batch, task)
	}

	if len(batch) == 0 {
		return 0, nil
	}

	payload, err := json.Marshal(batch)
	if err != nil {
		return 0, fmt.Errorf("failed to marshal batch: %w", err)
	}

	req, err := http.NewRequestWithContext(ctx, "POST", p.coreURL+"/api/v1/sync/up/bulk", bytes.NewBuffer(payload))
	if err != nil {
		return 0, err
	}
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("X-Organization-ID", orgID)

	resp, err := p.client.Do(req)
	if err != nil {
		return 0, err
	}
	defer resp.Body.Close()

	if resp.StatusCode == http.StatusOK {
		return len(batch), nil
	}

	return 0, fmt.Errorf("bulk sync up failed with status: %d", resp.StatusCode)
}

func (p *DefaultProvider) SyncDown(ctx context.Context, orgID string) (int, error) {
	req, err := http.NewRequestWithContext(ctx, "GET", p.coreURL+"/api/v1/sync/down", nil)
	if err != nil {
		return 0, err
	}
	req.Header.Set("X-Organization-ID", orgID)

	resp, err := p.client.Do(req)
	if err != nil {
		return 0, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return 0, fmt.Errorf("failed to fetch cloud tasks, status: %d", resp.StatusCode)
	}

	var tasks []*orchestration.SharedTask
	if err := json.NewDecoder(resp.Body).Decode(&tasks); err != nil {
		return 0, err
	}

	syncedCount := 0
	for _, task := range tasks {
		existing, err := p.localDB.GetTask(ctx, task.ID)
		if err == nil && existing != nil {
			if existing.UpdatedAt.Before(task.UpdatedAt) && existing.Status != task.Status {
				if updateErr := p.localDB.UpdateTaskStatus(ctx, task.ID, task.Status); updateErr == nil {
					syncedCount++
				}
			}
		} else {
			if createErr := p.localDB.CreateTask(ctx, task); createErr == nil {
				syncedCount++
			}
		}
	}

	return syncedCount, nil
}

func (p *DefaultProvider) GetStatus(ctx context.Context, orgID string) (*SyncStatus, error) {
	req, err := http.NewRequestWithContext(ctx, "GET", p.coreURL+"/api/v1/sync/status", nil)
	if err != nil {
		return nil, err
	}
	req.Header.Set("X-Organization-ID", orgID)

	resp, err := p.client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("failed to get sync status, status: %d", resp.StatusCode)
	}

	var status SyncStatus
	if err := json.NewDecoder(resp.Body).Decode(&status); err != nil {
		return nil, err
	}

	return &status, nil
}

type NoOpProvider struct{}

func NewNoOpProvider() *NoOpProvider {
	return &NoOpProvider{}
}

func (p *NoOpProvider) SyncUp(ctx context.Context, orgID string) (int, error) {
	return 0, nil
}

func (p *NoOpProvider) SyncDown(ctx context.Context, orgID string) (int, error) {
	return 0, nil
}

func (p *NoOpProvider) GetStatus(ctx context.Context, orgID string) (*SyncStatus, error) {
	return &SyncStatus{
		LastSyncTime: time.Now().Format(time.RFC3339),
		PendingTasks: 0,
		Status:       "cloud_native",
	}, nil
}
