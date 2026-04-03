package dashboard

import (
	"encoding/json"
	"errors"
	"fmt"
	"log/slog"
	"net/http"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/integrations"
	"github.com/onehumancorp/mono/srcs/server/interop"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel"
)

func (s *Server) handleMCPRegister(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	r.Body = http.MaxBytesReader(w, r.Body, 1<<20)

	var req mcpRegisterRequest
	dec := json.NewDecoder(r.Body)
	dec.DisallowUnknownFields()
	if err := dec.Decode(&req); err != nil {
		http.Error(w, "invalid JSON payload", http.StatusBadRequest)
		return
	}

	if req.Tool.ID == "" || req.Tool.Name == "" {
		http.Error(w, "tool ID and name are required", http.StatusBadRequest)
		return
	}

	if err := interop.ValidateSPIFFEID(req.SPIFFEID); err != nil {
		http.Error(w, "invalid SPIFFE ID: "+err.Error(), http.StatusForbidden)
		return
	}

	// Persist to SIP DB Mesh
	if s.hub.SIPDB() != nil {
		plugin := orchestration.CapabilityPlugin{
			PluginID:    req.Tool.ID,
			Name:        req.Tool.Name,
			Version:     "1.0.0", // Hardcoded for now if not provided
			ManifestURL: "internal",
			Status:      "available",
		}
		if err := s.hub.SIPDB().RegisterCapabilityPlugin(r.Context(), plugin); err != nil {
			http.Error(w, "failed to register capability plugin in mesh: "+err.Error(), http.StatusInternalServerError)
			return
		}
	}

	s.mu.Lock()
	defer s.mu.Unlock()

	// Check if tool already exists
	for i, t := range s.dynamicMCPTools {
		if t.ID == req.Tool.ID {
			s.dynamicMCPTools[i] = req.Tool
			writeJSON(w, map[string]interface{}{"status": "updated", "tool": req.Tool})
			return
		}
	}

	s.dynamicMCPTools = append(s.dynamicMCPTools, req.Tool)
	writeJSON(w, map[string]interface{}{"status": "registered", "tool": req.Tool})
}

func (s *Server) handleMCPTools(w http.ResponseWriter, _ *http.Request) {
	s.mu.RLock()
	defer s.mu.RUnlock()
	writeJSON(w, s.dynamicMCPTools)
}

func (s *Server) handleMCPInvoke(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// Enforce a strict 1MB limit on tool payloads to prevent DOS via massive JSON strings.
	r.Body = http.MaxBytesReader(w, r.Body, 1<<20)

	var req mcpInvokeRequest
	dec := json.NewDecoder(r.Body)
	dec.DisallowUnknownFields()
	if err := dec.Decode(&req); err != nil {
		http.Error(w, "invalid JSON payload", http.StatusBadRequest)
		return
	}
	if req.ToolID == "" {
		http.Error(w, "toolId is required", http.StatusBadRequest)
		return
	}
	if len(req.Params) == 0 {
		req.Params = []byte("{}")
	}
	if exists, belongs := s.agentOrgStatus(req.AgentID); exists && !belongs {
		http.Error(w, "agent does not belong to this organization", http.StatusForbidden)
		return
	}

	if err := interop.ValidateSPIFFEID(req.SPIFFEID); err != nil {
		http.Error(w, "invalid SPIFFE ID: "+err.Error(), http.StatusForbidden)
		return
	}

	// Check if the agent is rate-limited for this tool
	rateLimitKey := req.AgentID + ":" + req.ToolID
	s.mu.Lock()
	if s.rateLimitStates == nil {
		s.rateLimitStates = make(map[string]*RateLimitState)
	}
	state, exists := s.rateLimitStates[rateLimitKey]
	if !exists {
		state = &RateLimitState{Backoff: 1 * time.Second}
		s.rateLimitStates[rateLimitKey] = state
	}
	s.mu.Unlock()

	s.mu.RLock()
	if state.Failures >= 3 {
		s.mu.RUnlock()
		http.Error(w, "Max retries exceeded. Hard failure.", http.StatusTooManyRequests)
		return
	}
	if time.Since(state.LastFailure) < state.Backoff && state.Failures > 0 {
		s.mu.RUnlock()
		http.Error(w, "Rate limited. Please backoff.", http.StatusTooManyRequests)
		return
	}
	s.mu.RUnlock()

	result, err := s.invokeMCPTool(req)

	s.mu.Lock()
	if err != nil {
		// e.g. "Rate limited" or "429" or missing tool handling
		if strings.Contains(err.Error(), "429") || strings.Contains(err.Error(), "rate limited") {
			state.Failures++
			state.LastFailure = time.Now()
			state.Backoff = time.Duration(1<<state.Failures) * time.Second // Exponential backoff

			// Record failure event
			if req.AgentID != "" {
				msg := orchestration.Message{
					ID:         "rl-" + time.Now().UTC().Format("20060102150405.999999999"),
					FromAgent:  "SYSTEM",
					ToAgent:    req.AgentID,
					Type:       "ToolExecutionRateLimiting",
					Content:    fmt.Sprintf(`{"toolId": "%s", "status": "failed", "reason": "rate_limited", "backoff": "%s", "failures": %d}`, req.ToolID, state.Backoff.String(), state.Failures),
					OccurredAt: time.Now().UTC(),
				}
				_ = s.hub.Publish(msg)

				s.hub.LogEvent(msg)
			}

			s.mu.Unlock()
			if state.Failures >= 3 {
				http.Error(w, "Max retries exceeded. Hard failure.", http.StatusTooManyRequests)
			} else {
				http.Error(w, "Rate limited. Please backoff.", http.StatusTooManyRequests)
			}
			return
		} else if strings.Contains(err.Error(), "not found") || strings.Contains(err.Error(), "unknown tool") || strings.Contains(err.Error(), "invalid JSON-RPC") {
			if req.AgentID != "" {
				if agent, ok := s.hub.Agent(req.AgentID); ok {
					agent.Status = orchestration.StatusWaitingForTools
					s.hub.RegisterAgent(agent)
				}
			}
			s.mu.Unlock()
			http.Error(w, err.Error(), http.StatusNotFound)
			return
		}
	} else {
		// Reset on success
		delete(s.rateLimitStates, rateLimitKey) // Prevent unbounded memory leak

		if req.AgentID != "" {
			msg := orchestration.Message{
				ID:         "rl-succ-" + time.Now().UTC().Format("20060102150405.999999999"),
				FromAgent:  "SYSTEM",
				ToAgent:    req.AgentID,
				Type:       "ToolExecutionRateLimiting",
				Content:    fmt.Sprintf(`{"toolId": "%s", "status": "success"}`, req.ToolID),
				OccurredAt: time.Now().UTC(),
			}
			_ = s.hub.Publish(msg)

			s.hub.LogEvent(msg)
		}
	}
	s.mu.Unlock()

	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	writeJSON(w, result)
}

func (s *Server) invokeMCPTool(req mcpInvokeRequest) (map[string]any, error) {
	// Emit structured trace for MCP tool invocation
	if telemetry.Verbosity >= 2 {
		slog.Info("agent execution trace",
			"component", "telemetry",
			"api", "invokeMCPTool",
			"tool_id", req.ToolID,
			"action", req.Action,
		)
	}

	switch req.ToolID {
	// ── Communication tools ───────────────────────────────────────────────────
	case "telegram-mcp", "slack-mcp", "teams-mcp":
		var p chatToolParams
		// ⚡ BOLT: [JSON serialization thrashing on tool payloads] - Randomized Selection from Top 5
		// Eliminated json.NewDecoder allocations on hot native paths using json.Unmarshal.
		if err := json.Unmarshal(req.Params, &p); err != nil {
			return nil, errors.New("invalid chat tool parameters")
		}

		integrationID := p.IntegrationID
		if integrationID == "" {
			switch req.ToolID {
			case "telegram-mcp":
				integrationID = "telegram"
			case "slack-mcp":
				integrationID = "slack"
			case "teams-mcp":
				integrationID = "teams"
			}
		}

		channel := p.Channel
		fromAgent := p.FromAgent
		content := p.Content
		threadID := p.ThreadID

		if content == "" {
			return nil, errors.New("content is required")
		}
		if fromAgent == "" {
			fromAgent = "system"
		}
		// Fall back to the configured chatspace if no channel given.
		if channel == "" {
			if integ, ok := s.integReg.Instance(integrationID); ok {
				channel = integ.Chatspace
			}
		}
		if channel == "" {
			return nil, errors.New("channel is required — configure the integration's chatspace first")
		}
		msg, err := s.integReg.SendChatMessage(integrationID, channel, fromAgent, content, threadID, time.Now().UTC())
		if err != nil {
			return nil, err
		}
		return map[string]any{"message": msg, "delivered": true}, nil

	// ── Git tools ─────────────────────────────────────────────────────────────
	case "git-mcp":
		var p gitToolParams
		// ⚡ BOLT: [JSON serialization thrashing on tool payloads] - Randomized Selection from Top 5
		// Eliminated json.NewDecoder allocations on hot native paths using json.Unmarshal.
		if err := json.Unmarshal(req.Params, &p); err != nil {
			return nil, errors.New("invalid git tool parameters")
		}

		integrationID := p.IntegrationID
		if integrationID == "" {
			integrationID = "github"
		}

		repo := p.Repository
		title := p.Title
		body := p.Body
		source := p.SourceBranch
		target := p.TargetBranch
		createdBy := p.CreatedBy

		if target == "" {
			target = "main"
		}
		pr, err := s.integReg.CreatePullRequest(integrationID, repo, title, body, source, target, createdBy, time.Now().UTC())
		if err != nil {
			return nil, err
		}
		return map[string]any{"pullRequest": pr}, nil

	// ── Issue tracker tools ───────────────────────────────────────────────────
	case "jira-mcp", "linear-mcp":
		var p issueToolParams
		// ⚡ BOLT: [JSON serialization thrashing on tool payloads] - Randomized Selection from Top 5
		// Eliminated json.NewDecoder allocations on hot native paths using json.Unmarshal.
		if err := json.Unmarshal(req.Params, &p); err != nil {
			return nil, errors.New("invalid issue tool parameters")
		}

		integrationID := p.IntegrationID
		if integrationID == "" {
			if req.ToolID == "jira-mcp" {
				integrationID = "jira"
			} else {
				integrationID = "linear"
			}
		}

		project := p.Project
		title := p.Title
		description := p.Description
		createdBy := p.CreatedBy
		priority := p.Priority

		issue, err := s.integReg.CreateIssue(integrationID, project, title, description, createdBy,
			integrations.IssuePriority(priority), nil, time.Now().UTC())
		if err != nil {
			return nil, err
		}
		return map[string]any{"issue": issue}, nil

	// ── Unimplemented tools — return a structured acknowledgement ─────────────
	default:
		s.mu.RLock()
		found := false
		for _, t := range s.dynamicMCPTools {
			if t.ID == req.ToolID {
				found = true
				break
			}
		}
		s.mu.RUnlock()

		if !found {
			return nil, fmt.Errorf("unknown tool: %s", req.ToolID)
		}

		return map[string]any{
			"toolId":  req.ToolID,
			"status":  "invoked",
			"message": "Tool invocation recorded. Connect the corresponding service integration to enable live execution.",
		}, nil
	}
}

type AutoDreamPayload struct {
	Type     string `json:"type"` // "embedding" or "mission"
	ID       string `json:"id"`
	Data     string `json:"data"`
	Metadata string `json:"metadata"`
}

// handleSyncMissionsPayloads handles local-to-cloud Hybrid MCP RAG Protocol synchronization
func (s *Server) handleSyncMissionsPayloads(w http.ResponseWriter, r *http.Request) {
	ctx, span := otel.Tracer("github.com/onehumancorp/mono/srcs/server/dashboard").Start(r.Context(), "handleSyncMissionsPayloads")
	defer span.End()

	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	r.Body = http.MaxBytesReader(w, r.Body, 1<<20)

	var payloads []AutoDreamPayload
	if err := json.NewDecoder(r.Body).Decode(&payloads); err != nil {
		http.Error(w, "invalid JSON payload", http.StatusBadRequest)
		return
	}

	for _, payload := range payloads {
		if payload.Type != "mission" {
			continue // skip non-missions
		}

		forceLocal := r.Header.Get("X-OHC-Conflict-Resolution") == "force-local"

		err := s.hub.SIPDB().UpsertMission(ctx, payload.ID, payload.Metadata, payload.Data, forceLocal)
		if err != nil {
			slog.Error("failed to upsert synced mission", "error", err, "id", payload.ID)
		}
	}

	writeJSON(w, map[string]string{"status": "success", "message": "missions synced"})
}

// handleMissionsSync handles local-to-cloud mission synchronization via an UPSERT query
func (s *Server) handleMissionsSync(w http.ResponseWriter, r *http.Request) {
	ctx, span := otel.Tracer("github.com/onehumancorp/mono/srcs/server/dashboard").Start(r.Context(), "handleMissionsSync")
	defer span.End()

	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	r.Body = http.MaxBytesReader(w, r.Body, 1<<20)

	var payload map[string]interface{}
	if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
		http.Error(w, "invalid JSON payload", http.StatusBadRequest)
		return
	}

	idVal, ok := payload["id"]
	var missionID string
	if ok {
		missionID = fmt.Sprintf("%v", idVal)
	}

	if missionID == "" {
		http.Error(w, "missing mission id in payload", http.StatusBadRequest)
		return
	}

	statusVal, ok := payload["status"]
	var status string
	if ok {
		status = fmt.Sprintf("%v", statusVal)
	} else {
		status = "PENDING"
	}

	payloadBytes, err := json.Marshal(payload)
	if err != nil {
		http.Error(w, "failed to re-marshal payload", http.StatusInternalServerError)
		return
	}

	forceLocal := r.Header.Get("X-OHC-Conflict-Resolution") == "force-local"

	err = s.hub.SIPDB().UpsertMission(ctx, missionID, status, string(payloadBytes), forceLocal)
	if err != nil {
		slog.Error("failed to upsert mission", "error", err)
		http.Error(w, "internal server error", http.StatusInternalServerError)
		return
	}

	writeJSON(w, map[string]string{"status": "success", "message": "mission synced"})
}

// handleContextSync handles local-to-cloud Hybrid MCP RAG state synchronization
func (s *Server) handleContextSync(w http.ResponseWriter, r *http.Request) {
	ctx, span := otel.Tracer("github.com/onehumancorp/mono/srcs/server/dashboard").Start(r.Context(), "handleContextSync")
	defer span.End()

	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	r.Body = http.MaxBytesReader(w, r.Body, 1<<20)

	var payload map[string]interface{}
	if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
		http.Error(w, "invalid JSON payload", http.StatusBadRequest)
		return
	}

	// Memory says: "Safely decode the JSON payload into an interface{} and type assert to map[string]interface{}
	// (to avoid corrupting arrays or primitives) before utilizing the public telemetry.RedactPII function to strip personally identifiable information."

	// Ensure safe deep recursive redaction on string fields to prevent sensitive data leakage.
	var redactRecursive func(interface{}) interface{}
	redactRecursive = func(val interface{}) interface{} {
		switch v := val.(type) {
		case string:
			return telemetry.RedactPII(v)
		case map[string]interface{}:
			for mk, mv := range v {
				v[mk] = redactRecursive(mv)
			}
			return v
		case []interface{}:
			for i, iv := range v {
				v[i] = redactRecursive(iv)
			}
			return v
		default:
			return v
		}
	}

	for k, v := range payload {
		payload[k] = redactRecursive(v)
	}

	var memoryID string
	if idVal, ok := payload["memory_id"]; ok {
		memoryID = fmt.Sprintf("%v", idVal)
	}

	if memoryID == "" {
		// Try fallback to just generating a new memory ID if not provided, or return error
		memoryID = "ctx-" + time.Now().UTC().Format("20060102150405.999999999")
	}

	var contextStr string
	if ctxVal, ok := payload["context"]; ok {
		if strCtx, isStr := ctxVal.(string); isStr {
			contextStr = strCtx
		} else {
			marshaled, _ := json.Marshal(ctxVal)
			contextStr = string(marshaled)
		}
	} else {
		// If context isn't a direct top level key, marshal the whole sanitized payload as the context
		sanitizedBytes, _ := json.Marshal(payload)
		contextStr = string(sanitizedBytes)
	}

	var sourcePlugin string
	if srcVal, ok := payload["source_plugin"]; ok {
		sourcePlugin = fmt.Sprintf("%v", srcVal)
	} else {
		sourcePlugin = "sync_daemon"
	}

	var vectorEmbedding []byte
	if vecVal, ok := payload["vector_embedding"]; ok {
		if vecStr, isStr := vecVal.(string); isStr {
			vectorEmbedding = []byte(vecStr)
		} else if vecArr, isArr := vecVal.([]interface{}); isArr {
			bytes, _ := json.Marshal(vecArr)
			vectorEmbedding = bytes
		}
	}

	memory := orchestration.EpisodicMemory{
		MemoryID:        memoryID,
		Context:         contextStr,
		VectorEmbedding: vectorEmbedding,
		SourcePlugin:    sourcePlugin,
		CreatedAt:       time.Now().UTC(),
	}

	// Persist memory
	if s.hub.SIPDB() != nil {
		if err := s.hub.SIPDB().StoreEpisodicMemory(ctx, memory); err != nil {
			slog.Error("failed to sync context memory", "error", err)
			http.Error(w, "internal server error", http.StatusInternalServerError)
			return
		}
	} else {
		http.Error(w, "internal server error: sipdb not initialized", http.StatusInternalServerError)
		return
	}

	writeJSON(w, map[string]string{"status": "success", "message": "context synced"})
}
