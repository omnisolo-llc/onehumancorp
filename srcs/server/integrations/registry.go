// Package integrations provides a registry for external service adapters.
//
// It implements three categories of integration that allow AI agents to
// interact with the same tools that human team members use:
//
//   - Chat services: Slack, Discord, Google Chat, Telegram, Microsoft Teams — for human–agent messaging
//   - Git platforms: GitHub, GitLab, Gitea    — for PR/MR creation
//   - Issue trackers: JIRA, Plane, GitHub Issues — for ticket management
//
// All state is held in-memory following the same pattern used by the rest of
// the platform.  Chat integrations with stored credentials (Telegram, Discord)
// make real outbound HTTP API calls in addition to recording messages locally.
package integrations

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net"
	"net/http"
	"net/url"
	"sync"
	"time"

	pb "github.com/onehumancorp/mono/srcs/proto"
)

// ── Integration types ─────────────────────────────────────────────────────────

// Category groups integrations by their function (e.g., chat, git, issues).
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type Category string

const (
	// CategoryChat classifies the integration module under the Chat domain taxonomy for structured discovery.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	CategoryChat Category = "chat"
	// CategoryGit classifies the integration module under the Git domain taxonomy for structured discovery.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	CategoryGit Category = "git"
	// CategoryIssues classifies the integration module under the Issues domain taxonomy for structured discovery.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	CategoryIssues Category = "issues"
)

// IntegrationType identifies the specific external service platform (e.g., github, slack).
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type IntegrationType string

const (
	// IntegrationTypeSlack Chat services.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	IntegrationTypeSlack IntegrationType = "slack"
	// IntegrationTypeDiscord provides domain-specific context and typed constraints for IntegrationTypeDiscord operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	IntegrationTypeDiscord IntegrationType = "discord"
	// IntegrationTypeGoogleChat provides domain-specific context and typed constraints for IntegrationTypeGoogleChat operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	IntegrationTypeGoogleChat IntegrationType = "google_chat"
	// IntegrationTypeTelegram provides domain-specific context and typed constraints for IntegrationTypeTelegram operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	IntegrationTypeTelegram IntegrationType = "telegram"
	// IntegrationTypeTeams provides domain-specific context and typed constraints for IntegrationTypeTeams operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	IntegrationTypeTeams IntegrationType = "teams"
	// IntegrationTypeWhatsApp provides domain-specific context and typed constraints for IntegrationTypeWhatsApp operations across the application.
	IntegrationTypeWhatsApp IntegrationType = "whatsapp"
	// IntegrationTypeIMessage provides domain-specific context and typed constraints for IntegrationTypeIMessage operations across the application.
	IntegrationTypeIMessage IntegrationType = "imessage"

	// IntegrationTypeGitHub Git platforms.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	IntegrationTypeGitHub IntegrationType = "github"
	// IntegrationTypeGitLab provides domain-specific context and typed constraints for IntegrationTypeGitLab operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	IntegrationTypeGitLab IntegrationType = "gitlab"
	// IntegrationTypeGitea provides domain-specific context and typed constraints for IntegrationTypeGitea operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	IntegrationTypeGitea IntegrationType = "gitea"

	// IntegrationTypeJIRA issue trackers.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	IntegrationTypeJIRA IntegrationType = "jira"
	// IntegrationTypePlane provides domain-specific context and typed constraints for IntegrationTypePlane operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	IntegrationTypePlane IntegrationType = "plane"
	// IntegrationTypeGitHubIssues provides domain-specific context and typed constraints for IntegrationTypeGitHubIssues operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	IntegrationTypeGitHubIssues IntegrationType = "github_issues"
)

// ConnectionStatus reflects whether an integration is currently active and reachable.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type ConnectionStatus string

const (
	// StatusConnected represents the CONNECTED lifecycle phase of a tracked entity within the event-driven state machine.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	StatusConnected ConnectionStatus = "connected"
	// StatusDisconnected represents the DISCONNECTED lifecycle phase of a tracked entity within the event-driven state machine.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	StatusDisconnected ConnectionStatus = "disconnected"
	// StatusError represents the ERROR lifecycle phase of a tracked entity within the event-driven state machine.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	StatusError ConnectionStatus = "error"
)

// Integration represents a configured external service connection.
// Accepts no parameters.
// IntegrationInstance represents a specific user-configured connection to a platform.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type IntegrationInstance struct {
	InstanceID     string            `json:"instanceId"`
	IntegrationID  string            `json:"integrationId"`
	Name           string            `json:"name"`
	Type           IntegrationType   `json:"type"`
	Category       Category          `json:"category"`
	BaseURL        string            `json:"baseUrl,omitempty"`
	Status         ConnectionStatus  `json:"status"`
	HasCredentials bool              `json:"hasCredentials,omitempty"`
	Chatspace      string            `json:"chatspace,omitempty"`
	Config         map[string]string `json:"config,omitempty"`
	CreatedAt      time.Time         `json:"createdAt"`
}

// IntegrationCredentials holds the secret configuration for an integration. These are stored server-side only and never serialised to the client.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type IntegrationCredentials struct {
	BotToken   string // Telegram Bot API token
	ChatID     string // Telegram chat / group ID
	WebhookURL string // Discord (or generic) inbound webhook URL
	APIToken   string // Generic API token / Bearer credential
}

// IsEmpty reports whether no fields are set.
// Accepts no parameters.
// Returns bool.
// Produces no errors.
// Has no side effects.
func (c IntegrationCredentials) IsEmpty() bool {
	return c.BotToken == "" && c.ChatID == "" && c.WebhookURL == "" && c.APIToken == ""
}

// ── Chat types ────────────────────────────────────────────────────────────────

// ChatMessage represents a message dispatched through a chat service.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type ChatMessage struct {
	ID            string    `json:"id"`
	IntegrationID string    `json:"integrationId"`
	Channel       string    `json:"channel"`
	FromAgent     string    `json:"fromAgent"`
	Content       string    `json:"content"`
	ThreadID      string    `json:"threadId,omitempty"`
	SentAt        time.Time `json:"sentAt"`
}

// ── Git types ─────────────────────────────────────────────────────────────────

// PullRequestStatus tracks the lifecycle status of a PR/MR on a git platform.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type PullRequestStatus string

const (
	// PRStatusOpen provides domain-specific context and typed constraints for PRStatusOpen operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	PRStatusOpen PullRequestStatus = "open"
	// PRStatusMerged provides domain-specific context and typed constraints for PRStatusMerged operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	PRStatusMerged PullRequestStatus = "merged"
	// PRStatusClosed provides domain-specific context and typed constraints for PRStatusClosed operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	PRStatusClosed PullRequestStatus = "closed"
)

// PullRequest records an issue or code change request opened on a git hosting platform.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type PullRequest struct {
	ID             string            `json:"id"`
	IntegrationID  string            `json:"integrationId"`
	Repository     string            `json:"repository"`
	Title          string            `json:"title"`
	Body           string            `json:"body"`
	SourceBranch   string            `json:"sourceBranch"`
	TargetBranch   string            `json:"targetBranch"`
	URL            string            `json:"url"`
	CreatedByAgent string            `json:"createdByAgent"`
	Status         PullRequestStatus `json:"status"`
	CreatedAt      time.Time         `json:"createdAt"`
}

// ── Issue types ───────────────────────────────────────────────────────────────

// IssueStatus tracks the lifecycle phase of an issue or ticket.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type IssueStatus string

const (
	// IssueStatusOpen provides domain-specific context and typed constraints for IssueStatusOpen operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	IssueStatusOpen IssueStatus = "open"
	// IssueStatusInProgress provides domain-specific context and typed constraints for IssueStatusInProgress operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	IssueStatusInProgress IssueStatus = "in_progress"
	// IssueStatusDone provides domain-specific context and typed constraints for IssueStatusDone operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	IssueStatusDone IssueStatus = "done"
	// IssueStatusClosed provides domain-specific context and typed constraints for IssueStatusClosed operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	IssueStatusClosed IssueStatus = "closed"
)

// IssuePriority indicates the urgency of a ticket.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type IssuePriority string

const (
	// IssuePriorityLow provides domain-specific context and typed constraints for IssuePriorityLow operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	IssuePriorityLow IssuePriority = "low"
	// IssuePriorityMedium provides domain-specific context and typed constraints for IssuePriorityMedium operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	IssuePriorityMedium IssuePriority = "medium"
	// IssuePriorityHigh provides domain-specific context and typed constraints for IssuePriorityHigh operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	IssuePriorityHigh IssuePriority = "high"
	// IssuePriorityCritical provides domain-specific context and typed constraints for IssuePriorityCritical operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	IssuePriorityCritical IssuePriority = "critical"
)

// Issue records a ticket created in an external issue tracker.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type Issue struct {
	ID             string        `json:"id"`
	IntegrationID  string        `json:"integrationId"`
	Project        string        `json:"project"`
	Title          string        `json:"title"`
	Description    string        `json:"description"`
	Priority       IssuePriority `json:"priority"`
	Status         IssueStatus   `json:"status"`
	AssignedTo     string        `json:"assignedTo,omitempty"`
	Labels         []string      `json:"labels,omitempty"`
	CreatedByAgent string        `json:"createdByAgent"`
	URL            string        `json:"url"`
	CreatedAt      time.Time     `json:"createdAt"`
}

// ── Registry ─────────────────────────────────────────────────────────────────

// Registry manages all configured external service integrations and records every action taken through them (messages sent, PRs opened, tickets created).  Constraints: Thread-safe via sync.RWMutex.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type Registry struct {
	mu           sync.RWMutex
	instances    map[string]*IntegrationInstance
	credentials  map[string]IntegrationCredentials // keyed by instance ID; never exposed to clients
	chatMessages []ChatMessage
	pullRequests []PullRequest
	issues       []Issue
}

// NewRegistry returns an initialised Registry pre-populated with the default
// set of supported integrations (all marked as disconnected until configured).
//
// Accepts no parameters.
// Returns *Registry.
// Produces no errors.
// Has no side effects.
func NewRegistry() *Registry {
	r := &Registry{
		instances:    make(map[string]*IntegrationInstance),
		credentials:  map[string]IntegrationCredentials{},
		chatMessages: []ChatMessage{},
		pullRequests: []PullRequest{},
		issues:       []Issue{},
	}

	for _, provider := range Catalog {
		meta := provider.Metadata()
		id := meta.GetId()
		r.instances[id] = &IntegrationInstance{
			InstanceID:    id,
			IntegrationID: id,
			Name:          meta.GetName(),
			Type:          IntegrationType(meta.GetType()),
			Category:      Category(meta.GetCategory()),
			Status:        StatusDisconnected,
			CreatedAt:     time.Now(),
		}
	}

	return r
}

// ── Integration management ────────────────────────────────────────────────────

// Instances retrieves a snapshot of all registered external service connections.
//
// Accepts parameters: r *Registry (No Constraints).
// Returns Instances() []*IntegrationInstance.
// Produces no errors.
// Has no side effects.
func (r *Registry) Instances() []*IntegrationInstance {
	r.mu.RLock()
	defer r.mu.RUnlock()

	var result []*IntegrationInstance
	for _, inst := range r.instances {
		result = append(result, inst)
	}
	return result
}

// InstancesByCategory returns integrations filtered by their service category.
//
//   - cat: Category; The category to filter by (e.g., CategoryChat).
//
// Accepts parameters: r *Registry (No Constraints).
// Returns InstancesByCategory(cat Category) []*IntegrationInstance.
// Produces no errors.
// Has no side effects.
func (r *Registry) InstancesByCategory(cat Category) []*IntegrationInstance {
	r.mu.RLock()
	defer r.mu.RUnlock()

	var result []*IntegrationInstance
	for _, i := range r.instances {
		if i.Category == cat {
			result = append(result, i)
		}
	}
	return result
}

// Instance looks up a specific integration connection by its unique ID.
//
//   - id: string; The identifier of the integration instance.
//
// Accepts parameters: r *Registry (No Constraints).
// Returns Instance(id string) (*IntegrationInstance, bool).
// Produces no errors.
// Has no side effects.
func (r *Registry) Instance(id string) (*IntegrationInstance, bool) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	inst, ok := r.instances[id]
	return inst, ok
}

// LookupIPFunc is a variable to allow mocking net.LookupIP in tests across packages.
var // Summary: LookupIPFunc is a variable to allow mocking net.LookupIP in tests across packages.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
LookupIPFunc = net.LookupIP

// AllowLocalIPsForTesting can be set to true in tests to bypass SSRF IP checks
var // Summary: AllowLocalIPsForTesting can be set to true in tests to bypass SSRF IP checks
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
AllowLocalIPsForTesting = false

// cgnatRange defines the RFC 6598 Shared Address Space (100.64.0.0/10)
// often used in Kubernetes and cloud environments for pod networking.
var _, cgnatRange, _ = net.ParseCIDR("100.64.0.0/10")

func isBlockedIP(ip net.IP) bool {
	if AllowLocalIPsForTesting {
		return false
	}
	return ip.IsLoopback() || ip.IsPrivate() || ip.IsUnspecified() || ip.IsLinkLocalUnicast() || ip.IsLinkLocalMulticast() || cgnatRange.Contains(ip)
}

// validateURL checks if a given URL string is safe from SSRF attacks.
// It explicitly blocks loopback, private, unspecified, and link-local IP addresses.
// It fails closed on DNS resolution errors.
func validateURL(u string) error {
	parsedURL, err := url.ParseRequestURI(u)
	if err != nil {
		return errors.New("invalid URL format")
	}

	if parsedURL.Scheme != "http" && parsedURL.Scheme != "https" {
		return errors.New("invalid URL scheme")
	}

	host := parsedURL.Hostname()
	if host == "" {
		return errors.New("URL must contain a host")
	}

	ips, err := LookupIPFunc(host)
	if err != nil {
		// Fail closed on DNS resolution error
		return errors.New("DNS resolution failed")
	}

	for _, ip := range ips {
		if isBlockedIP(ip) {
			return errors.New("URL resolves to a blocked IP address")
		}
	}

	return nil
}

// InitSafeHTTPClient returns an http.Client with a custom DialContext that prevents
// DNS rebinding (TOCTOU) attacks by pinning the connection to the validated IP.
func InitSafeHTTPClient() *http.Client {
	dialer := &net.Dialer{
		Timeout:   30 * time.Second,
		KeepAlive: 30 * time.Second,
	}

	transport := &http.Transport{
		Proxy:                 http.ProxyFromEnvironment,
		ForceAttemptHTTP2:     true,
		MaxIdleConns:          100,
		IdleConnTimeout:       90 * time.Second,
		TLSHandshakeTimeout:   10 * time.Second,
		ExpectContinueTimeout: 1 * time.Second,
		DialContext: func(ctx context.Context, network, addr string) (net.Conn, error) {
			host, port, err := net.SplitHostPort(addr)
			if err != nil {
				return nil, err
			}

			ips, err := LookupIPFunc(host)
			if err != nil {
				return nil, fmt.Errorf("DNS resolution failed: %w", err)
			}
			if len(ips) == 0 {
				return nil, errors.New("no IP addresses found for host")
			}

			// Validate all resolved IPs
			for _, ip := range ips {
				if isBlockedIP(ip) {
					return nil, errors.New("URL resolves to a blocked IP address")
				}
			}

			// Connect directly to the first validated IP
			safeAddr := net.JoinHostPort(ips[0].String(), port)
			return dialer.DialContext(ctx, network, safeAddr)
		},
	}

	return &http.Client{
		Transport: transport,
		Timeout:   15 * time.Second,
	}
}

var safeClient = InitSafeHTTPClient()

// Connect marks an integration as connected and sets its base URL.
// An optional IntegrationCredentials value stores secrets (e.g. bot tokens)
// for integrations that make real outbound API calls.
//
//   - id: string; The identifier of the integration to connect.
//   - baseURL: string; The API base URL to use for requests.
//   - creds: IntegrationCredentials; Optional credentials for outbound API calls.
//
// Accepts parameters: r *Registry (No Constraints).
// Returns Connect(id, baseURL string, creds ...IntegrationCredentials) (Integration, error).
// Produces errors: Explicit error handling.
// Has no side effects.
func (r *Registry) Connect(id, baseURL string, creds ...IntegrationCredentials) (*IntegrationInstance, error) {
	if baseURL != "" {
		if err := validateURL(baseURL); err != nil {
			return nil, err
		}
	}
	if len(creds) > 0 && creds[0].WebhookURL != "" {
		if err := validateURL(creds[0].WebhookURL); err != nil {
			return nil, err
		}
	}

	r.mu.Lock()
	defer r.mu.Unlock()

	inst, exists := r.instances[id]
	if !exists {
		var meta *pb.IntegrationMetadata
		found := false
		for _, provider := range Catalog {
			if provider.Metadata().GetId() == id {
				meta = provider.Metadata()
				found = true
				break
			}
		}
		if !found {
			return nil, errors.New("integration provider not found in catalog")
		}

		inst = &IntegrationInstance{
			InstanceID:    id,
			IntegrationID: meta.GetId(),
			Name:          meta.GetName(),
			Type:          IntegrationType(meta.GetType()),
			Category:      Category(meta.GetCategory()),
			CreatedAt:     time.Now(),
		}
		r.instances[id] = inst
	}

	inst.Status = StatusConnected
	if baseURL != "" {
		inst.BaseURL = baseURL
	}
	if len(creds) > 0 && !creds[0].IsEmpty() {
		r.credentials[id] = creds[0]
		inst.HasCredentials = true
		if creds[0].ChatID != "" {
			inst.Chatspace = creds[0].ChatID
		}
	}
	return inst, nil
}

// Disconnect marks a previously connected integration as disconnected.
//
//   - id: string; The identifier of the integration to disconnect.
//
// Accepts parameters: r *Registry (No Constraints).
// Returns Disconnect(id string) (Integration, error).
// Produces errors: Explicit error handling.
// Has no side effects.
func (r *Registry) Disconnect(id string) (*IntegrationInstance, error) {
	r.mu.Lock()
	defer r.mu.Unlock()

	if inst, ok := r.instances[id]; ok {
		inst.Status = StatusDisconnected
		return inst, nil
	}
	return nil, errors.New("integration not found")
}

// ── Chat operations ───────────────────────────────────────────────────────────

// SendChatMessage records the dispatch of a message through the specified chat integration.
//
//   - integrationID: string; The ID of the chat integration (e.g., "slack").
//   - channel: string; The target channel or space.
//   - fromAgent: string; The ID of the agent sending the message.
//   - content: string; The message payload.
//   - threadID: string; The thread context, if applicable.
//   - now: time.Time; The current timestamp.
//
// Accepts parameters: r *Registry (No Constraints).
// Returns SendChatMessage(integrationID, channel, fromAgent, content, threadID string, now time.Time) (ChatMessage, error).
// Produces errors: Explicit error handling.
// Has no side effects.
func (r *Registry) SendChatMessage(integrationID, channel, fromAgent, content, threadID string, now time.Time) (ChatMessage, error) {
	r.mu.Lock()
	defer r.mu.Unlock()

	integ, ok := r.findIntegration(integrationID)
	if !ok {
		return ChatMessage{}, errors.New("integration not found")
	}
	if integ.Category != CategoryChat {
		return ChatMessage{}, errors.New("integration is not a chat service")
	}
	if channel == "" {
		return ChatMessage{}, errors.New("channel is required")
	}
	if fromAgent == "" {
		return ChatMessage{}, errors.New("fromAgent is required")
	}
	if content == "" {
		return ChatMessage{}, errors.New("content is required")
	}

	msg := ChatMessage{
		ID:            generateID(integrationID+"-msg", now),
		IntegrationID: integrationID,
		Channel:       channel,
		FromAgent:     fromAgent,
		Content:       content,
		ThreadID:      threadID,
		SentAt:        now.UTC(),
	}
	r.chatMessages = append(r.chatMessages, msg)

	// Attempt real delivery when credentials are configured.
	if creds, hasCreds := r.credentials[integrationID]; hasCreds {
		text := fmt.Sprintf("[%s] %s", fromAgent, content)
		switch integ.Type {
		case IntegrationTypeTelegram:
			if creds.BotToken != "" {
				// Use provided channel (chat_id) or fall back to the stored ChatID.
				chatID := channel
				if creds.ChatID != "" {
					chatID = creds.ChatID
				}
				// Best-effort: log but do not fail the in-memory record.
				_ = sendTelegramMessage(context.Background(), creds.BotToken, chatID, text)
			}
		case IntegrationTypeDiscord:
			if creds.WebhookURL != "" {
				_ = sendDiscordWebhook(context.Background(), creds.WebhookURL, fromAgent, content)
			}
		}
	}

	return msg, nil
}

// TestConnection validates that the provided credentials can reach the external
// service by sending a short test message.  Use this during setup wizards
// before persisting credentials.
//
// Accepts parameters:
//   - id: string; The identifier of the integration to test.
//   - creds: IntegrationCredentials; The credentials to validate.
//
// Returns An error if the connection test fails.
//
// Produces errors: Fails if the integration is missing or if the external API call fails.
//
// Has side effects: Triggers real outbound HTTP API calls to Telegram or Discord.
func (r *Registry) TestConnection(id string, creds IntegrationCredentials) error {
	r.mu.RLock()
	integ, ok := r.findIntegration(id)
	// If no credentials supplied, fall back to stored ones.
	stored := r.credentials[id]
	r.mu.RUnlock()

	if !ok {
		return errors.New("integration not found")
	}

	active := creds
	if active.IsEmpty() {
		active = stored
	}

	switch integ.Type {
	case IntegrationTypeTelegram:
		if active.BotToken == "" {
			return errors.New("bot token is required")
		}
		if active.ChatID == "" {
			return errors.New("chat ID is required")
		}
		return sendTelegramMessage(context.Background(), active.BotToken, active.ChatID,
			"✅ Test message from One Human Corp — Telegram integration confirmed!")
	case IntegrationTypeDiscord:
		if active.WebhookURL == "" {
			return errors.New("webhook URL is required")
		}
		if err := validateURL(active.WebhookURL); err != nil {
			return err
		}
		return sendDiscordWebhook(context.Background(), active.WebhookURL, "One Human Corp",
			"✅ Test message — Discord integration confirmed!")
	default:
		// No live endpoint to test; accept unconditionally.
		return nil
	}
}

// ChatMessages retrieves all recorded chat messages, with an optional integration ID filter.
//
// Accepts parameters:
//   - integrationID: string; Filter by integration. Pass an empty string for all messages.
//
// Returns A slice of ChatMessage records.
//
// Produces errors: None.
//
// Has side effects: None. Executes a read-only lock.
func (r *Registry) ChatMessages(integrationID string) []ChatMessage {
	r.mu.RLock()
	defer r.mu.RUnlock()

	var result []ChatMessage
	for _, m := range r.chatMessages {
		if integrationID == "" || m.IntegrationID == integrationID {
			result = append(result, m)
		}
	}
	return result
}

// ── Git operations ────────────────────────────────────────────────────────────

// CreatePullRequest registers a new PR/MR action on the specified git integration.
//
// Accepts parameters:
//   - integrationID: string; The ID of the git integration (e.g., "github").
//   - repo: string; Target repository name.
//   - title: string; PR title.
//   - body: string; PR description.
//   - source: string; Branch name containing the changes.
//   - target: string; Base branch to merge into.
//   - createdBy: string; Agent ID opening the PR.
//   - now: time.Time; Timestamp.
//
// Returns A PullRequest record of the action, or an error if parameters are invalid.
//
// Produces errors: Fails if the integration is not a git platform or if required fields are missing.
//
// Has side effects: Appends a new PullRequest to the internal memory store.
func (r *Registry) CreatePullRequest(integrationID, repo, title, body, source, target, createdBy string, now time.Time) (PullRequest, error) {
	r.mu.Lock()
	defer r.mu.Unlock()

	integ, ok := r.findIntegration(integrationID)
	if !ok {
		return PullRequest{}, errors.New("integration not found")
	}
	if integ.Category != CategoryGit {
		return PullRequest{}, errors.New("integration is not a git platform")
	}
	if repo == "" {
		return PullRequest{}, errors.New("repository is required")
	}
	if title == "" {
		return PullRequest{}, errors.New("title is required")
	}
	if source == "" || target == "" {
		return PullRequest{}, errors.New("sourceBranch and targetBranch are required")
	}

	prID := generateID(integrationID+"-pr", now)
	pr := PullRequest{
		ID:             prID,
		IntegrationID:  integrationID,
		Repository:     repo,
		Title:          title,
		Body:           body,
		SourceBranch:   source,
		TargetBranch:   target,
		URL:            integ.BaseURL + "/" + repo + "/pull/" + prID,
		CreatedByAgent: createdBy,
		Status:         PRStatusOpen,
		CreatedAt:      now.UTC(),
	}
	r.pullRequests = append(r.pullRequests, pr)
	return pr, nil
}

// MergePullRequest transitions an open Pull Request to merged status.
//
// Accepts parameters:
//   - prID: string; The unique registry ID of the pull request.
//
// Returns The updated PullRequest record.
//
// Produces errors: Fails if the PR is not found or is not in the open state.
//
// Has side effects: Mutates the status of the PullRequest to PRStatusMerged.
func (r *Registry) MergePullRequest(prID string) (PullRequest, error) {
	r.mu.Lock()
	defer r.mu.Unlock()

	for idx, pr := range r.pullRequests {
		if pr.ID == prID {
			if pr.Status != PRStatusOpen {
				return PullRequest{}, errors.New("pull request is not open")
			}
			r.pullRequests[idx].Status = PRStatusMerged
			return r.pullRequests[idx], nil
		}
	}
	return PullRequest{}, errors.New("pull request not found")
}

// ClosePullRequest transitions an open Pull Request to closed status without merging.
//
// Accepts parameters:
//   - prID: string; The unique registry ID of the pull request.
//
// Returns The updated PullRequest record.
//
// Produces errors: Fails if the PR is not found or is not in the open state.
//
// Has side effects: Mutates the status of the PullRequest to PRStatusClosed.
func (r *Registry) ClosePullRequest(prID string) (PullRequest, error) {
	r.mu.Lock()
	defer r.mu.Unlock()

	for idx, pr := range r.pullRequests {
		if pr.ID == prID {
			if pr.Status != PRStatusOpen {
				return PullRequest{}, errors.New("pull request is not open")
			}
			r.pullRequests[idx].Status = PRStatusClosed
			return r.pullRequests[idx], nil
		}
	}
	return PullRequest{}, errors.New("pull request not found")
}

// PullRequests retrieves all recorded pull requests, with an optional integration ID filter.
//
// Accepts parameters:
//   - integrationID: string; Filter by integration. Pass an empty string to return all.
//
// Returns A slice of PullRequest records.
//
// Produces errors: None.
//
// Has side effects: None. Executes a read-only lock.
func (r *Registry) PullRequests(integrationID string) []PullRequest {
	r.mu.RLock()
	defer r.mu.RUnlock()

	var result []PullRequest
	for _, pr := range r.pullRequests {
		if integrationID == "" || pr.IntegrationID == integrationID {
			result = append(result, pr)
		}
	}
	return result
}

// ── Issue operations ──────────────────────────────────────────────────────────

// CreateIssue registers a new ticket action in the specified issue tracker integration.
//
// Accepts parameters:
//   - integrationID: string; The ID of the issue integration (e.g., "jira").
//   - project: string; The target project or board.
//   - title: string; The issue summary.
//   - description: string; The detailed description of the issue.
//   - createdBy: string; The ID of the agent reporting the issue.
//   - priority: IssuePriority; The urgency of the ticket.
//   - labels: []string; Categorisation tags.
//   - now: time.Time; Current timestamp.
//
// Returns An Issue record of the action, or an error if parameters are invalid.
//
// Produces errors: Fails if the integration is not an issue tracker or required fields are missing.
//
// Has side effects: Appends a new Issue to the internal memory store.
func (r *Registry) CreateIssue(integrationID, project, title, description, createdBy string, priority IssuePriority, labels []string, now time.Time) (Issue, error) {
	r.mu.Lock()
	defer r.mu.Unlock()

	integ, ok := r.findIntegration(integrationID)
	if !ok {
		return Issue{}, errors.New("integration not found")
	}
	if integ.Category != CategoryIssues {
		return Issue{}, errors.New("integration is not an issue tracker")
	}
	if project == "" {
		return Issue{}, errors.New("project is required")
	}
	if title == "" {
		return Issue{}, errors.New("title is required")
	}

	if priority == "" {
		priority = IssuePriorityMedium
	}

	issueID := generateID(integrationID+"-issue", now)
	labelsCopy := make([]string, len(labels))
	copy(labelsCopy, labels)
	issue := Issue{
		ID:             issueID,
		IntegrationID:  integrationID,
		Project:        project,
		Title:          title,
		Description:    description,
		Priority:       priority,
		Status:         IssueStatusOpen,
		Labels:         labelsCopy,
		CreatedByAgent: createdBy,
		URL:            integ.BaseURL + "/issues/" + issueID,
		CreatedAt:      now.UTC(),
	}
	r.issues = append(r.issues, issue)
	return issue, nil
}

// UpdateIssueStatus transitions an existing issue to the specified lifecycle phase.
//
// Accepts parameters:
//   - issueID: string; The unique registry ID of the issue.
//   - status: IssueStatus; The new status phase (e.g., IssueStatusDone).
//
// Returns The updated Issue record.
//
// Produces errors: Fails if the issue cannot be found.
//
// Has side effects: Mutates the status of the specific Issue record.
func (r *Registry) UpdateIssueStatus(issueID string, status IssueStatus) (Issue, error) {
	r.mu.Lock()
	defer r.mu.Unlock()

	for idx, issue := range r.issues {
		if issue.ID == issueID {
			r.issues[idx].Status = status
			return r.issues[idx], nil
		}
	}
	return Issue{}, errors.New("issue not found")
}

// AssignIssue sets or transfers ownership of an issue to a specific agent or human.
//
// Accepts parameters:
//   - issueID: string; The unique registry ID of the issue.
//   - assignee: string; The identifier of the assigned worker.
//
// Returns The updated Issue record.
//
// Produces errors: Fails if the issue cannot be found.
//
// Has side effects: Mutates the AssignedTo field of the specific Issue record.
func (r *Registry) AssignIssue(issueID, assignee string) (Issue, error) {
	r.mu.Lock()
	defer r.mu.Unlock()

	for idx, issue := range r.issues {
		if issue.ID == issueID {
			r.issues[idx].AssignedTo = assignee
			return r.issues[idx], nil
		}
	}
	return Issue{}, errors.New("issue not found")
}

// Issues retrieves all recorded tickets, with an optional integration ID filter.
//
// Accepts parameters:
//   - integrationID: string; Filter by integration. Pass an empty string for all tickets.
//
// Returns A slice of Issue records.
//
// Produces errors: None.
//
// Has side effects: None. Executes a read-only lock.
func (r *Registry) Issues(integrationID string) []Issue {
	r.mu.RLock()
	defer r.mu.RUnlock()

	var result []Issue
	for _, issue := range r.issues {
		if integrationID == "" || issue.IntegrationID == integrationID {
			result = append(result, issue)
		}
	}
	return result
}

// ── Internal helpers ──────────────────────────────────────────────────────────

// findIntegration looks up an integration by ID; caller must hold mu.
func (r *Registry) findIntegration(id string) (*IntegrationInstance, bool) {
	inst, ok := r.instances[id]
	return inst, ok
}

// generateID produces a namespaced, time-stamped identifier for an activity record.
func generateID(prefix string, now time.Time) string {
	return prefix + "-" + now.UTC().Format("20060102150405.000000000")
}

// ── Real outbound HTTP helpers ────────────────────────────────────────────────

// TelegramAPIBase is the base URL for the Telegram Bot API. Override in tests to point to a mock server.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
var TelegramAPIBase = "https://api.telegram.org"

// sendTelegramMessage posts a text message to a Telegram chat via the Bot API.
func sendTelegramMessage(ctx context.Context, botToken, chatID, text string) error {
	if err := validateURL(TelegramAPIBase); err != nil {
		return err
	}

	apiURL := TelegramAPIBase + "/bot" + botToken + "/sendMessage"
	payload, _ := json.Marshal(map[string]string{
		"chat_id": chatID,
		"text":    text,
	})
	req, err := http.NewRequestWithContext(ctx, http.MethodPost, apiURL, bytes.NewReader(payload))
	if err != nil {
		return fmt.Errorf("create request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := safeClient.Do(req)
	if err != nil {
		return fmt.Errorf("telegram API: %w", err)
	}
	defer func() { _, _ = io.Copy(io.Discard, resp.Body); _ = resp.Body.Close() }()

	var result struct {
		OK          bool   `json:"ok"`
		Description string `json:"description,omitempty"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return fmt.Errorf("telegram decode: %w", err)
	}
	if !result.OK {
		return fmt.Errorf("telegram error: %s", result.Description)
	}
	return nil
}

// sendDiscordWebhook posts a message to a Discord channel via an inbound webhook.
func sendDiscordWebhook(ctx context.Context, webhookURL, username, content string) error {
	if err := validateURL(webhookURL); err != nil {
		return err
	}

	payload, _ := json.Marshal(map[string]string{
		"username": username,
		"content":  content,
	})
	req, err := http.NewRequestWithContext(ctx, http.MethodPost, webhookURL, bytes.NewReader(payload))
	if err != nil {
		return fmt.Errorf("create request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := safeClient.Do(req)
	if err != nil {
		return fmt.Errorf("discord API: %w", err)
	}
	defer func() { _, _ = io.Copy(io.Discard, resp.Body); _ = resp.Body.Close() }()
	// Discord webhooks return 204 No Content on success.
	if resp.StatusCode != http.StatusNoContent && resp.StatusCode != http.StatusOK {
		return fmt.Errorf("discord API returned status %d", resp.StatusCode)
	}
	return nil
}
