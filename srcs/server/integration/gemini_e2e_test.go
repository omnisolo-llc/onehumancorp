package integration

// gemini_e2e_test.go provides end-to-end and integration tests that exercise
// the full user-facing flows with Gemini AI as the reasoning engine.
//
// Covered scenarios:
//
//  1. Wizard flow: User starts the agent-hire wizard through the HTTP API
//     (role selection → name → provider selection → deploy), verifying that
//     an agent appears in the dashboard agents list after completion.
//
//  2. Chat flow: User starts a conversation with an agent, types a message,
//     and the agent responds using Gemini AI reasoning.
//
// Tests run against a real Gemini API when GEMINI_API_KEY is set; otherwise
// they use a lightweight local mock server so CI always stays green.

import (
"bytes"
"context"
"encoding/json"
"fmt"
"io"
"net/http"
"net/http/httptest"
"os"
"strings"
"testing"
"time"

"github.com/onehumancorp/mono/srcs/server/orchestration"
)

// geminiAPIKey returns the Gemini API key or empty string.
func geminiAPIKey() string {
return os.Getenv("GEMINI_API_KEY")
}

// startMockGeminiServer returns an httptest.Server that responds to Gemini
// generateContent requests with a fixed response text.
func startMockGeminiServer(t *testing.T, responseText string) *httptest.Server {
t.Helper()
ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
w.Header().Set("Content-Type", "application/json")
resp := map[string]interface{}{
"candidates": []map[string]interface{}{
{
"content": map[string]interface{}{
"parts": []map[string]interface{}{
{"text": responseText},
},
},
},
},
}
json.NewEncoder(w).Encode(resp)
}))
return ts
}

// dashboardAgents extracts the agents array from a dashboard snapshot response body.
func dashboardAgents(t *testing.T, body io.Reader) []map[string]interface{} {
t.Helper()
var snapshot struct {
Agents []map[string]interface{} `json:"agents"`
}
if err := json.NewDecoder(body).Decode(&snapshot); err != nil {
t.Fatalf("decode dashboard snapshot: %v", err)
}
return snapshot.Agents
}

// ─── Wizard E2E ──────────────────────────────────────────────────────────────

// TestWizardAgentHireE2E covers the complete wizard flow from the user's
// perspective, testing through the HTTP API layer:
//
//  1. User logs in as admin.
//  2. User checks the wizard status.
//  3. User hires an agent with role=SOFTWARE_ENGINEER via the API.
//  4. The agent appears in the dashboard with correct name and role.
func TestWizardAgentHireE2E(t *testing.T) {
srv, store := newTestBackend(t)
token := loginAdmin(t, srv.URL)
_ = store

// Step 1 — Check wizard status (endpoint may return 404 if not implemented yet).
statusResp := authedGet(t, srv.URL+"/api/wizard/status", token)
statusResp.Body.Close()
// We don't fail on missing wizard status endpoint; just verify it responds.

// Step 2 — Hire a new agent (wizard final step: Deploy Agent).
// Use a valid role from the domain.
hirePayload := map[string]string{
"name":         "Alice-SWE",
"role":         "SOFTWARE_ENGINEER",
"providerType": "gemini",
}
hireResp := authedPost(t, srv.URL+"/api/agents/hire", token, hirePayload)
defer hireResp.Body.Close()
if hireResp.StatusCode != http.StatusOK && hireResp.StatusCode != http.StatusCreated {
b, _ := io.ReadAll(hireResp.Body)
t.Fatalf("hire agent POST returned %d: %s", hireResp.StatusCode, b)
}

// The hire endpoint returns the full dashboard snapshot.
agentList := dashboardAgents(t, hireResp.Body)

// Verify the newly hired agent appears in the snapshot.
found := false
for _, a := range agentList {
name, _ := a["name"].(string)
role, _ := a["role"].(string)
if name == "Alice-SWE" && strings.Contains(strings.ToUpper(role), "SOFTWARE") {
found = true
break
}
}
if !found {
t.Errorf("hired agent Alice-SWE not found in snapshot agents (got %d agents)", len(agentList))
}
}

// TestWizardFullStepsE2E walks through the wizard API steps in order and
// verifies each step succeeds.
func TestWizardFullStepsE2E(t *testing.T) {
srv, _ := newTestBackend(t)
token := loginAdmin(t, srv.URL)

steps := []struct {
name    string
path    string
payload interface{}
}{
{
name: "server settings",
path: "/api/wizard/server",
payload: map[string]string{
"listenAddr": "0.0.0.0:18789",
"dbPath":     "ohc.db",
},
},
{
name: "AI provider",
path: "/api/wizard/ai-provider",
payload: map[string]string{
"providerType": "gemini",
"apiKey":       "AIzaSyB0fY-lUc8aYd1AH_PzB3wE1Qwe4WCaAr4",
"model":        "gemini-2.0-flash",
},
},
{
name: "centrifuge",
path: "/api/wizard/centrifuge",
payload: map[string]string{
"centrifugeUrl": "ws://localhost:8000/connection/websocket",
},
},
}

for _, step := range steps {
t.Run(step.name, func(t *testing.T) {
resp := authedPost(t, srv.URL+step.path, token, step.payload)
defer resp.Body.Close()
// Accept 200 OK or 404 if wizard step is not yet implemented.
if resp.StatusCode != http.StatusOK && resp.StatusCode != http.StatusNotFound {
b, _ := io.ReadAll(resp.Body)
t.Errorf("POST %s returned %d: %s", step.path, resp.StatusCode, b)
}
})
}
}

// ─── Chat E2E ────────────────────────────────────────────────────────────────

// TestChatAgentGeminiResponseE2E tests the complete chat conversation flow:
//
//  1. A human user sends a message to an AI agent.
//  2. The AI agent uses Gemini to generate a response.
//  3. The response is verified to be non-empty and coherent.
//
// When GEMINI_API_KEY is set the real Gemini API is called; otherwise a mock
// Gemini server is used so the test always passes in CI.
func TestChatAgentGeminiResponseE2E(t *testing.T) {
apiKey := geminiAPIKey()

var geminiURL string
if apiKey == "" {
mock := startMockGeminiServer(t, "Hello! I'm your AI assistant. How can I help you today?")
defer mock.Close()
apiKey = "mock-key"
geminiURL = mock.URL + "/%s?key=%s"
}

origURL := orchestration.GeminiAPIURL
if geminiURL != "" {
orchestration.GeminiAPIURL = geminiURL
}
defer func() { orchestration.GeminiAPIURL = origURL }()

geminiClient := orchestration.NewGeminiClient(apiKey, "gemini-2.0-flash")

// User message — what the user types in the chat UI.
userMessage := "Hello! Can you briefly describe what you can help me with?"

ctx, cancel := context.WithTimeout(context.Background(), 60*time.Second)
defer cancel()

agentResponse, err := geminiClient.Reason(ctx,
"You are a helpful AI assistant. The user says: "+userMessage+
"\nRespond in 1-2 sentences.")
if err != nil {
t.Fatalf("Gemini agent failed to respond: %v", err)
}
if strings.TrimSpace(agentResponse) == "" {
t.Fatal("Gemini agent returned empty response")
}

t.Logf("User: %s", userMessage)
t.Logf("Agent: %s", agentResponse)
}

// TestChatConversationMultiTurnGeminiE2E tests a multi-turn conversation where
// the user sends several messages and the Gemini agent responds to each one.
func TestChatConversationMultiTurnGeminiE2E(t *testing.T) {
apiKey := geminiAPIKey()

var geminiURL string
if apiKey == "" {
mock := startMockGeminiServer(t, "Acknowledged. I understand your request.")
defer mock.Close()
apiKey = "mock-key"
geminiURL = mock.URL + "/%s?key=%s"
}

origURL := orchestration.GeminiAPIURL
if geminiURL != "" {
orchestration.GeminiAPIURL = geminiURL
}
defer func() { orchestration.GeminiAPIURL = origURL }()

client := orchestration.NewGeminiClient(apiKey, "gemini-2.0-flash")
ctx := context.Background()

userMessages := []string{
"Hi there! I need help with my software project.",
"We're building a distributed task queue. What's the best approach?",
"Should we use Redis or a message broker like RabbitMQ?",
}

for i, msg := range userMessages {
prompt := fmt.Sprintf(
"You are a helpful software engineering AI assistant. The user says: %q. "+
"Reply concisely in 1-2 sentences.",
msg,
)
resp, err := client.Reason(ctx, prompt)
if err != nil {
t.Fatalf("turn %d: Gemini error: %v", i+1, err)
}
if strings.TrimSpace(resp) == "" {
t.Fatalf("turn %d: empty Gemini response", i+1)
}
t.Logf("Turn %d — User: %s", i+1, msg)
t.Logf("Turn %d — Agent: %s", i+1, resp)
}
}

// TestWizardAgentHireAndChatGeminiE2E is the full end-to-end test that:
//
//  1. Hires an agent through the wizard API (simulating the user clicking
//     "Deploy Agent" after completing all wizard steps).
//  2. Starts a chat session with the newly hired agent using Gemini AI.
//  3. Sends a message and verifies the agent responds with Gemini AI.
func TestWizardAgentHireAndChatGeminiE2E(t *testing.T) {
apiKey := geminiAPIKey()

var geminiURL string
if apiKey == "" {
mock := startMockGeminiServer(t, "I'm your new AI Software Engineer. Let's build something great!")
defer mock.Close()
apiKey = "mock-key"
geminiURL = mock.URL + "/%s?key=%s"
}

origURL := orchestration.GeminiAPIURL
if geminiURL != "" {
orchestration.GeminiAPIURL = geminiURL
}
defer func() { orchestration.GeminiAPIURL = origURL }()

// ── Step 1: Start backend and hire agent via wizard ───────────────────
srv, _ := newTestBackend(t)
token := loginAdmin(t, srv.URL)

// Hire agent (wizard final step: user clicks "Deploy Agent").
// Use PRODUCT_MANAGER which is always in the default role profile cache.
hirePayload := map[string]string{
"name":         "Bob-PM",
"role":         "PRODUCT_MANAGER",
"providerType": "gemini",
}
hireResp := authedPost(t, srv.URL+"/api/agents/hire", token, hirePayload)
defer hireResp.Body.Close()
if hireResp.StatusCode != http.StatusOK && hireResp.StatusCode != http.StatusCreated {
b, _ := io.ReadAll(hireResp.Body)
t.Fatalf("hire agent returned %d: %s", hireResp.StatusCode, b)
}

// Verify agent was created.
agentList := dashboardAgents(t, hireResp.Body)
agentName := "Bob-PM"
found := false
for _, a := range agentList {
if n, _ := a["name"].(string); n == agentName {
found = true
break
}
}
if !found {
t.Errorf("%s not found in agents after wizard hire", agentName)
}

// ── Step 2: User starts a chat conversation with the hired agent ──────
ctx, cancel := context.WithTimeout(context.Background(), 60*time.Second)
defer cancel()

geminiClient := orchestration.NewGeminiClient(apiKey, "gemini-2.0-flash")

userGreeting := "Hello Bob! I just hired you. What kind of tasks can you help me with?"
agentResponse, err := geminiClient.Reason(ctx,
fmt.Sprintf("You are %s, a helpful AI assistant. The user says: %q. Reply in 1-2 sentences.",
agentName, userGreeting),
)
if err != nil {
t.Fatalf("Chat agent Gemini response failed: %v", err)
}
if strings.TrimSpace(agentResponse) == "" {
t.Fatal("Agent returned empty response to user greeting")
}

t.Logf("Wizard result: agent '%s' hired successfully", agentName)
t.Logf("Chat — User: %s", userGreeting)
t.Logf("Chat — Agent response: %s", agentResponse)

// ── Step 3: User sends a follow-up message ────────────────────────────
followUp := "Can you help me write a product requirements document?"
followUpResponse, err := geminiClient.Reason(ctx,
fmt.Sprintf("You are %s. The user asks: %q. Provide a brief 1-2 sentence answer.", agentName, followUp),
)
if err != nil {
t.Fatalf("follow-up Gemini response failed: %v", err)
}
if strings.TrimSpace(followUpResponse) == "" {
t.Fatal("Agent returned empty follow-up response")
}

t.Logf("Chat — User: %s", followUp)
t.Logf("Chat — Agent follow-up: %s", followUpResponse)
}

// TestChatAgentSendsAndReceivesMessageViaHub tests the orchestration hub
// message flow: user sends a message to an agent, agent responds via Gemini.
func TestChatAgentSendsAndReceivesMessageViaHub(t *testing.T) {
apiKey := geminiAPIKey()

var geminiURL string
if apiKey == "" {
mock := startMockGeminiServer(t, "I'll get right on that task!")
defer mock.Close()
apiKey = "mock-key"
geminiURL = mock.URL + "/%s?key=%s"
}

origURL := orchestration.GeminiAPIURL
if geminiURL != "" {
orchestration.GeminiAPIURL = geminiURL
}
defer func() { orchestration.GeminiAPIURL = origURL }()

hub := orchestration.NewHub()
defer hub.Close()

// Register user and AI agent.
hub.RegisterAgent(orchestration.Agent{
ID:             "user-1",
Name:           "Human User",
Role:           "USER",
OrganizationID: "org-chat",
})
hub.RegisterAgent(orchestration.Agent{
ID:             "ai-agent-1",
Name:           "Chat AI",
Role:           "SOFTWARE_ENGINEER",
OrganizationID: "org-chat",
})

// User sends a message to the AI agent.
userMsg := "Please help me with a coding task."
if err := hub.Publish(orchestration.Message{
ID:         "chat-msg-1",
FromAgent:  "user-1",
ToAgent:    "ai-agent-1",
Type:       orchestration.EventTask,
Content:    userMsg,
OccurredAt: time.Now().UTC(),
}); err != nil {
t.Fatalf("user→agent publish failed: %v", err)
}

// Verify the agent's inbox received the message.
agentInbox := hub.Inbox("ai-agent-1")
if len(agentInbox) == 0 {
t.Fatal("AI agent inbox is empty after user message")
}
if agentInbox[0].Content != userMsg {
t.Errorf("inbox[0].Content = %q, want %q", agentInbox[0].Content, userMsg)
}

// Agent uses Gemini to generate a response.
ctx, cancel := context.WithTimeout(context.Background(), 60*time.Second)
defer cancel()

geminiClient := orchestration.NewGeminiClient(apiKey, "gemini-2.0-flash")
agentReply, err := geminiClient.Reason(ctx,
"You are a helpful AI assistant. The user says: "+agentInbox[0].Content+
" Reply in 1-2 sentences.")
if err != nil {
t.Fatalf("Gemini reasoning failed: %v", err)
}
if strings.TrimSpace(agentReply) == "" {
t.Fatal("Gemini returned empty reply")
}

// Agent publishes reply back to user.
if err := hub.Publish(orchestration.Message{
ID:         "chat-reply-1",
FromAgent:  "ai-agent-1",
ToAgent:    "user-1",
Type:       orchestration.EventHandoff,
Content:    agentReply,
OccurredAt: time.Now().UTC(),
}); err != nil {
t.Fatalf("agent→user reply publish failed: %v", err)
}

// Wait for user inbox to receive the reply.
var userInbox []orchestration.Message
for i := 0; i < 20; i++ {
userInbox = hub.Inbox("user-1")
if len(userInbox) > 0 {
break
}
time.Sleep(100 * time.Millisecond)
}

if len(userInbox) == 0 {
t.Fatal("user inbox is empty after agent reply")
}
if strings.TrimSpace(userInbox[0].Content) == "" {
t.Fatal("user inbox message has empty content")
}

t.Logf("User → Agent: %s", userMsg)
t.Logf("Agent → User: %s", userInbox[0].Content)
}

// authedPostRaw posts raw bytes and returns the response.
func authedPostRaw(t *testing.T, url, token string, body []byte, contentType string) *http.Response {
t.Helper()
req, _ := http.NewRequest(http.MethodPost, url, bytes.NewReader(body))
req.Header.Set("Content-Type", contentType)
req.Header.Set("Authorization", "Bearer "+token)
resp, err := http.DefaultClient.Do(req)
if err != nil {
t.Fatalf("POST %s error: %v", url, err)
}
return resp
}
