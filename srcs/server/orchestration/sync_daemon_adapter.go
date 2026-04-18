package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"bytes"
	"net/http"
	"os"
	"time"
	"io"
	"sync"

	ohc_sync "github.com/onehumancorp/mono/srcs/server/orchestration/sync"
)

type CloudSyncPayload struct {
	LocalID string                 `json:"local_id"`
	Payload map[string]interface{} `json:"payload"`
}

type CloudSyncResponse struct {
	CloudID string `json:"cloud_id"`
	Status  string `json:"status"`
}

type CloudStatusResponse struct {
	CloudID string `json:"cloud_id"`
	Status  string `json:"status"`
	Result  string `json:"result,omitempty"`
}

// PushPendingMissions finds tasks marked for escalation and sends them to the cloud
func (d *HybridMCPRAGDaemon) PushPendingMissions(ctx context.Context) error {
	repo := ohc_sync.NewSqliteLocalRepository(d.dbWrapper)
	missions, err := repo.GetPendingSync(ctx, 500)
	if err != nil {
		slog.Error("sync_daemon: failed to get pending sync", "error", err)
		return err
	}

	if len(missions) == 0 {
		return nil
	}

	var wg sync.WaitGroup
	semaphore := make(chan struct{}, 10)

	for _, m := range missions {
		wg.Add(1)
		go func(m ohc_sync.LocalMission) {
			defer wg.Done()
			semaphore <- struct{}{}
			defer func() { <-semaphore }()

			var rawPayload map[string]interface{}
			if err := json.Unmarshal([]byte(m.PayloadRaw), &rawPayload); err != nil {
				repo.MarkSyncError(ctx, m.ID, "failed to unmarshal payload")
				return
			}

			// Sanitize
			parsedIface := SanitizePayloadMap(rawPayload)
			redactedMap, ok := parsedIface.(map[string]interface{})
			if !ok {
				repo.MarkSyncError(ctx, m.ID, "failed to sanitize payload map")
				return
			}

			if ctxStr, ok := redactedMap["context"].(string); ok {
				sanitizedCtx, _ := SanitizePayload(ctxStr)
				redactedMap["context"] = sanitizedCtx
			}

			payload := CloudSyncPayload{
				LocalID: m.ID,
				Payload: redactedMap,
			}

			jsonData, err := json.Marshal(payload)
			if err != nil {
				repo.MarkSyncError(ctx, m.ID, "failed to marshal payload")
				return
			}

			syncEndpoint := fmt.Sprintf("%s/api/v1/missions/escalate", d.cloudAPIURL)
			req, err := http.NewRequestWithContext(ctx, http.MethodPost, syncEndpoint, bytes.NewBuffer(jsonData))
			if err != nil {
				repo.MarkSyncError(ctx, m.ID, err.Error())
				return
			}
			req.Header.Set("Content-Type", "application/json")

			if spiffeToken := os.Getenv("SPIFFE_IDENTITY_TOKEN"); spiffeToken != "" {
				req.Header.Set("Authorization", "Bearer "+spiffeToken)
			}

			client := &http.Client{Timeout: 10 * time.Second}
			resp, err := client.Do(req)
			if err != nil {
				repo.MarkSyncError(ctx, m.ID, err.Error())
				return
			}

			body, _ := io.ReadAll(resp.Body)
			resp.Body.Close()

			if resp.StatusCode >= 300 {
				repo.MarkSyncError(ctx, m.ID, fmt.Sprintf("status %d: %s", resp.StatusCode, string(body)))
				return
			}

			var syncResp CloudSyncResponse
			if err := json.Unmarshal(body, &syncResp); err != nil {
				repo.MarkSyncError(ctx, m.ID, "failed to decode response")
				return
			}

			if syncResp.Status == "ACCEPTED" {
				repo.MarkSynced(ctx, m.ID, syncResp.CloudID)
			}
		}(m)
	}

	wg.Wait()
	return nil
}

// PullMissionUpdates polls the cloud for updates to previously escalated tasks
func (d *HybridMCPRAGDaemon) PullMissionUpdates(ctx context.Context) error {
	repo := ohc_sync.NewSqliteLocalRepository(d.dbWrapper)
	activeMissions, err := repo.GetActiveEscalations(ctx)
	if err != nil {
		slog.Error("sync_daemon: failed to get active escalations", "error", err)
		return err
	}

	var wg sync.WaitGroup
	semaphore := make(chan struct{}, 10)

	for _, m := range activeMissions {
		wg.Add(1)
		go func(m ohc_sync.LocalMission) {
			defer wg.Done()
			semaphore <- struct{}{}
			defer func() { <-semaphore }()

			syncEndpoint := fmt.Sprintf("%s/api/v1/missions/%s/status", d.cloudAPIURL, m.CloudMissionID)
			req, err := http.NewRequestWithContext(ctx, http.MethodGet, syncEndpoint, nil)
			if err != nil {
				return
			}

			if spiffeToken := os.Getenv("SPIFFE_IDENTITY_TOKEN"); spiffeToken != "" {
				req.Header.Set("Authorization", "Bearer "+spiffeToken)
			}

			client := &http.Client{Timeout: 10 * time.Second}
			resp, err := client.Do(req)
			if err != nil {
				return
			}

			body, _ := io.ReadAll(resp.Body)
			resp.Body.Close()

			if resp.StatusCode == 200 {
				var statusResp CloudStatusResponse
				if err := json.Unmarshal(body, &statusResp); err == nil {
					if statusResp.Status == "DONE" || statusResp.Status == "COMPLETED" || statusResp.Status == "FAILED" {
						repo.UpdateLocalStatus(ctx, m.ID, statusResp.Status)
					}
				}
			}
		}(m)
	}

	wg.Wait()
	return nil
}
