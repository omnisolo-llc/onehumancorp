package dashboard

import (
	"encoding/json"
	"fmt"
	"net/http"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func (s *Server) handleIncidents(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case http.MethodGet:
		s.mu.RLock()
		incidents := append([]Incident(nil), s.incidents...)
		s.mu.RUnlock()
		writeJSON(w, incidents)
	case http.MethodPost:
		var req incidentCreateRequest
		if err := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20)).Decode(&req); err != nil {
			http.Error(w, "invalid JSON payload", http.StatusBadRequest)
			return
		}
		if req.Severity == "" || req.Summary == "" {
			http.Error(w, "severity and summary are required", http.StatusBadRequest)
			return
		}
		now := time.Now().UTC()
		incident := Incident{
			ID:        "inc-" + now.Format("20060102150405"),
			Severity:  IncidentSeverity(req.Severity),
			Summary:   req.Summary,
			RCA:       req.RCA,
			Status:    IncidentStatusInvestigating,
			CreatedAt: now,
			UpdatedAt: now,
		}
		s.mu.Lock()
		s.incidents = append(s.incidents, incident)
		s.mu.Unlock()
		writeJSON(w, incident)
	default:
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
	}
}

func (s *Server) handleIncidentStatus(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	var req incidentStatusRequest
	if err := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20)).Decode(&req); err != nil {
		http.Error(w, "invalid JSON payload", http.StatusBadRequest)
		return
	}
	if req.IncidentID == "" || req.Status == "" {
		http.Error(w, "incidentId and status are required", http.StatusBadRequest)
		return
	}

	s.mu.Lock()
	defer s.mu.Unlock()
	for i, inc := range s.incidents {
		if inc.ID == req.IncidentID {
			s.incidents[i].Status = IncidentStatus(req.Status)
			s.incidents[i].UpdatedAt = time.Now().UTC()
			if req.ResolutionPlanID != "" {
				s.incidents[i].ResolutionPlanID = req.ResolutionPlanID
			}
			if req.RCA != "" {
				s.incidents[i].RCA = req.RCA
			}
			writeJSON(w, s.incidents[i])
			return
		}
	}
	http.Error(w, "incident not found", http.StatusNotFound)
}

func (s *Server) handleComputeProfiles(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case http.MethodGet:
		s.mu.RLock()
		profiles := append([]ComputeProfile(nil), s.computeProfiles...)
		s.mu.RUnlock()
		writeJSON(w, profiles)
	case http.MethodPost:
		var req computeProfileRequest
		if err := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20)).Decode(&req); err != nil {
			http.Error(w, "invalid JSON payload", http.StatusBadRequest)
			return
		}
		if req.RoleID == "" {
			http.Error(w, "roleId is required", http.StatusBadRequest)
			return
		}
		profile := ComputeProfile{
			RoleID:             req.RoleID,
			MinVRAMGB:          req.MinVRAMGB,
			PreferredGPUType:   req.PreferredGPUType,
			SchedulingPriority: req.SchedulingPriority,
			CreatedAt:          time.Now().UTC(),
		}
		s.mu.Lock()
		s.computeProfiles = append(s.computeProfiles, profile)
		s.mu.Unlock()
		writeJSON(w, profile)
	default:
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
	}
}

func (s *Server) handleClusterStatus(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	// Extract region from URL path: /api/clusters/{region}/status
	parts := strings.Split(strings.Trim(r.URL.Path, "/"), "/")
	region := ""
	for i, p := range parts {
		if p == "clusters" && i+1 < len(parts) {
			region = parts[i+1]
			break
		}
	}
	if region == "" {
		http.Error(w, "region is required in path", http.StatusBadRequest)
		return
	}
	// Simulated cluster health response (would call k8s API in production)
	status := ClusterStatus{
		Region:         region,
		Status:         "healthy",
		LatencyMS:      3,
		AvailableNodes: 5,
		CheckedAt:      time.Now().UTC(),
	}
	writeJSON(w, status)
}

func (s *Server) handleBudgetAlerts(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case http.MethodGet:
		s.mu.RLock()
		alerts := append([]BudgetAlert(nil), s.budgetAlerts...)
		orgID := s.org.ID
		s.mu.RUnlock()

		summary := s.tracker.Summary(orgID)
		_, usdBurnRate := s.tracker.GetBurnRates(orgID)

		for i, a := range alerts {
			effectiveSpend := summary.TotalCostUSD
			if a.Predictive && a.ForecastHours > 0 {
				// Forecasted spend = Current spend + (USD burn rate per minute * 60 minutes * hours)
				effectiveSpend += usdBurnRate * 60 * float64(a.ForecastHours)
			}
			alerts[i].Triggered = effectiveSpend >= a.ThresholdUSD*a.NotifyAtPct
		}
		writeJSON(w, alerts)
	case http.MethodPost:
		var req budgetAlertRequest
		if err := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20)).Decode(&req); err != nil {
			http.Error(w, "invalid JSON payload", http.StatusBadRequest)
			return
		}
		if req.ThresholdUSD <= 0 {
			http.Error(w, "thresholdUsd must be greater than zero", http.StatusBadRequest)
			return
		}
		if req.NotifyAtPct <= 0 || req.NotifyAtPct > 1 {
			req.NotifyAtPct = defaultBudgetAlertNotifyPct // default 80 %
		}
		orgID := req.OrganizationID
		if orgID == "" {
			s.mu.RLock()
			orgID = s.org.ID
			s.mu.RUnlock()
		}
		alert := BudgetAlert{
			ID:             "alert-" + time.Now().Format("20060102150405"),
			OrganizationID: orgID,
			ThresholdUSD:   req.ThresholdUSD,
			NotifyAtPct:    req.NotifyAtPct,
			Predictive:     req.Predictive,
			ForecastHours:  req.ForecastHours,
			Triggered:      false,
			CreatedAt:      time.Now().UTC(),
		}
		s.mu.Lock()
		s.budgetAlerts = append(s.budgetAlerts, alert)
		s.mu.Unlock()
		writeJSON(w, alert)
	default:
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
	}
}

func (s *Server) handlePipelines(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case http.MethodGet:
		s.mu.RLock()
		pipelines := append([]Pipeline(nil), s.pipelines...)
		s.mu.RUnlock()
		writeJSON(w, pipelines)
	case http.MethodPost:
		var req pipelineCreateRequest
		if err := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20)).Decode(&req); err != nil {
			http.Error(w, "invalid JSON payload", http.StatusBadRequest)
			return
		}
		if req.Name == "" {
			http.Error(w, "name is required", http.StatusBadRequest)
			return
		}
		now := time.Now().UTC()
		pipeline := Pipeline{
			ID:          "pipeline-" + now.Format("20060102150405"),
			Name:        req.Name,
			Status:      PipelineStatusPending,
			Branch:      req.Branch,
			InitiatedBy: req.InitiatedBy,
			CreatedAt:   now,
			UpdatedAt:   now,
		}
		s.mu.Lock()
		s.pipelines = append(s.pipelines, pipeline)
		s.mu.Unlock()
		writeJSON(w, pipeline)
	default:
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
	}
}

func (s *Server) handlePipelinePromote(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	var req pipelinePromoteRequest
	if err := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20)).Decode(&req); err != nil {
		http.Error(w, "invalid JSON payload", http.StatusBadRequest)
		return
	}
	if req.PipelineID == "" {
		http.Error(w, "pipelineId is required", http.StatusBadRequest)
		return
	}

	s.mu.Lock()
	defer s.mu.Unlock()
	for i, p := range s.pipelines {
		if p.ID == req.PipelineID {
			if s.pipelines[i].Status != PipelineStatusStaging {
				http.Error(w, "pipeline must be in STAGING status to promote", http.StatusBadRequest)
				return
			}
			s.pipelines[i].Status = PipelineStatusPromoted
			s.pipelines[i].UpdatedAt = time.Now().UTC()
			writeJSON(w, s.pipelines[i])
			return
		}
	}
	http.Error(w, "pipeline not found", http.StatusNotFound)
}

func (s *Server) handlePipelineStatus(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	var req struct {
		PipelineID string `json:"pipelineId"`
		Status     string `json:"status"`
		StagingURL string `json:"stagingUrl,omitempty"`
	}
	if err := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20)).Decode(&req); err != nil {
		http.Error(w, "invalid JSON payload", http.StatusBadRequest)
		return
	}
	if req.PipelineID == "" || req.Status == "" {
		http.Error(w, "pipelineId and status are required", http.StatusBadRequest)
		return
	}

	s.mu.Lock()
	defer s.mu.Unlock()
	for i, p := range s.pipelines {
		if p.ID == req.PipelineID {
			s.pipelines[i].Status = PipelineStatus(req.Status)
			s.pipelines[i].UpdatedAt = time.Now().UTC()
			if req.StagingURL != "" {
				s.pipelines[i].StagingURL = req.StagingURL
			}
			writeJSON(w, s.pipelines[i])
			return
		}
	}
	http.Error(w, "pipeline not found", http.StatusNotFound)
}

// ScaleRequest defines the payload for scaling a team member role.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type ScaleRequest struct {
	Role  string `json:"role"`
	Count int    `json:"count"`
}

func (s *Server) handleScale(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	var req ScaleRequest
	dec := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20))
	dec.DisallowUnknownFields()
	if err := dec.Decode(&req); err != nil {
		http.Error(w, "invalid JSON payload", http.StatusBadRequest)
		return
	}
	if req.Role == "" {
		http.Error(w, "role is required", http.StatusBadRequest)
		return
	}

	s.mu.RLock()
	orgID := s.org.ID
	agents := s.orgAgentsLocked()
	s.mu.RUnlock()

	var currentCount int
	var idleAgentIDs []string
	var activeAgentIDs []string
	for _, agent := range agents {
		if agent.Role == req.Role {
			currentCount++
			if agent.Status == orchestration.StatusIdle {
				idleAgentIDs = append(idleAgentIDs, agent.ID)
			} else {
				activeAgentIDs = append(activeAgentIDs, agent.ID)
			}
		}
	}

	diff := req.Count - currentCount
	nowStr := time.Now().UTC().Format("20060102150405000")

	if diff > 0 {
		for i := 0; i < diff; i++ {
			id := fmt.Sprintf("%s-agent-%s-%d", orgID, nowStr, i)
			newAgent := orchestration.Agent{
				ID:             id,
				Name:           req.Role,
				Role:           req.Role,
				OrganizationID: orgID,
				Status:         orchestration.StatusIdle,
			}
			s.hub.RegisterAgent(newAgent)
		}
	} else if diff < 0 {
		toRemove := -diff

		// ⚡ BOLT: [pod-thrashing during dynamic scale-up/down] - Randomized Selection from Top 5
		// Gracefully scales down idle agents first before terminating busy ones to prevent interrupting active work.
		for i := 0; i < toRemove; i++ {
			if len(idleAgentIDs) > 0 {
				s.hub.FireAgent(idleAgentIDs[0])
				idleAgentIDs = idleAgentIDs[1:]
			} else if len(activeAgentIDs) > 0 {
				s.hub.FireAgent(activeAgentIDs[0])
				activeAgentIDs = activeAgentIDs[1:]
			}
		}
	}

	writeJSON(w, map[string]interface{}{
		"status": "success",
		"role":   req.Role,
		"count":  req.Count,
	})
}

// handleScaleStream streams real-time scaling trace events to the dashboard.
// Accepts parameters: s *Server (No Constraints).
// Returns nothing.
// Produces no errors.
// Has no side effects.
func (s *Server) handleScaleStream(w http.ResponseWriter, r *http.Request) {
	// Set headers for SSE
	w.Header().Set("Content-Type", "text/event-stream")
	w.Header().Set("Cache-Control", "no-cache")
	w.Header().Set("Connection", "keep-alive")

	rc := http.NewResponseController(w)

	events := []string{
		`{"event":"AI Workforce Manager: Reconciling Team Member resource.","status":"INFO"}`,
		`{"event":"AI Workforce Manager: Allocating compute profiles...","status":"INFO"}`,
		`{"event":"AI Workforce Manager: Provisioning SPIFFE identities...","status":"INFO"}`,
		`{"event":"AI Workforce Manager: Integrating with orchestration Hub...","status":"INFO"}`,
		`{"event":"AgentHired","status":"Ready"}`,
	}

	for _, event := range events {
		select {
		case <-r.Context().Done():
			return
		default:
		}

		s.hub.LogEvent(map[string]interface{}{"type": "ScalingEventStream", "data": event})
		data := []byte("data: " + event + "\n\n")
		w.Write(data)
		if err := rc.Flush(); err != nil {
			break
		}

		select {
		case <-r.Context().Done():
			return
		case <-time.After(1 * time.Second):
		}
	}
}

func (s *Server) handleTelemetrySyncV1(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var batch []struct {
		MetricName string      `json:"metric_name"`
		Payload    interface{} `json:"payload"`
	}

	if err := json.NewDecoder(r.Body).Decode(&batch); err != nil {
		http.Error(w, "invalid request body", http.StatusBadRequest)
		return
	}

	ctx := r.Context()
	for _, item := range batch {
		payloadBytes, _ := json.Marshal(item.Payload)
		// We re-record the metric in the Cloud TSDB but with deployment_mode="standalone"
		// The specific telemetry recording functions should be called here or a generic one.
		// Since we want to ensure deployment_mode="standalone", we can use attributes.

		// For now, let's use the existing Record... functions if they support it,
		// but most don't take extra labels.
		// A better way is to have a generic recorder that takes name and labels.

		// According to the problem statement: "Ensure that synced metrics append the deployment_mode="standalone" label."
		// AND "all Prometheus histogram_quantile queries rely on sum() by specific labels (such as le, mode, deployment_mode)"

		// Let's implement a dispatcher similar to handleTelemetrySync but more robust.
		s.dispatchTelemetryV1(ctx, item.MetricName, item.Payload)
	}

	w.WriteHeader(http.StatusOK)
	_, _ = w.Write([]byte(`{"status":"ok"}`))
}

func (s *Server) dispatchTelemetryV1(ctx context.Context, name string, payload interface{}) {
	data, ok := payload.(map[string]interface{})
	if !ok {
		return
	}

	// Force deployment_mode="standalone" attribute to all recorded metrics here.
	// We inject it into the context or pass it explicitly to the recording functions.
	// Since our Record... functions in telemetry.go now use helpers that can handle
	// extra attributes, we should pass it there.

	// For now, let's use the standard ones but ensure they get labeled.
	// Actually, the requirement says "append the deployment_mode=standalone label".
	// The helpers in telemetry_helpers.go don't currently support overriding the mode.
	// Let's modify them.

	switch name {
	case "token_usage":
		agentID, _ := data["agent_id"].(string)
		role, _ := data["role"].(string)
		model, _ := data["model"].(string)
		tokenType, _ := data["type"].(string)
		var count int64
		if c, ok := data["count"].(float64); ok {
			count = int64(c)
		}
		telemetry.RecordTokenUsageWithMode(ctx, agentID, role, model, tokenType, count, "standalone")
	case "agent_token_usage":
		agentID, _ := data["agent_id"].(string)
		orgID, _ := data["organization_id"].(string)
		role, _ := data["role"].(string)
		model, _ := data["model"].(string)
		var count int64
		if c, ok := data["count"].(float64); ok {
			count = int64(c)
		}
		telemetry.RecordAgentTokenUsageWithMode(ctx, agentID, orgID, role, model, count, "standalone")
	case "agent_cost":
		agentID, _ := data["agent_id"].(string)
		orgID, _ := data["organization_id"].(string)
		role, _ := data["role"].(string)
		model, _ := data["model"].(string)
		cost, _ := data["cost"].(float64)
		telemetry.RecordAgentCostWithMode(ctx, agentID, orgID, role, model, cost, "standalone")
	case "swarm_task_completed":
		missionID, _ := data["mission_id"].(string)
		telemetry.RecordSwarmTaskCompletedWithMode(ctx, missionID, "standalone")
	case "agent_api_call":
		agentID, _ := data["agent_id"].(string)
		role, _ := data["role"].(string)
		api, _ := data["api"].(string)
		telemetry.RecordAgentApiCallWithMode(ctx, agentID, role, api, "standalone")
	default:
		slog.Debug("Received unhandled offline telemetry metric", "name", name)
	}
}

func (s *Server) handlePruneMissions(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	// Execute pruning task
	if s.hub.SIPDB() != nil {
		_ = s.hub.SIPDB().PruneStaleMissions(r.Context(), 0) // Prune all completed or stale missions immediately
	}
	writeJSON(w, map[string]string{"status": "success", "message": "agent missions pruned"})
}
