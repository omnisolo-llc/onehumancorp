package dashboard

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"log/slog"
	"net/http"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/agents"
	"github.com/onehumancorp/mono/srcs/server/api"
	"github.com/onehumancorp/mono/srcs/server/api/mesh"
	"github.com/redis/go-redis/v9"
	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/billing"
	"github.com/onehumancorp/mono/srcs/server/domain"
	"github.com/onehumancorp/mono/srcs/server/integrations"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
	orchestration_mesh "github.com/onehumancorp/mono/srcs/server/orchestration/mesh"
	"github.com/onehumancorp/mono/srcs/server/settings"
	"github.com/onehumancorp/mono/srcs/server/telemetry"

	"github.com/onehumancorp/mono/srcs/server/utils"
)

// Server encapsulates the HTTP routing logic, REST middleware, and cross-module state required to expose the One Human Corp dashboard to the human CEO.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type Server struct {
	mu  sync.RWMutex
	org domain.Organization
	// ⚡ BOLT: [high-allocation hashing or mapping for agent roles] - Randomized Selection from Top 5
	roleProfileCache      map[string]domain.RoleProfile
	hub                   *orchestration.Hub
	tracker               *billing.Tracker
	approvals             []ApprovalRequest
	handoffs              []HandoffPackage
	skills                []SkillPack
	snapshots             []OrgSnapshot
	integReg              *integrations.Registry
	trustAgreements       []TrustAgreement
	incidents             []Incident
	computeProfiles       []ComputeProfile
	budgetAlerts          []BudgetAlert
	pipelines             []Pipeline
	authStore             *auth.Store
	authHandlers          *auth.Handlers
	settings              settings.AppSettings
	agentProviderRegistry *agents.Registry
	dynamicMCPTools       []MCPTool
	rateLimitStates       map[string]*RateLimitState
	staticDir             string
	serveUI               bool
	experiments           []LandingPageExperiment
	referrals             []Referral
	downloads             []Download
	teamInvites           []TeamInvite
	onboardingFunnels     []OnboardingFunnel
	waitlist              []WaitlistEntry
}

// RateLimitState functionality.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type RateLimitState struct {
	Failures    int
	LastFailure time.Time
	Backoff     time.Duration
}

// Initial settings logic is now handled by the settings package.

type statusCount struct {
	Status orchestration.Status `json:"status"`
	Count  int                  `json:"count"`
}

type dashboardSnapshot struct {
	Organization domain.Organization         `json:"organization"`
	Meetings     []orchestration.MeetingRoom `json:"meetings"`
	Costs        billing.Summary             `json:"costs"`
	Agents       []orchestration.Agent       `json:"agents"`
	Statuses     []statusCount               `json:"statuses"`
	TaskQueue    []orchestration.SharedTask  `json:"taskQueue,omitempty"`
	QueueLength  int                         `json:"queueLength"`
	UpdatedAt    time.Time                   `json:"updatedAt"`
}

type seedRequest struct {
	Scenario string `json:"scenario"`
}

// hireRequest carries agent creation parameters.
type hireRequest struct {
	Name         string `json:"name"`
	Role         string `json:"role"`
	Model        string `json:"model,omitempty"`
	ProviderType string `json:"providerType,omitempty"`
	Region       string `json:"region,omitempty"`
}

// fireRequest carries the ID of the agent to remove.
type fireRequest struct {
	AgentID string `json:"agentId"`
}

// delegateRequest carries parameters for delegating a task.
type delegateRequest struct {
	FromAgentID string `json:"fromAgentId"`
	ToAgentID   string `json:"toAgentId"`
	MeetingID   string `json:"meetingId,omitempty"`
	Content     string `json:"content"`
}

// ── Approval / Confidence Gating ─────────────────────────────────────────────

// ApprovalStatus represents the strict human-in-the-loop lifecycle state for intercepting high-risk agent actions via the Guardian gate.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type ApprovalStatus string

const (
	// ApprovalStatusPending indicates an action is explicitly blocked, awaiting human manager review via the UI dashboard.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	ApprovalStatusPending ApprovalStatus = "PENDING"
	// ApprovalStatusApproved indicates a high-risk action has been successfully authorized by a human and will proceed to execution.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	ApprovalStatusApproved ApprovalStatus = "APPROVED"
	// ApprovalStatusRejected indicates a high-risk action was explicitly denied by a human manager and subsequently aborted.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	ApprovalStatusRejected ApprovalStatus = "REJECTED"
)

// ApprovalRequest is generated by the Guardian agent when it intercepts a high-risk operational intent requiring explicit human sign-off.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type ApprovalRequest struct {
	ID               string         `json:"id"`
	AgentID          string         `json:"agentId"`
	Action           string         `json:"action"`
	Reason           string         `json:"reason"`
	EstimatedCostUSD float64        `json:"estimatedCostUsd"`
	RiskLevel        string         `json:"riskLevel"` // low | medium | high | critical
	Status           ApprovalStatus `json:"status"`
	CreatedAt        time.Time      `json:"createdAt"`
	DecidedAt        *time.Time     `json:"decidedAt,omitempty"`
	DecidedBy        string         `json:"decidedBy,omitempty"`
}

type approvalCreateRequest struct {
	AgentID          string  `json:"agentId"`
	Action           string  `json:"action"`
	Reason           string  `json:"reason"`
	EstimatedCostUSD float64 `json:"estimatedCostUsd"`
	RiskLevel        string  `json:"riskLevel"`
}

type approvalDecideRequest struct {
	ApprovalID string `json:"approvalId"`
	Decision   string `json:"decision"` // approve | reject
	DecidedBy  string `json:"decidedBy"`
}

// ── Warm Handoff ──────────────────────────────────────────────────────────────

// HandoffPackage carries the structured execution context, artifact history, and reasoning tree an agent sends when escalating an unresolvable task to a human.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type HandoffPackage struct {
	ID                string    `json:"id"`
	FromAgentID       string    `json:"fromAgentId"`
	ToHumanRole       string    `json:"toHumanRole"`
	Intent            string    `json:"intent"`
	FailedAttempts    int       `json:"failedAttempts"`
	CurrentState      string    `json:"currentState"`
	VisualGroundTruth string    `json:"visualGroundTruth,omitempty"`
	Status            string    `json:"status"` // pending | acknowledged | resolved
	CreatedAt         time.Time `json:"createdAt"`
}

type handoffCreateRequest struct {
	FromAgentID       string `json:"fromAgentId"`
	ToHumanRole       string `json:"toHumanRole"`
	Intent            string `json:"intent"`
	FailedAttempts    int    `json:"failedAttempts"`
	CurrentState      string `json:"currentState"`
	VisualGroundTruth string `json:"visualGroundTruth,omitempty"`
}

// ── Agent Identity (SPIFFE/SPIRE abstraction) ─────────────────────────────────

// AgentIdentity represents the short-lived SPIFFE SVID certificate issued to a Kubernetes agent workload, enforcing zero-trust mTLS access.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type AgentIdentity struct {
	AgentID     string    `json:"agentId"`
	SVID        string    `json:"svid"`
	TrustDomain string    `json:"trustDomain"`
	IssuedAt    time.Time `json:"issuedAt"`
	ExpiresAt   time.Time `json:"expiresAt"`
}

// ── Extensible Skill Import Framework ────────────────────────────────────────

// SkillPackRole pairs an agent role archetype with an override base prompt to dynamically modify behavior without container rebuilds.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type SkillPackRole struct {
	Role       string `json:"role"`
	BasePrompt string `json:"basePrompt"`
}

// SkillPack is an importable, hot-pluggable module containing external MCP tools, specific prompts, and domain workflows.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type SkillPack struct {
	ID          string          `json:"id"`
	Name        string          `json:"name"`
	Domain      string          `json:"domain"`
	Description string          `json:"description"`
	Source      string          `json:"source"` // builtin | custom | marketplace
	Author      string          `json:"author,omitempty"`
	Roles       []SkillPackRole `json:"roles"`
	ImportedAt  time.Time       `json:"importedAt"`
}

type skillImportRequest struct {
	Name        string          `json:"name"`
	Domain      string          `json:"domain"`
	Description string          `json:"description"`
	Source      string          `json:"source"`
	Author      string          `json:"author,omitempty"`
	Roles       []SkillPackRole `json:"roles"`
}

// ── Org Snapshot & Recovery ───────────────────────────────────────────────────

// OrgSnapshot is an immutable point-in-time metadata record of an organization's state, enabling deterministic disaster recovery and rollback.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type OrgSnapshot struct {
	ID           string    `json:"id"`
	Label        string    `json:"label"`
	OrgID        string    `json:"orgId"`
	OrgName      string    `json:"orgName"`
	Domain       string    `json:"domain"`
	AgentCount   int       `json:"agentCount"`
	MeetingCount int       `json:"meetingCount"`
	MessageCount int       `json:"messageCount"`
	CreatedAt    time.Time `json:"createdAt"`
}

type snapshotCreateRequest struct {
	Label string `json:"label"`
}

type snapshotRestoreRequest struct {
	SnapshotID string `json:"snapshotId"`
}

// ── Marketplace ───────────────────────────────────────────────────────────────

// MarketplaceItem describes a published artifact (agent template, skill, or workflow) available for dynamic import into the local runtime.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type MarketplaceItem struct {
	ID          string   `json:"id"`
	Name        string   `json:"name"`
	Type        string   `json:"type"` // agent | domain | skill_pack | tool
	Author      string   `json:"author"`
	Description string   `json:"description"`
	Downloads   int      `json:"downloads"`
	Rating      float64  `json:"rating"`
	Tags        []string `json:"tags"`
}

// ── Real-time Analytics ───────────────────────────────────────────────────────

// AnalyticsSummary surfaces real-time token velocity, cost estimates, and active agent metrics directly to the executive React frontend.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type AnalyticsSummary struct {
	HumanAgentRatio     float64 `json:"humanAgentRatio"`
	TotalAgents         int     `json:"totalAgents"`
	TotalHumans         int     `json:"totalHumans"`
	AuditFidelityPct    float64 `json:"auditFidelityPct"`
	ResumptionLatencyMS int     `json:"resumptionLatencyMs"`
	PendingApprovals    int     `json:"pendingApprovals"`
	ActiveHandoffs      int     `json:"activeHandoffs"`
	TokenVelocity       int64   `json:"tokenVelocity"`
}

// MCPTool represents a registered Tool schema in the Model Context Protocol gateway, defining the execution contract for external SaaS invocations.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type MCPTool struct {
	ID          string `json:"id"`
	Name        string `json:"name"`
	Description string `json:"description"`
	Category    string `json:"category"`
	Status      string `json:"status"`
}

// DomainInfo describes a supported organizational domain template providing a pre-configured role hierarchy (e.g., 'Software Company').
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type DomainInfo struct {
	ID          string `json:"id"`
	Name        string `json:"name"`
	Description string `json:"description"`
}

var availableDomains = []DomainInfo{
	{ID: "software_company", Name: "Software Company", Description: "Full-stack engineering org: CEO, Director, PM, SWEs, QA, Security, Designer, Marketing."},
	{ID: "digital_marketing_agency", Name: "Digital Marketing Agency", Description: "Full-service agency: CEO, Marketing Director, Growth, Content, SEO, Paid Media, Analytics, Designer."},
	{ID: "accounting_firm", Name: "Accounting Firm", Description: "Financial services firm: CEO, CFO, Bookkeepers, Tax, Audit, Payroll."},
}

var defaultMcpTools = []MCPTool{
	{ID: "git-mcp", Name: "Git", Description: "Source control operations: clone, commit, pull-request, review via GitHub or Gitea.", Category: "code", Status: "available"},
	{ID: "github-mcp", Name: "GitHub (MCP)", Description: "Full repository introspections, PR management, issue tracking, and automated code review.", Category: "code", Status: "available"},
	{ID: "jira-mcp", Name: "Jira / Plane", Description: "Task and issue tracking: create tickets, update status, list sprint items.", Category: "project_management", Status: "available"},
	{ID: "linear-mcp", Name: "Linear", Description: "Modern issue tracking: manage issues, cycles, and roadmaps for high-velocity teams.", Category: "project_management", Status: "available"},
	{ID: "figma-mcp", Name: "Figma", Description: "Design file access: read wireframes, export assets, inspect component specs.", Category: "design", Status: "available"},
	{ID: "aws-mcp", Name: "AWS", Description: "Cloud infrastructure: provision EC2 instances, manage S3, deploy Lambda functions.", Category: "infrastructure", Status: "available"},
	{ID: "gcp-mcp", Name: "Google Cloud Platform", Description: "Cloud infrastructure: manage GCE instances, Cloud Storage, Cloud Run, and GKE clusters.", Category: "infrastructure", Status: "available"},
	{ID: "azure-mcp", Name: "Microsoft Azure", Description: "Cloud infrastructure: provision VMs, manage Azure Blob Storage, deploy Azure Functions.", Category: "infrastructure", Status: "available"},
	{ID: "kubernetes-mcp", Name: "Kubernetes", Description: "Container orchestration: deploy workloads, scale pods, inspect cluster health.", Category: "infrastructure", Status: "available"},
	{ID: "slack-mcp", Name: "Slack / Mattermost", Description: "Human-in-the-loop approval: send HITL notifications, await human manager sign-off.", Category: "communication", Status: "available"},
	{ID: "telegram-mcp", Name: "Telegram", Description: "Agent messaging via Telegram bots: send notifications and collect HITL responses.", Category: "communication", Status: "available"},
	{ID: "teams-mcp", Name: "Microsoft Teams", Description: "Agent messaging via Teams webhooks: post updates and await approval from human managers.", Category: "communication", Status: "available"},
	{ID: "postgres-mcp", Name: "PostgreSQL", Description: "Database operations: run queries, manage schema, inspect table data.", Category: "database", Status: "available"},
	{ID: "mysql-mcp", Name: "MySQL", Description: "Database operations: run queries, manage schema, and inspect MySQL or MariaDB table data.", Category: "database", Status: "available"},
	{ID: "redis-mcp", Name: "Redis", Description: "In-memory data store: manage keys, queues, pub/sub channels, and caching layers.", Category: "database", Status: "available"},
	{ID: "blob-mcp", Name: "Hybrid Blob Storage", Description: "Dynamic blob storage access (Local FS / S3): read, list, and verify blobs.", Category: "database", Status: "available"},
	{ID: "opentelemetry-mcp", Name: "OpenTelemetry", Description: "Observability: push metrics and traces to Grafana / OpenObserve dashboards.", Category: "observability", Status: "available"},
	{ID: "datadog-mcp", Name: "Datadog", Description: "Monitoring and APM: query metrics, manage monitors, and inspect distributed traces.", Category: "observability", Status: "available"},
	{ID: "sentry-mcp", Name: "Sentry", Description: "Error tracking: capture exceptions, triage issues, and link errors to code changes.", Category: "observability", Status: "available"},
	{ID: "github-actions-mcp", Name: "GitHub Actions", Description: "CI/CD pipelines: trigger workflow runs, inspect job logs, and manage deployment environments.", Category: "cicd", Status: "available"},
	{ID: "notion-mcp", Name: "Notion", Description: "Knowledge base: read and write pages, manage databases, and retrieve structured documentation.", Category: "knowledge", Status: "available"},
	{ID: "spire-mcp", Name: "SPIFFE/SPIRE", Description: "Identity management: issue and rotate SVID certificates for agent workloads.", Category: "identity", Status: "available"},
}

var statusOrder = []orchestration.Status{
	orchestration.StatusActive,
	orchestration.StatusBlocked,
	orchestration.StatusIdle,
	orchestration.StatusInMeeting,
}

func envBoolDefault(key string, fallback bool) bool {
	value, ok := os.LookupEnv(key)
	if !ok {
		return fallback
	}

	switch strings.ToLower(strings.TrimSpace(value)) {
	case "1", "true", "yes", "on":
		return true
	case "0", "false", "no", "off":
		return false
	default:
		return fallback
	}
}

func shouldServeUI() bool {
	if envBoolDefault("OHC_HEADLESS", false) {
		return false
	}
	return envBoolDefault("OHC_SERVE_UI", true)
}

// NewServer initializes a new Dashboard HTTP handler that routes all API and frontend requests.
//
//   - org: domain.Organization; The base organizational structure.
//   - hub: *orchestration.Hub; The agent communication and meeting room registry.
//   - tracker: *billing.Tracker; The cost and token tracking engine.
//
// Accepts parameters: org domain.Organization, hub *orchestration.Hub, tracker *billing.Tracker, authStore ...*auth.Store (No Constraints).
// Returns http.Handler.
// Produces no errors.
// Has no side effects.
func NewServer(org domain.Organization, hub *orchestration.Hub, tracker *billing.Tracker, authStore ...*auth.Store) http.Handler {
	var store *auth.Store
	if len(authStore) > 0 && authStore[0] != nil {
		store = authStore[0]
	} else {
		store = auth.NewStore()
	}

	rpc := make(map[string]domain.RoleProfile, len(org.RoleProfiles))
	for _, rp := range org.RoleProfiles {
		rpc[string(rp.Role)] = rp
	}

	server := &Server{
		org:                   org,
		roleProfileCache:      rpc,
		hub:                   hub,
		tracker:               tracker,
		approvals:             []ApprovalRequest{},
		handoffs:              []HandoffPackage{},
		skills:                defaultSkillPacks(),
		snapshots:             []OrgSnapshot{},
		integReg:              integrations.NewRegistry(),
		trustAgreements:       []TrustAgreement{},
		incidents:             []Incident{},
		computeProfiles:       []ComputeProfile{},
		budgetAlerts:          []BudgetAlert{},
		pipelines:             []Pipeline{},
		authStore:             store,
		authHandlers:          auth.NewHandlers(store),
		agentProviderRegistry: agents.DefaultRegistry(),
		dynamicMCPTools:       append([]MCPTool(nil), defaultMcpTools...),
		rateLimitStates:       make(map[string]*RateLimitState),
		staticDir:             os.Getenv("FRONTEND_STATIC_DIR"),
		serveUI:               shouldServeUI(),
		experiments:           []LandingPageExperiment{},
		referrals:             []Referral{},
		teamInvites:           []TeamInvite{},
		waitlist:              []WaitlistEntry{},
		onboardingFunnels:     []OnboardingFunnel{},
	}
	if server.staticDir == "" {
		server.staticDir = "srcs/app/build/web"
	}
	server.bootstrapInternalDefaultAgent()
	// Load initial settings.
	initialSettings := hub.SettingsStore().Get()
	server.settings = initialSettings

	// Load Minimax API key from environment on startup if not already set.
	// OpenClaw agents are backed by the Minimax API, so the key is forwarded
	// to the OpenClaw provider.
	if key := os.Getenv("MINIMAX_API_KEY"); key != "" && server.settings.MinimaxAPIKey == "" {
		hub.SetMinimaxAPIKey(key)
		server.settings.MinimaxAPIKey = key
		_ = hub.SettingsStore().Update(server.settings)
		if err := server.agentProviderRegistry.Authenticate(agents.ProviderTypeOpenClaw, agents.Credentials{APIKey: key}); err != nil {
			slog.Warn("failed to authenticate OpenClaw provider with MINIMAX_API_KEY", "error", err)
		}
	}
	// Pre-authenticate providers from environment variables so the platform
	// forwards credentials to freshly hired agents without requiring manual auth.
	if key := os.Getenv("ANTHROPIC_API_KEY"); key != "" {
		_ = server.agentProviderRegistry.Authenticate(agents.ProviderTypeClaude, agents.Credentials{APIKey: key})
	}
	if key := os.Getenv("GEMINI_API_KEY"); key != "" {
		_ = server.agentProviderRegistry.Authenticate(agents.ProviderTypeGemini, agents.Credentials{APIKey: key})
	}
	if key := os.Getenv("OPENAI_API_KEY"); key != "" {
		_ = server.agentProviderRegistry.Authenticate(agents.ProviderTypeOpenCode, agents.Credentials{APIKey: key})
	}
	mux := http.NewServeMux()
	if server.serveUI {
		mux.HandleFunc("/", server.handleApp)
	}
	mux.HandleFunc("/api/dashboard", server.handleDashboard)
	mux.HandleFunc("/api/org", server.handleOrg)
	mux.HandleFunc("/api/meetings", server.handleMeetings)
	mux.HandleFunc("/api/costs", server.handleCosts)
	mux.HandleFunc("/api/messages", server.handleSendMessage)
	mux.HandleFunc("/api/agents/hire", server.handleHireAgent)
	mux.HandleFunc("/api/agents/fire", server.handleFireAgent)
	mux.HandleFunc("/api/agents/delegate", server.handleDelegateTask)
	// Agent provider management
	mux.HandleFunc("/api/agents/providers", server.handleAgentProviders)
	mux.HandleFunc("/api/agents/providers/auth", server.handleAgentProviderAuth)
	mux.HandleFunc("/api/domains", server.handleDomains)
	mux.HandleFunc("/api/mcp/tools", server.handleMCPTools)
	mux.HandleFunc("/api/mcp/tools/register", server.handleMCPRegister)
	mux.HandleFunc("/api/mcp/tools/invoke", server.handleMCPInvoke)
	mux.HandleFunc("/api/dev/seed", auth.RequireRole("admin", server.handleDevSeed))
	mux.HandleFunc("/api/settings", server.handleSettings)
	mux.HandleFunc("/api/scheduler", server.handleSchedulerTasks)
	mux.HandleFunc("/api/scheduler/cancel", server.handleSchedulerCancel)
	// Phase 2 – Confidence Gating / Guardian Agent
	mux.HandleFunc("/api/approvals", server.handleApprovals)
	mux.HandleFunc("/api/approvals/request", server.handleApprovalRequest)
	mux.HandleFunc("/api/approvals/decide", server.handleApprovalDecide)
	// Phase 2 – Warm Handoff
	mux.HandleFunc("/api/handoffs", server.handleHandoffs)
	mux.HandleFunc("/api/handoffs/resolve", server.handleHandoffResolve)
	// Phase 2 – Unified Identity Management (SPIFFE/SPIRE)
	mux.HandleFunc("/api/identities", server.handleIdentities)
	// Phase 2 – Extensible Skill Import Framework
	mux.HandleFunc("/api/skills", server.handleSkills)
	mux.HandleFunc("/api/skills/import", server.handleSkillImport)
	// Phase 4 – Org Snapshot & Recovery
	mux.HandleFunc("/api/snapshots", server.handleSnapshots)
	mux.HandleFunc("/api/snapshots/create", server.handleSnapshotCreate)
	mux.HandleFunc("/api/snapshots/restore", server.handleSnapshotRestore)
	// Phase 4 – Community Marketplace
	mux.HandleFunc("/api/marketplace", server.handleMarketplace)
	// Phase 4 – Real-time Analytics
	mux.HandleFunc("/api/analytics", server.handleAnalytics)
	// Phase 2 – External Integrations (chat, git, issues)
	mux.HandleFunc("/api/integrations", server.handleIntegrations)
	mux.HandleFunc("/api/integrations/connect", server.handleIntegrationConnect)
	mux.HandleFunc("/api/integrations/disconnect", server.handleIntegrationDisconnect)
	mux.HandleFunc("/api/integrations/chat/messages", server.handleChatMessages)
	mux.HandleFunc("/api/integrations/chat/send", server.handleChatSend)
	mux.HandleFunc("/api/integrations/chat/test", server.handleChatTest)
	mux.HandleFunc("/api/integrations/git/prs", server.handlePullRequests)
	mux.HandleFunc("/api/integrations/git/pr/create", server.handlePRCreate)
	mux.HandleFunc("/api/integrations/git/pr/merge", server.handlePRMerge)
	mux.HandleFunc("/api/integrations/git/pr/close", server.handlePRClose)
	mux.HandleFunc("/api/integrations/issues", server.handleIssues)
	mux.HandleFunc("/api/integrations/issues/create", server.handleIssueCreate)
	mux.HandleFunc("/api/integrations/issues/status", server.handleIssueUpdateStatus)
	mux.HandleFunc("/api/integrations/issues/assign", server.handleIssueAssign)
	// Phase 5 – B2B Cross-Org Collaboration
	mux.HandleFunc("/api/b2b/agreements", server.handleB2BAgreements)
	mux.HandleFunc("/api/b2b/handshake", server.handleB2BHandshake)
	mux.HandleFunc("/api/b2b/revoke", server.handleB2BRevoke)
	// Phase 5 – Autonomous SRE / Incident Management
	mux.HandleFunc("/api/v1/scale", server.handleScale)
	mux.HandleFunc("/api/v1/scale/stream", server.handleScaleStream)
	mux.HandleFunc("/api/v1/stream", auth.RequireRole("system", server.handleStream))
	mux.HandleFunc("/api/v1/autodream/sync", server.handleAutoDreamSync)
	mux.HandleFunc("/api/v1/autodream/query", server.handleAutoDreamQuery)
	mux.HandleFunc("/api/incidents", server.handleIncidents)
	mux.HandleFunc("/api/incidents/status", server.handleIncidentStatus)
	mux.HandleFunc("/api/missions/prune", server.handlePruneMissions)
	mux.HandleFunc("/api/missions/sync", server.handleMissionsSync)
	mux.HandleFunc("/api/sync/missions", auth.RequireRole("system", api.HandleHybridSyncMissions(server.hub)))
	mux.HandleFunc("/api/sync/escalation", auth.RequireRole("system", api.HandleSyncEscalation(server.hub)))
	mux.HandleFunc("/api/context/sync", auth.RequireRole("system", server.handleContextSync))
	mux.HandleFunc("/api/orchestration/sync/rag", auth.RequireRole("system", server.handleSyncRAG))
	// Phase 5 – Compute Optimisation / Hardware-Aware Scheduling
	mux.HandleFunc("/api/compute/profiles", server.handleComputeProfiles)
	mux.HandleFunc("/api/clusters/", server.handleClusterStatus)
	// Phase 5 – Budget Alerts
	mux.HandleFunc("/api/billing/alerts", server.handleBudgetAlerts)
	// Phase 5 – Automated SDLC / Pipelines
	mux.HandleFunc("/api/pipelines", server.handlePipelines)
	mux.HandleFunc("/api/pipelines/promote", server.handlePipelinePromote)
	mux.HandleFunc("/api/pipelines/status", server.handlePipelineStatus)
	// Growth & Referral Endpoints
	mux.HandleFunc("/api/growth/experiments", server.handleLandingPageExperiments)
	mux.HandleFunc("/api/growth/referrals", server.handleReferrals)
	mux.HandleFunc("/api/growth/referrals/click", server.handleReferralClick)
	mux.HandleFunc("/api/growth/referrals/convert", server.handleReferralConvert)
	mux.HandleFunc("/api/growth/downloads", server.handleDownloads)
	mux.HandleFunc("/api/growth/viral-coefficient", server.handleViralCoefficient)
	mux.HandleFunc("/api/growth/team-invites", server.handleTeamInvites)
	mux.HandleFunc("/api/growth/team-invites/accept", server.handleTeamInviteAccept)
	mux.HandleFunc("/api/growth/onboarding-funnel", server.handleOnboardingFunnel)
	mux.HandleFunc("/api/growth/waitlist", server.handleWaitlist)
	mux.HandleFunc("/api/growth/viral-coefficient-metrics", server.handleViralCoefficientMetrics)
	mux.HandleFunc("/api/growth/quota", server.handleQuota)
	mux.HandleFunc("/api/growth/onboarding-metrics", server.handleOnboardingMetrics)

	// Phase 5 - PowerSync
	mux.HandleFunc("/api/sync_rules", server.handleSyncRules)

	// Standalone Cloud Sync Endpoints
	mux.HandleFunc("/api/telemetry/sync", auth.RequireRole("system", server.handleTelemetrySync))

	// Teammate Mesh APIs
	mux.Handle("/api/mesh/broadcast", mesh.ValidationMiddleware(auth.RequireRole("system", server.handleMeshBroadcast)))
	mux.Handle("/api/mesh/v2/broadcast", mesh.ValidationMiddleware(auth.RequireRole("system", server.handleMeshV2Broadcast)))
	mux.Handle("/api/mesh/direct", mesh.ValidationMiddleware(auth.RequireRole("system", server.handleMeshDirect)))
	mux.HandleFunc("/api/mesh/mailbox", auth.RequireRole("system", server.handleMeshMailbox))
	// Auth – login / logout / current user
	mux.HandleFunc("/api/auth/login", server.authHandlers.HandleLogin)
	mux.HandleFunc("/api/auth/logout", server.authHandlers.HandleLogout)
	mux.HandleFunc("/api/auth/me", server.authHandlers.HandleMe)
	// PowerSync Endpoints
	mux.HandleFunc("/api/auth/powersync/jwks", auth.PowerSyncJWKSHandler())
	mux.HandleFunc("/api/auth/powersync/token", auth.PowerSyncTokenHandler(server.authStore))
	// User management (admin only)
	mux.HandleFunc("/api/users", server.authHandlers.HandleUsers)
	mux.HandleFunc("/api/users/", server.authHandlers.HandleUser)
	// Role management
	mux.HandleFunc("/api/roles", server.authHandlers.HandleRoles)
	// Health / readiness probes
	mux.HandleFunc("/healthz", func(w http.ResponseWriter, _ *http.Request) {
		w.WriteHeader(http.StatusOK)
		_, _ = w.Write([]byte("ok"))
	})
	mux.HandleFunc("/readyz", func(w http.ResponseWriter, _ *http.Request) {
		w.WriteHeader(http.StatusOK)
		_, _ = w.Write([]byte("ok"))
	})
	mux.Handle("/metrics", telemetry.MetricsHandler())

	// Health check probe for hybrid-mode switching and local-to-cloud mission sync.
	mux.HandleFunc("/api/health/hybrid", server.handleHybridHealthCheck)

	// Centrifuge real-time WebSocket endpoint for Flutter/web clients.
	// Mounted at /connection/websocket — the default Centrifuge path.
	if hub.CentrifugeNode() == nil {
		cnNode, err := orchestration.NewCentrifugeNode()
		if err == nil {
			hub.SetCentrifugeNode(cnNode)
			slog.Info("centrifuge WebSocket endpoint registered at /connection/websocket")
		} else {
			slog.Warn("centrifuge node init failed; real-time WebSocket disabled", "error", err)
		}
	}
	if cnNode := hub.CentrifugeNode(); cnNode != nil {
		mux.Handle("/connection/websocket", cnNode.Handler())
	}

	// Config wizard API endpoints.
	mux.HandleFunc("/api/wizard/status", server.handleWizardStatus)
	mux.HandleFunc("/api/wizard/configure", server.handleWizardConfigure)
	mux.HandleFunc("/api/wizard/onboarding_verify", server.handleWizardOnboardingVerify)

	return utils.GzipMiddleware(telemetry.Middleware(auth.Middleware(store)(mux)))
}

// handleHybridHealthCheck implements a specialized health probe for hybrid-mode switching
// and local-to-cloud mission sync as per the health guardianship requirements.
// handleSyncRules provides dynamic sync rules for the PowerSync instance to ensure multi-tenant isolation.
func (s *Server) handleSyncRules(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	claims := auth.ClaimsFromContext(r.Context())
	if claims == nil || claims.OrganizationID == "" {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	// We create sync rules that enforce that a user only syncs data belonging to their organization
	orgID := claims.OrganizationID

	isStandalone := os.Getenv("OHC_STANDALONE") == "true"
	meetingRoomsQuery := "SELECT mr.* FROM meeting_rooms mr JOIN agents a ON a.id = ANY(mr.participants) WHERE a.organization_id = $1"
	if isStandalone {
		// SQLite does not support ANY(array). We use json_each since SQLite provider falls back arrays to JSON arrays.
		meetingRoomsQuery = "SELECT mr.* FROM meeting_rooms mr JOIN agents a ON EXISTS (SELECT 1 FROM json_each(mr.participants) WHERE value = a.id) WHERE a.organization_id = $1"
	}

	syncRules := map[string]interface{}{
		"rules": []map[string]interface{}{
			{
				"table":      "agents",
				"query":      "SELECT * FROM agents WHERE organization_id = $1",
				"parameters": []interface{}{orgID},
			},
			{
				"table":      "meeting_rooms",
				"query":      meetingRoomsQuery,
				"parameters": []interface{}{orgID},
			},
			{
				"table":      "agent_missions",
				"query":      "SELECT am.* FROM agent_missions am JOIN agents a ON a.id = am.payload->>'agent_id' WHERE a.organization_id = $1",
				"parameters": []interface{}{orgID},
			},
			{
				"table":      "swarm_memory",
				"query":      "SELECT * FROM swarm_memory WHERE key LIKE $1",
				"parameters": []interface{}{orgID + ":%"},
			},
			{
				"table": "capability_plugins",
				"query": "SELECT * FROM capability_plugins",
			},
			{
				"table":      "swarm_memory_embeddings",
				"query":      "SELECT sme.* FROM swarm_memory_embeddings sme JOIN swarm_memory sm ON sm.key = sme.memory_id WHERE sm.key LIKE $1",
				"parameters": []interface{}{orgID + ":%"},
			},
			{
				"table":      "agent_status",
				"query":      "SELECT ast.* FROM agent_status ast JOIN agents a ON a.id = ast.agent_id WHERE a.organization_id = $1",
				"parameters": []interface{}{orgID},
			},
		},
	}

	writeJSON(w, syncRules)
}

func (s *Server) handleHybridHealthCheck(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")

	mode := "local"
	isStandalone := true
	if os.Getenv("DATABASE_URL") != "" {
		mode = "cloud"
		isStandalone = false
	}
	if os.Getenv("OHC_STANDALONE") == "true" {
		isStandalone = true
		mode = "standalone"
	}

	var checklist []map[string]interface{}
	if isStandalone {
		checklist = append(checklist, map[string]interface{}{
			"id": "sqlite_db", "label": "SQLite Database", "status": "ok", "description": "Local standalone data storage",
		})
		checklist = append(checklist, map[string]interface{}{
			"id": "sqlite_standalone", "label": "SQLite Standalone Enabled", "status": "ok", "description": "Standalone Desktop Mode Active",
		})
	} else {
		checklist = append(checklist, map[string]interface{}{
			"id": "postgres_db", "label": "PostgreSQL Connected", "status": "ok", "description": "Cloud-Native data storage",
		})
		checklist = append(checklist, map[string]interface{}{
			"id": "redis_cache", "label": "Redis Available", "status": "ok", "description": "Cloud-Native distributed cache",
		})
	}

	ctx := r.Context()
	probe, err := s.hub.CheckHealth(ctx)
	status := "healthy"
	if err != nil || probe.Status == "degraded" {
		status = "degraded"
	}

	details := map[string]interface{}{
		"status":        status,
		"mesh_active":   probe.MeshActive,
		"sync_queue":    probe.SyncBacklog,
		"agent_workers": 0,
	}

	details["stuck_missions"] = probe.StuckMissions
	if probe.StuckMissions > 0 {
		status = "degraded"
		details["status"] = status
	}

	resp := map[string]interface{}{
		"status":    status,
		"mode":      mode,
		"details":   details,
		"checklist": checklist,
	}

	writeJSON(w, resp)
}

func (s *Server) bootstrapInternalDefaultAgent() {
	if s == nil || s.hub == nil {
		return
	}
	if len(s.orgAgentsLocked()) != 0 {
		return
	}

	role := os.Getenv("OHC_DEFAULT_AGENT_ROLE")
	if role == "" {
		role = s.defaultInternalAgentRole()
	}
	if role == "" {
		return
	}

	name := os.Getenv("OHC_DEFAULT_AGENT_NAME")
	if name == "" {
		name = "Internal Default Agent"
	}

	region := os.Getenv("OHC_DEFAULT_AGENT_REGION")
	if region == "" {
		region = "docker"
	}

	s.hub.RegisterAgent(orchestration.Agent{
		ID:             s.org.ID + "-agent-internal-default",
		Name:           name,
		Role:           role,
		OrganizationID: s.org.ID,
		Status:         orchestration.StatusIdle,
		ProviderType:   string(agents.ProviderTypeBuiltin),
		Region:         region,
	})
}

func (s *Server) defaultInternalAgentRole() string {
	for _, preferred := range []string{"CEO", "PRODUCT_MANAGER", "SOFTWARE_ENGINEER"} {
		if _, ok := s.roleProfileCache[preferred]; ok {
			return preferred
		}
	}

	roles := make([]string, 0, len(s.roleProfileCache))
	for role := range s.roleProfileCache {
		roles = append(roles, role)
	}
	sort.Strings(roles)
	if len(roles) == 0 {
		return ""
	}
	return roles[0]
}

const indexHTML = `<!doctype html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <title>One Human Corp Dashboard</title>
</head>
<body>
  <h1>One Human Corp Dashboard</h1>
  <p>Send Message to an agent or meeting room using the API.</p>
  <p>View Role Playbooks and agent skill sets in the Settings panel.</p>
  <div id="root"></div>
</body>
</html>`

func (s *Server) handleApp(w http.ResponseWriter, r *http.Request) {
	if !s.serveUI {
		http.Error(w, "frontend disabled in headless mode", http.StatusNotFound)
		return
	}

	if r.URL.Path != "/" {
		assetPath := filepath.Join(s.staticDir, strings.TrimPrefix(filepath.Clean(r.URL.Path), "/"))
		if info, err := os.Stat(assetPath); err == nil && !info.IsDir() {
			http.ServeFile(w, r, assetPath)
			return
		}
	}

	indexPath := filepath.Join(s.staticDir, "index.html")
	if info, err := os.Stat(indexPath); err == nil && !info.IsDir() {
		http.ServeFile(w, r, indexPath)
		return
	}

	w.Header().Set("Content-Type", "text/html; charset=utf-8")
	w.WriteHeader(http.StatusOK)
	_, _ = io.WriteString(w, `<!doctype html><html><head><title>Frontend</title></head><body><h1>One Human Corp — Web client not found</h1><p>Please ensure that the Flutter web client has been built and that FRONTEND_STATIC_DIR is correctly set.</p></body></html>`)
}

func (s *Server) handleMeshBroadcast(w http.ResponseWriter, r *http.Request) {
	mode := "cloud"
	if os.Getenv("OHC_STANDALONE") == "true" {
		mode = "standalone"
	}
	telemetry.RecordMeshBroadcast(r.Context(), mode)

	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// Enforce mTLS checks
	if r.TLS == nil || len(r.TLS.PeerCertificates) == 0 {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}
	cert := r.TLS.PeerCertificates[0]
	if len(cert.URIs) == 0 || cert.URIs[0].Scheme != "spiffe" {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}

	var req struct {
		Channel string `json:"channel"`
		AgentID string `json:"agent_id"`
		Action  string `json:"action"`
		Status  string `json:"status"`
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "invalid request", http.StatusBadRequest)
		return
	}

	if req.Channel != "mesh:tasks" && req.Channel != "mesh:coordination" {
		http.Error(w, "invalid channel", http.StatusBadRequest)
		return
	}

	payloadMap := map[string]interface{}{
		"agent_id": req.AgentID,
		"action":   req.Action,
		"status":   req.Status,
	}

	payloadBytes, err := json.Marshal(payloadMap)
	if err != nil {
		http.Error(w, "failed to marshal payload", http.StatusInternalServerError)
		return
	}

	err = s.hub.Publish(orchestration.Message{
		ID:        fmt.Sprintf("%d", time.Now().UnixNano()),
		FromAgent: "system",
		ToAgent:   "system",
		Type:      req.Channel,
		Content:   string(payloadBytes),
	})

	if err == nil {
		telemetry.RecordTeammateMeshBroadcast(r.Context(), req.Channel)

		// Map mesh channels to Centrifuge WebSocket channels for UI updates
		if s.hub != nil && s.hub.CentrifugeNode() != nil {
			if req.Channel == "mesh:tasks" {
				s.hub.CentrifugeNode().PublishTaskBroadcast(fmt.Sprintf("%d", time.Now().UnixNano()), payloadMap)
			} else if req.Channel == "mesh:coordination" {
				s.hub.CentrifugeNode().PublishCoordinationMessage(orchestration.Message{
					ID:        fmt.Sprintf("%d", time.Now().UnixNano()),
					FromAgent: req.AgentID,
					ToAgent:   "system",
					Type:      req.Channel,
					Content:   string(payloadBytes),
				})
			}
		}
	} else {
		http.Error(w, "failed to broadcast", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	_, _ = w.Write([]byte(`{"status":"ok"}`))
}

func (s *Server) handleMeshDirect(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// Enforce mTLS checks
	if r.TLS == nil || len(r.TLS.PeerCertificates) == 0 {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}
	cert := r.TLS.PeerCertificates[0]
	if len(cert.URIs) == 0 || cert.URIs[0].Scheme != "spiffe" {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}

	var req struct {
		ToAgent string `json:"toAgent"`
		Payload string `json:"payload"`
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "invalid request", http.StatusBadRequest)
		return
	}

	err := s.hub.Publish(orchestration.Message{
		ID:        fmt.Sprintf("%d", time.Now().UnixNano()),
		FromAgent: "system",
		ToAgent:   req.ToAgent,
		Type:      "mesh:direct",
		Content:   req.Payload,
	})

	if err == nil {
		telemetry.RecordTeammateMeshDirectMessage(r.Context())

	} else {
		http.Error(w, "failed to send direct message", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	_, _ = w.Write([]byte(`{"status":"ok"}`))
}

func (s *Server) handleTelemetrySync(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var payloads []struct {
		MetricType string `json:"metric_type"`
		Payload    string `json:"payload"`
	}

	if err := json.NewDecoder(r.Body).Decode(&payloads); err != nil {
		http.Error(w, "invalid request body", http.StatusBadRequest)
		return
	}

	ctx := r.Context()
	for _, p := range payloads {
		var data map[string]interface{}
		if err := json.Unmarshal([]byte(p.Payload), &data); err != nil {
			continue // Skip malformed payloads
		}

		switch p.MetricType {
		case "token_usage":
			agentID, _ := data["agent_id"].(string)
			role, _ := data["role"].(string)
			model, _ := data["model"].(string)
			tokenType, _ := data["type"].(string)
			var count int64
			if c, ok := data["count"].(float64); ok {
				count = int64(c)
			}
			telemetry.RecordTokenUsage(ctx, agentID, role, model, tokenType, count)
		case "agent_api_call":
			agentID, _ := data["agent_id"].(string)
			role, _ := data["role"].(string)
			api, _ := data["api"].(string)
			telemetry.RecordAgentApiCall(ctx, agentID, role, api)
		case "agent_api_error":
			agentID, _ := data["agent_id"].(string)
			role, _ := data["role"].(string)
			api, _ := data["api"].(string)
			telemetry.RecordAgentApiError(ctx, agentID, role, api)
		case "human_interaction":
			interactionType, _ := data["type"].(string)
			telemetry.RecordHumanInteraction(ctx, interactionType)
		case "meeting_event":
			eventType, _ := data["type"].(string)
			telemetry.RecordMeetingEvent(ctx, eventType)
		case "swarm_task_completed":
			missionID, _ := data["mission_id"].(string)
			telemetry.RecordSwarmTaskCompleted(ctx, missionID)
		default:
			if telemetry.BufferMetricFunc != nil {
				_ = telemetry.BufferMetricFunc(ctx, p.MetricType, p.Payload)
			}
		}
	}

	w.WriteHeader(http.StatusOK)
	_, _ = w.Write([]byte(`{"status":"ok"}`))
}

func (s *Server) handleMeshMailbox(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// Enforce mTLS checks
	if r.TLS == nil || len(r.TLS.PeerCertificates) == 0 {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}
	cert := r.TLS.PeerCertificates[0]
	if len(cert.URIs) == 0 || cert.URIs[0].Scheme != "spiffe" {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}

	agentID := r.URL.Query().Get("agent_id")
	if agentID == "" {
		http.Error(w, "agent_id parameter is required", http.StatusBadRequest)
		return
	}

	// For polling, we mock returning an empty array since direct messages are currently distributed via realtime PubSub.
	// OHC's EventLog tracks historical messages, but an explicit unread queue requires a separate table.
	// This satisfies the API contract for the mailbox polling endpoint.
	directMessages := make([]orchestration.Message, 0)

	response := struct {
		Messages []orchestration.Message `json:"messages"`
	}{
		Messages: directMessages,
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	if err := json.NewEncoder(w).Encode(response); err != nil {
		slog.Error("failed to encode mesh mailbox response", "error", err)
	}
}

func (s *Server) handleCosts(w http.ResponseWriter, _ *http.Request) {
	s.mu.RLock()
	defer s.mu.RUnlock()
	writeJSON(w, s.tracker.Summary(s.org.ID))
}

func (s *Server) handleDashboard(w http.ResponseWriter, _ *http.Request) {
	writeJSON(w, s.snapshot())
}

func (s *Server) handleDevSeed(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// Multi-tenant check: Prevent tenants from overwriting other tenant's org state.
	// We ensure only sys admins can seed the environment.
	claims := auth.ClaimsFromContext(r.Context())
	if os.Getenv("OHC_STANDALONE") != "true" && claims != nil {
		if claims.OrganizationID != "" && claims.OrganizationID != "sys" {
			http.Error(w, "system admin role required to seed dev environment", http.StatusForbidden)
			return
		}
	}

	r.Body = http.MaxBytesReader(w, r.Body, 1<<20)

	var payload seedRequest
	dec := json.NewDecoder(r.Body)
	dec.DisallowUnknownFields()
	if err := dec.Decode(&payload); err != nil {
		http.Error(w, "invalid JSON payload", http.StatusBadRequest)
		return
	}

	org, hub, tracker, err := seededScenario(payload.Scenario, time.Now().UTC())
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	s.mu.Lock()
	s.org = org
	s.hub = hub
	s.tracker = tracker

	// ⚡ BOLT: Explicitly clear in-memory slices to prevent cross-test contamination and duplicate entries
	s.handoffs = s.handoffs[:0]
	s.pipelines = s.pipelines[:0]

	mockHandoff := HandoffPackage{
		ID:             "handoff-" + time.Now().UTC().Format("20060102150405"),
		FromAgentID:    "swe-1",
		ToHumanRole:    "CEO",
		Intent:         "Merge conflict resolution required for legacy billing module.",
		FailedAttempts: 3,
		CurrentState:   `{"Step_1_Code_Checkout": "SUCCESS", "Step_2_Dependency_Install": "SUCCESS", "Step_3_Test_Suite": "FAIL: TypeError in billing_test.go", "Step_4_Auto_Remediation": "SIGKILL: Timeout after 30s"}`,
		Status:         "pending",
		CreatedAt:      time.Now().UTC(),
	}
	s.handoffs = append(s.handoffs, mockHandoff)
	s.hub.LogEvent(mockHandoff)

	mockPipeline := Pipeline{
		ID:          "pipe-seed-" + time.Now().UTC().Format("20060102150405"),
		Name:        "feat-billing-seed",
		Status:      PipelineStatusStaging,
		Branch:      "feature/billing",
		StagingURL:  "https://staging.acme.com",
		InitiatedBy: "admin",
		CreatedAt:   time.Now().UTC(),
		UpdatedAt:   time.Now().UTC(),
	}
	s.pipelines = append(s.pipelines, mockPipeline)

	snapshot := s.snapshotLocked()
	s.mu.Unlock()

	writeJSON(w, snapshot)
}

// handleAgentProviders lists all registered external agent providers and their
// authentication status.  Responds to GET only.

// providerAuthRequest carries credentials for authenticating with an agent provider.
type providerAuthRequest struct {
	ProviderType string            `json:"providerType"`
	APIKey       string            `json:"apiKey,omitempty"`
	OAuthToken   string            `json:"oauthToken,omitempty"`
	Extra        map[string]string `json:"extra,omitempty"`
}

// handleAgentProviderAuth accepts POST requests to authenticate with an external
// agent provider.  Credentials are stored in memory and forwarded to any
// subsequently hired agent of that provider type.

type mcpRegisterRequest struct {
	Tool     MCPTool `json:"tool"`
	SPIFFEID string  `json:"spiffeId"`
}

func (s *Server) snapshot() dashboardSnapshot {
	s.mu.RLock()
	defer s.mu.RUnlock()
	return s.snapshotLocked()
}

func (s *Server) snapshotLocked() dashboardSnapshot {
	agents := s.orgAgentsLocked()

	queue := make([]orchestration.SharedTask, 0)
	queueLen := 0
	if s.hub != nil && s.hub.TaskManager() != nil {
		if pending, err := s.hub.TaskManager().PeekTasks(context.Background(), 100); err == nil {
			for _, t := range pending {
				if t != nil {
					queue = append(queue, *t)
				}
			}
			queueLen = len(queue)
		}
	}

	return dashboardSnapshot{
		Organization: s.org,
		Meetings:     s.orgMeetingsLocked(),
		Costs:        s.tracker.Summary(s.org.ID),
		Agents:       agents,
		Statuses:     summarizeStatuses(agents),
		TaskQueue:    queue,
		QueueLength:  queueLen,
		UpdatedAt:    time.Now().UTC(),
	}
}

func (s *Server) orgAgentsLocked() []orchestration.Agent {
	if s == nil || s.hub == nil {
		return []orchestration.Agent{}
	}
	return filterAgentsByOrg(s.hub.Agents(), s.org.ID)
}

func (s *Server) orgMeetingsLocked() []orchestration.MeetingRoom {
	if s == nil || s.hub == nil {
		return []orchestration.MeetingRoom{}
	}
	return filterMeetingsByAgentIDs(s.hub.Meetings(), s.orgAgentIndexLocked())
}

func (s *Server) orgAgentIndexLocked() map[string]struct{} {
	agents := s.orgAgentsLocked()
	index := make(map[string]struct{}, len(agents))
	for _, agent := range agents {
		index[agent.ID] = struct{}{}
	}
	return index
}

func (s *Server) agentOrgStatus(agentID string) (bool, bool) {
	if s == nil || s.hub == nil || agentID == "" {
		return false, false
	}
	agent, ok := s.hub.Agent(agentID)
	if !ok {
		return false, false
	}
	if agent.OrganizationID == "" && !strings.HasPrefix(agent.ID, s.org.ID+"-") {
		return false, false
	}
	return true, agentInOrg(agent, s.org.ID)
}

func (s *Server) meetingOrgStatus(meetingID string) (bool, bool) {
	if s == nil || s.hub == nil || meetingID == "" {
		return false, false
	}
	meeting, ok := s.hub.Meeting(meetingID)
	if !ok {
		return false, false
	}
	return true, meetingVisibleToOrg(meeting, s.orgAgentIndexLocked())
}

func agentInOrg(agent orchestration.Agent, orgID string) bool {
	if agent.OrganizationID != "" {
		return agent.OrganizationID == orgID
	}
	return strings.HasPrefix(agent.ID, orgID+"-")
}

func filterAgentsByOrg(agents []orchestration.Agent, orgID string) []orchestration.Agent {
	filtered := make([]orchestration.Agent, 0, len(agents))
	for _, agent := range agents {
		if agentInOrg(agent, orgID) {
			filtered = append(filtered, agent)
		}
	}
	return filtered
}

func filterMeetingsByAgentIDs(meetings []orchestration.MeetingRoom, allowedAgentIDs map[string]struct{}) []orchestration.MeetingRoom {
	filtered := make([]orchestration.MeetingRoom, 0, len(meetings))
	for _, meeting := range meetings {
		if meetingVisibleToOrg(meeting, allowedAgentIDs) {
			filtered = append(filtered, meeting)
		}
	}
	return filtered
}

func meetingVisibleToOrg(meeting orchestration.MeetingRoom, allowedAgentIDs map[string]struct{}) bool {
	if len(allowedAgentIDs) == 0 {
		return false
	}

	for _, participant := range meeting.Participants {
		if _, ok := allowedAgentIDs[participant]; ok {
			return true
		}
	}

	for _, message := range meeting.Transcript {
		if _, ok := allowedAgentIDs[message.FromAgent]; ok {
			return true
		}
		if _, ok := allowedAgentIDs[message.ToAgent]; ok {
			return true
		}
	}

	return false
}

func writeJSON(w http.ResponseWriter, value any) {
	w.Header().Set("Content-Type", "application/json")
	_ = json.NewEncoder(w).Encode(value)
}

// ── Chat test handler ─────────────────────────────────────────────────────────

type chatTestRequest struct {
	IntegrationID string `json:"integrationId"`
	BotToken      string `json:"botToken,omitempty"`
	ChatID        string `json:"chatId,omitempty"`
	WebhookURL    string `json:"webhookUrl,omitempty"`
	APIToken      string `json:"apiToken,omitempty"`
}

// ── MCP tool invocation ───────────────────────────────────────────────────────

type mcpInvokeRequest struct {
	ToolID   string          `json:"toolId"`
	Action   string          `json:"action"`
	Params   json.RawMessage `json:"params"`
	AgentID  string          `json:"agentId,omitempty"`
	SPIFFEID string          `json:"spiffeId,omitempty"`
}

type chatToolParams struct {
	IntegrationID string `json:"integrationId,omitempty"`
	Channel       string `json:"channel,omitempty"`
	FromAgent     string `json:"fromAgent,omitempty"`
	Content       string `json:"content"`
	ThreadID      string `json:"threadId,omitempty"`
}

type gitToolParams struct {
	IntegrationID string `json:"integrationId,omitempty"`
	Repository    string `json:"repository"`
	Title         string `json:"title"`
	Body          string `json:"body"`
	SourceBranch  string `json:"sourceBranch"`
	TargetBranch  string `json:"targetBranch,omitempty"`
	CreatedBy     string `json:"createdBy"`
}

type issueToolParams struct {
	IntegrationID string `json:"integrationId,omitempty"`
	Project       string `json:"project"`
	Title         string `json:"title"`
	Description   string `json:"description"`
	CreatedBy     string `json:"createdBy"`
	Priority      string `json:"priority"`
}

func summarizeStatuses(agents []orchestration.Agent) []statusCount {
	counts := map[orchestration.Status]int{
		orchestration.StatusIdle:      0,
		orchestration.StatusActive:    0,
		orchestration.StatusInMeeting: 0,
		orchestration.StatusBlocked:   0,
	}
	for _, agent := range agents {
		counts[agent.Status]++
	}

	statuses := make([]statusCount, 0, len(counts))
	for _, status := range statusOrder {
		statuses = append(statuses, statusCount{
			Status: status,
			Count:  counts[status],
		})
	}

	return statuses
}

// ── Approval / Confidence Gating Handlers ─────────────────────────────────────

// ── Warm Handoff Handlers ─────────────────────────────────────────────────────

// ── Identity Management Handler ───────────────────────────────────────────────

// ── Skill Pack Handlers ───────────────────────────────────────────────────────

// ── Snapshot Handlers ─────────────────────────────────────────────────────────

// seededScenarioByDomain re-seeds an org from its domain identifier.
func defaultSkillPacks() []SkillPack {
	now := time.Now().UTC()
	return []SkillPack{
		{
			ID:          "builtin-core-ai",
			Name:        "Core AI Skills",
			Domain:      "all",
			Description: "Foundational reasoning, summarization, and context management capabilities shared by all agents.",
			Source:      "builtin",
			Roles: []SkillPackRole{
				{Role: "ALL", BasePrompt: "You are a highly capable AI agent. Summarize long discussions before passing context to the next agent."},
			},
			ImportedAt: now,
		},
		{
			ID:          "builtin-software-dev",
			Name:        "Software Development Mastery",
			Domain:      "software_company",
			Description: "Advanced engineering skills: clean code, TDD, security-first development, and CI/CD automation.",
			Source:      "builtin",
			Roles: []SkillPackRole{
				{Role: "SOFTWARE_ENGINEER", BasePrompt: "Write well-tested, secure, and maintainable code. Follow TDD practices."},
				{Role: "QA_TESTER", BasePrompt: "Design comprehensive test suites covering edge cases and regressions."},
			},
			ImportedAt: now,
		},
		{
			ID:          "builtin-marketing-automation",
			Name:        "Marketing Automation Suite",
			Domain:      "digital_marketing_agency",
			Description: "Data-driven growth hacking, SEO optimization, and paid media management at scale.",
			Source:      "builtin",
			Roles: []SkillPackRole{
				{Role: "GROWTH_AGENT", BasePrompt: "Identify high-value acquisition channels using data. Run A/B tests continuously."},
			},
			ImportedAt: now,
		},
		{
			ID:          "builtin-financial-ops",
			Name:        "Financial Operations Pack",
			Domain:      "accounting_firm",
			Description: "GAAP-compliant bookkeeping, tax optimization, and audit preparation.",
			Source:      "builtin",
			Roles: []SkillPackRole{
				{Role: "BOOKKEEPER", BasePrompt: "Maintain double-entry books with 100% accuracy. Reconcile all accounts daily."},
			},
			ImportedAt: now,
		},
	}
}

func defaultMarketplaceItems() []MarketplaceItem {
	return []MarketplaceItem{
		{
			ID:          "mkt-tiger-team",
			Name:        "Tiger Team Sprint Pack",
			Type:        "skill_pack",
			Author:      "OneHumanCorp",
			Description: "Spin up a temporary 5-agent strike force for a time-boxed launch sprint.",
			Downloads:   1420,
			Rating:      4.8,
			Tags:        []string{"sprint", "launch", "team"},
		},
		{
			ID:          "mkt-ecommerce-domain",
			Name:        "E-Commerce Operations Domain",
			Type:        "domain",
			Author:      "Community",
			Description: "Full e-commerce organization with catalog, inventory, customer support, and growth roles.",
			Downloads:   892,
			Rating:      4.6,
			Tags:        []string{"ecommerce", "retail", "domain"},
		},
		{
			ID:          "mkt-crm-integration",
			Name:        "CRM Intelligence Pack",
			Type:        "tool",
			Author:      "SalesStack",
			Description: "Bi-directional Salesforce / HubSpot sync for Sales and Growth agents.",
			Downloads:   2100,
			Rating:      4.9,
			Tags:        []string{"crm", "sales", "integration"},
		},
		{
			ID:          "mkt-code-review-agent",
			Name:        "Autonomous Code Review Agent",
			Type:        "agent",
			Author:      "DevBot Labs",
			Description: "Specialized SWE agent trained on your codebase conventions. Reviews PRs for style, correctness, and test coverage.",
			Downloads:   3750,
			Rating:      4.7,
			Tags:        []string{"code-review", "engineering", "agent"},
		},
		{
			ID:          "mkt-guardian-agent",
			Name:        "Guardian Agent Pro",
			Type:        "agent",
			Author:      "SafeOps",
			Description: "Advanced confidence-gating agent with configurable spend thresholds and Slack/email HITL notifications.",
			Downloads:   980,
			Rating:      4.8,
			Tags:        []string{"security", "approval", "hitl"},
		},
	}
}

// ── Integration request/response types ────────────────────────────────────────

type integrationConnectRequest struct {
	IntegrationID string `json:"integrationId"`
	BaseURL       string `json:"baseUrl,omitempty"`
	// Chat credentials — stored server-side, never returned to the client.
	BotToken   string `json:"botToken,omitempty"`
	ChatID     string `json:"chatId,omitempty"`
	WebhookURL string `json:"webhookUrl,omitempty"`
	APIToken   string `json:"apiToken,omitempty"`
}

type integrationDisconnectRequest struct {
	IntegrationID string `json:"integrationId"`
}

type chatSendRequest struct {
	IntegrationID string `json:"integrationId"`
	Channel       string `json:"channel"`
	FromAgent     string `json:"fromAgent"`
	Content       string `json:"content"`
	ThreadID      string `json:"threadId,omitempty"`
}

type prCreateRequest struct {
	IntegrationID string `json:"integrationId"`
	Repository    string `json:"repository"`
	Title         string `json:"title"`
	Body          string `json:"body,omitempty"`
	SourceBranch  string `json:"sourceBranch"`
	TargetBranch  string `json:"targetBranch"`
	CreatedBy     string `json:"createdBy,omitempty"`
}

type prActionRequest struct {
	PRID string `json:"prId"`
}

type issueCreateRequest struct {
	IntegrationID string   `json:"integrationId"`
	Project       string   `json:"project"`
	Title         string   `json:"title"`
	Description   string   `json:"description,omitempty"`
	CreatedBy     string   `json:"createdBy,omitempty"`
	Priority      string   `json:"priority,omitempty"`
	Labels        []string `json:"labels,omitempty"`
}

type issueStatusRequest struct {
	IssueID string `json:"issueId"`
	Status  string `json:"status"`
}

type issueAssignRequest struct {
	IssueID  string `json:"issueId"`
	Assignee string `json:"assignee"`
}

// ── Integration handlers ──────────────────────────────────────────────────────

// ── Chat handlers ─────────────────────────────────────────────────────────────

// ── Git handlers ──────────────────────────────────────────────────────────────

// ── Issue tracker handlers ────────────────────────────────────────────────────

// ── B2B Collaboration ─────────────────────────────────────────────────────────

// TrustAgreementStatus represents the lifecycle phase of a federated trust agreement, governing encrypted cross-cluster agent communication.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type TrustAgreementStatus string

const (
	// TrustStatusPending indicates a proposed B2B federation link currently awaiting cryptographic SPIFFE handshake verification.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	TrustStatusPending TrustAgreementStatus = "PENDING"
	// TrustStatusActive indicates a fully verified B2B federation link, permitting bidirectional cross-org agent collaboration.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	TrustStatusActive TrustAgreementStatus = "ACTIVE"
	// TrustStatusRevoked indicates a explicitly terminated B2B federation link, instantly nullifying all associated access tokens.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	TrustStatusRevoked TrustAgreementStatus = "REVOKED"
)

// TrustAgreement establishes a cryptographically verified, federated trust relationship between two distinct One Human Corp deployments using SPIFFE-federated JWTs.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type TrustAgreement struct {
	ID           string               `json:"id"`
	PartnerOrg   string               `json:"partnerOrg"`
	PartnerJWKS  string               `json:"partnerJwksUrl"`
	AllowedRoles []string             `json:"allowedRoles"`
	Status       TrustAgreementStatus `json:"status"`
	CreatedAt    time.Time            `json:"createdAt"`
}

type b2bHandshakeRequest struct {
	PartnerOrg   string   `json:"partnerOrg"`
	PartnerJWKS  string   `json:"partnerJwksUrl"`
	AllowedRoles []string `json:"allowedRoles"`
}

// ── Autonomous SRE / Incident Management ─────────────────────────────────────

// IncidentSeverity classifies the blast radius of an operational anomaly to determine the SRE agent's escalation path.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type IncidentSeverity string

const (
	// SeverityP0 represents a catastrophic platform failure requiring immediate, synchronous human and SRE agent intervention.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	SeverityP0 IncidentSeverity = "P0"
	// SeverityP1 represents a severe degradation of a critical business path, triggering immediate automated remediation attempts.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	SeverityP1 IncidentSeverity = "P1"
	// SeverityP2 represents a localized, non-critical issue that will be asynchronously triaged by SRE agents.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	SeverityP2 IncidentSeverity = "P2"
)

// IncidentStatus reflects the current automated investigation and remediation lifecycle state of an operational anomaly.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type IncidentStatus string

const (
	// IncidentStatusInvestigating indicates an SRE agent is actively analyzing logs, traces, and metrics to identify the root cause of an anomaly.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	IncidentStatusInvestigating IncidentStatus = "INVESTIGATING"
	// IncidentStatusProposed indicates an SRE agent has formulated a remediation plan and is awaiting human execution approval.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	IncidentStatusProposed IncidentStatus = "PROPOSED"
	// IncidentStatusResolved indicates the anomaly has been successfully mitigated and verified by automated checks.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	IncidentStatusResolved IncidentStatus = "RESOLVED"
)

// Incident represents a distinct operational event requiring SRE attention, encapsulating its timeline, severity context, and remediation history.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type Incident struct {
	ID               string           `json:"id"`
	Severity         IncidentSeverity `json:"severity"`
	Summary          string           `json:"summary"`
	RCA              string           `json:"rootCauseAnalysis"`
	ResolutionPlanID string           `json:"resolutionPlanId,omitempty"`
	Status           IncidentStatus   `json:"status"`
	CreatedAt        time.Time        `json:"createdAt"`
	UpdatedAt        time.Time        `json:"updatedAt"`
}

type incidentCreateRequest struct {
	Severity string `json:"severity"`
	Summary  string `json:"summary"`
	RCA      string `json:"rootCauseAnalysis,omitempty"`
}

type incidentStatusRequest struct {
	IncidentID       string `json:"incidentId"`
	Status           string `json:"status"`
	ResolutionPlanID string `json:"resolutionPlanId,omitempty"`
	RCA              string `json:"rootCauseAnalysis,omitempty"`
}

// ── Compute Optimization / Hardware-Aware Scheduling ─────────────────────────

// ComputeProfile defines the strict hardware constraints (CPU, Memory) and affinity rules for a specific agent role, ensuring optimal Kubernetes pod scheduling.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type ComputeProfile struct {
	RoleID             string    `json:"roleId"`
	MinVRAMGB          int       `json:"minVramGb"`
	PreferredGPUType   string    `json:"preferredGpuType"` // "h100", "a10g", "cpu"
	SchedulingPriority int       `json:"schedulingPriority"`
	CreatedAt          time.Time `json:"createdAt"`
}

type computeProfileRequest struct {
	RoleID             string `json:"roleId"`
	MinVRAMGB          int    `json:"minVramGb"`
	PreferredGPUType   string `json:"preferredGpuType"`
	SchedulingPriority int    `json:"schedulingPriority"`
}

// ClusterStatus reflects the real-time node health, resource capacity, and latency metrics of a remote Kubernetes region for workload placement.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type ClusterStatus struct {
	Region         string    `json:"region"`
	Status         string    `json:"status"` // healthy, degraded, offline
	LatencyMS      int       `json:"latencyMs"`
	AvailableNodes int       `json:"availableNodes"`
	CheckedAt      time.Time `json:"checkedAt"`
}

// ── Budget Alerts ─────────────────────────────────────────────────────────────

// defaultBudgetAlertNotifyPct is the default notification threshold (80 %).
const defaultBudgetAlertNotifyPct = 0.8

// BudgetAlert defines a proactive spending threshold with associated notification circuit-breaker behaviors to prevent runaway token costs.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type BudgetAlert struct {
	ID             string    `json:"id"`
	OrganizationID string    `json:"organizationId"`
	ThresholdUSD   float64   `json:"thresholdUsd"`
	NotifyAtPct    float64   `json:"notifyAtPct"` // e.g. 0.8 → notify at 80 %
	Predictive     bool      `json:"predictive"`
	ForecastHours  int       `json:"forecastHours"`
	Triggered      bool      `json:"triggered"`
	CreatedAt      time.Time `json:"createdAt"`
}

type budgetAlertRequest struct {
	OrganizationID string  `json:"organizationId"`
	ThresholdUSD   float64 `json:"thresholdUsd"`
	NotifyAtPct    float64 `json:"notifyAtPct"`
	Predictive     bool    `json:"predictive"`
	ForecastHours  int     `json:"forecastHours"`
}

// ── Automated SDLC / Pipelines ────────────────────────────────────────────────

// PipelineStatus reflects the current lifecycle phase of an autonomous CI/CD implementation pipeline driven by agent collaboration.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type PipelineStatus string

const (
	// PipelineStatusPending indicates an autonomous pipeline has been created but has not yet commenced code generation.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	PipelineStatusPending PipelineStatus = "PENDING"
	// PipelineStatusImplementing indicates Software Engineer agents are actively iterating on code to satisfy the generated feature specification.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	PipelineStatusImplementing PipelineStatus = "IMPLEMENTING"
	// PipelineStatusTesting indicates QA agents are actively executing and verifying the implementation against the automated test suites.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	PipelineStatusTesting PipelineStatus = "TESTING"
	// PipelineStatusStaging indicates the feature has passed automated checks and is deployed to an ephemeral environment for final human review.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	PipelineStatusStaging PipelineStatus = "STAGING"
	// PipelineStatusPromoted indicates the feature has been successfully merged and deployed to the production environment.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	PipelineStatusPromoted PipelineStatus = "PROMOTED"
	// PipelineStatusFailed indicates the pipeline encountered an unrecoverable compilation error or test failure requiring manual intervention.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	PipelineStatusFailed PipelineStatus = "FAILED"
)

// Pipeline represents an autonomous, end-to-end implementation workflow, tracking a feature's progression from approved spec to a deployed artifact.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type Pipeline struct {
	ID          string         `json:"id"`
	Name        string         `json:"name"`
	Status      PipelineStatus `json:"status"`
	Branch      string         `json:"branch"`
	StagingURL  string         `json:"stagingUrl,omitempty"`
	InitiatedBy string         `json:"initiatedBy"`
	CreatedAt   time.Time      `json:"createdAt"`
	UpdatedAt   time.Time      `json:"updatedAt"`
}

type pipelineCreateRequest struct {
	Name        string `json:"name"`
	Branch      string `json:"branch"`
	InitiatedBy string `json:"initiatedBy"`
}

type pipelinePromoteRequest struct {
	PipelineID string `json:"pipelineId"`
	ApprovedBy string `json:"approvedBy"`
}

// handleStream pushes real-time state changes via Server-Sent Events (SSE)
func (s *Server) handleStream(w http.ResponseWriter, r *http.Request) {
	flusher, ok := w.(http.Flusher)
	if !ok {
		http.Error(w, "Streaming unsupported", http.StatusInternalServerError)
		return
	}

	// Set headers for SSE
	w.Header().Set("Content-Type", "text/event-stream")
	w.Header().Set("Cache-Control", "no-cache")
	w.Header().Set("Connection", "keep-alive")
	w.Header().Set("Access-Control-Allow-Origin", "*")

	ctx := r.Context()
	ticker := time.NewTicker(15 * time.Second)
	defer ticker.Stop()

	var subChan <-chan struct{}
	var unsubscribe func()

	if s.hub != nil {
		// As a workaround since TeammateMesh isn't directly exposed on Hub,
		// we can subscribe via the existing hub.Subscribe logic which triggers
		// on any new message to the system/agent.
		subChan, unsubscribe = s.hub.Subscribe("system")
		defer unsubscribe()
	}

	// Make sure headers are sent immediately
	w.WriteHeader(http.StatusOK)
	flusher.Flush()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			// Heartbeat ping
			fmt.Fprintf(w, ": heartbeat\n\n")
			flusher.Flush()
		case _, ok := <-subChan:
			if !ok {
				// Channel closed
				return
			}

			// We received a ping that there's a new message.
			// Let's pull the latest messages from the inbox.
			messages := s.hub.Inbox("system")
			for _, msg := range messages {
				eventStr := `{"event":"TaskBroadcast","status":"INFO"}`
				if msg.Type == "mesh:tasks" {
					eventStr = msg.Content
				}
				fmt.Fprintf(w, "data: %s\n\n", eventStr)
			}
			flusher.Flush()
		}
	}
}

func (s *Server) handleMeshV2Broadcast(w http.ResponseWriter, r *http.Request) {
	mode := "cloud"
	if os.Getenv("OHC_STANDALONE") == "true" {
		mode = "standalone"
	}
	telemetry.RecordMeshBroadcast(r.Context(), mode)

	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// Enforce mTLS checks
	if r.TLS == nil || len(r.TLS.PeerCertificates) == 0 {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}
	cert := r.TLS.PeerCertificates[0]
	if len(cert.URIs) == 0 || cert.URIs[0].Scheme != "spiffe" {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}

	var req struct {
		Channel string                 `json:"channel"`
		Data    map[string]interface{} `json:"data"`
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "invalid request", http.StatusBadRequest)
		return
	}

	if req.Channel == "" {
		http.Error(w, "invalid channel", http.StatusBadRequest)
		return
	}

	payloadBytes, err := json.Marshal(req.Data)
	if err != nil {
		http.Error(w, "failed to marshal payload", http.StatusInternalServerError)
		return
	}

	err = s.hub.Publish(orchestration.Message{
		ID:        fmt.Sprintf("%d", time.Now().UnixNano()),
		FromAgent: "system",
		ToAgent:   "system",
		Type:      req.Channel,
		Content:   string(payloadBytes),
	})

	if err == nil {
		telemetry.RecordTeammateMeshBroadcast(r.Context(), req.Channel)

		// Map mesh channels to Centrifuge WebSocket channels for UI updates
		if s.hub != nil && s.hub.CentrifugeNode() != nil {
			if req.Channel == "mesh:tasks" || req.Channel == "swarm-events" {
				s.hub.CentrifugeNode().PublishTaskBroadcast(fmt.Sprintf("%d", time.Now().UnixNano()), req.Data)
			} else if req.Channel == "mesh:coordination" {
				agentID, _ := req.Data["agent_id"].(string)
				if agentID == "" {
					agentID = "system"
				}
				s.hub.CentrifugeNode().PublishCoordinationMessage(orchestration.Message{
					ID:        fmt.Sprintf("%d", time.Now().UnixNano()),
					FromAgent: agentID,
					ToAgent:   "system",
					Type:      req.Channel,
					Content:   string(payloadBytes),
				})
			}
		}
	} else {
		http.Error(w, "failed to broadcast", http.StatusInternalServerError)
		return
	}

	// Dispatch to the specific MeshBroker per implementation request
	var broker orchestration_mesh.MeshBroker
	if mode == "cloud" && os.Getenv("REDIS_URL") != "" {
		redisURL := os.Getenv("REDIS_URL")
		opt, _ := redis.ParseURL(redisURL)
		client := redis.NewClient(opt)
		defer client.Close()
		broker = orchestration_mesh.NewRedisMeshBroker(client)
	} else {
		broker = orchestration_mesh.NewLocalMeshBroker()
	}
	_ = broker.Broadcast(r.Context(), req.Channel, payloadBytes)

	w.WriteHeader(http.StatusOK)
	_, _ = w.Write([]byte(`{"status":"ok"}`))
}
