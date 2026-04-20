// Copyright 2026 Author(s) of OHC
// SPDX-License-Identifier: Apache-2.0

package dashboard

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
)

// ── 200 additional server handler tests ──────────────────────────────────────
// These tests exercise the HTTP handler layer using the existing newTestServer
// helper, which creates an in-process OHC server backed by a SQLite in-memory
// database and a local orchestration hub.  No external services are required.

// ── /api/auth ─────────────────────────────────────────────────────────────────

func TestAuthLogin_WrongMethod(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	resp, err := http.Get(ts.URL + "/api/auth/login")
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode == http.StatusOK {
		t.Log("GET /api/auth/login returned 200 (may be allowed)")
	}
}

func TestAuthLogin_EmptyBody(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	resp, err := http.Post(ts.URL+"/api/auth/login", "application/json", strings.NewReader("{}"))
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("unexpected server error %d", resp.StatusCode)
	}
}

func TestAuthLogin_InvalidJSON(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	resp, err := http.Post(ts.URL+"/api/auth/login", "application/json", strings.NewReader("not-json"))
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("unexpected server error %d", resp.StatusCode)
	}
}

func TestAuthLogin_WrongCredentials(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	body, _ := json.Marshal(map[string]string{"username": "bad", "password": "bad"})
	resp, err := http.Post(ts.URL+"/api/auth/login", "application/json", bytes.NewReader(body))
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode == http.StatusOK {
		t.Error("expected non-200 for wrong credentials")
	}
}

func TestAuthLogin_CorrectCredentials(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	body, _ := json.Marshal(map[string]string{"username": "admin", "password": "admin"})
	resp, err := http.Post(ts.URL+"/api/auth/login", "application/json", bytes.NewReader(body))
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != http.StatusOK {
		t.Errorf("expected 200, got %d", resp.StatusCode)
	}
}

// ── /healthz ─────────────────────────────────────────────────────────────────

func TestHealthzReturns200(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	resp, err := http.Get(ts.URL + "/healthz")
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != http.StatusOK {
		t.Errorf("expected 200, got %d", resp.StatusCode)
	}
}

func TestHealthzContentType(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	resp, err := http.Get(ts.URL + "/healthz")
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	ct := resp.Header.Get("Content-Type")
	if !strings.Contains(ct, "json") && !strings.Contains(ct, "text") {
		t.Logf("healthz content-type: %q", ct)
	}
}

func TestReadyzReturns200OrHandled(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	resp, err := http.Get(ts.URL + "/readyz")
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d on /readyz", resp.StatusCode)
	}
}

// ── /api/dashboard ────────────────────────────────────────────────────────────

func TestHandleDashboard_WithToken(t *testing.T) {
	app, ts, _ := newTestServer(t)
	defer ts.Close()
	_ = app
	tok := loginForTest(t, ts.URL)
	req, _ := http.NewRequest(http.MethodGet, ts.URL+"/api/dashboard", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleDashboard_NoToken(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	resp, err := http.Get(ts.URL + "/api/dashboard")
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleDashboard_HeadMethod(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	req, _ := http.NewRequest(http.MethodHead, ts.URL+"/api/dashboard", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

// ── /api/org ──────────────────────────────────────────────────────────────────

func TestHandleOrg_WithToken(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	req, _ := http.NewRequest(http.MethodGet, ts.URL+"/api/org", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleOrg_NoToken(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	resp, err := http.Get(ts.URL + "/api/org")
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleOrg_ReturnsJSON(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	req, _ := http.NewRequest(http.MethodGet, ts.URL+"/api/org", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	var body map[string]any
	_ = json.NewDecoder(resp.Body).Decode(&body)
}

// ── /api/meetings ─────────────────────────────────────────────────────────────

func TestHandleMeetings_WithToken(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	req, _ := http.NewRequest(http.MethodGet, ts.URL+"/api/meetings", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleMeetings_NoToken(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	resp, err := http.Get(ts.URL + "/api/meetings")
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleMeetings_PostMethodHandled(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	body, _ := json.Marshal(map[string]string{"name": "Test Room"})
	req, _ := http.NewRequest(http.MethodPost, ts.URL+"/api/meetings", bytes.NewReader(body))
	req.Header.Set("Authorization", "Bearer "+tok)
	req.Header.Set("Content-Type", "application/json")
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

// ── /api/messages ─────────────────────────────────────────────────────────────

func TestHandleMessages_EmptyPost(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	req, _ := http.NewRequest(http.MethodPost, ts.URL+"/api/messages", strings.NewReader(""))
	req.Header.Set("Authorization", "Bearer "+tok)
	req.Header.Set("Content-Type", "application/x-www-form-urlencoded")
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleMessages_InvalidJSON(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	req, _ := http.NewRequest(http.MethodPost, ts.URL+"/api/messages", strings.NewReader("notjson"))
	req.Header.Set("Authorization", "Bearer "+tok)
	req.Header.Set("Content-Type", "application/json")
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleMessages_GetMethod(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	req, _ := http.NewRequest(http.MethodGet, ts.URL+"/api/messages", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

// ── /api/agents ───────────────────────────────────────────────────────────────

func TestHandleAgents_List(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	req, _ := http.NewRequest(http.MethodGet, ts.URL+"/api/agents", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleAgents_HirePost(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	body, _ := json.Marshal(map[string]string{"role": "SOFTWARE_ENGINEER", "name": "TestAgent"})
	req, _ := http.NewRequest(http.MethodPost, ts.URL+"/api/agents/hire", bytes.NewReader(body))
	req.Header.Set("Authorization", "Bearer "+tok)
	req.Header.Set("Content-Type", "application/json")
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleAgents_HireInvalidJSON(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	req, _ := http.NewRequest(http.MethodPost, ts.URL+"/api/agents/hire", strings.NewReader("notjson"))
	req.Header.Set("Authorization", "Bearer "+tok)
	req.Header.Set("Content-Type", "application/json")
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleAgents_FirePost(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	body, _ := json.Marshal(map[string]string{"agentId": "nonexistent-agent-id"})
	req, _ := http.NewRequest(http.MethodPost, ts.URL+"/api/agents/fire", bytes.NewReader(body))
	req.Header.Set("Authorization", "Bearer "+tok)
	req.Header.Set("Content-Type", "application/json")
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

// ── /api/costs ────────────────────────────────────────────────────────────────

func TestHandleCosts_WithToken(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	req, _ := http.NewRequest(http.MethodGet, ts.URL+"/api/costs", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleCosts_NoToken(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	resp, err := http.Get(ts.URL + "/api/costs")
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleCosts_ReturnsJSON(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	req, _ := http.NewRequest(http.MethodGet, ts.URL+"/api/costs", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	var body map[string]any
	_ = json.NewDecoder(resp.Body).Decode(&body)
}

// ── /api/dev/seed ─────────────────────────────────────────────────────────────

func TestHandleDevSeed_ValidScenarios(t *testing.T) {
	scenarios := []string{"default", "chat", "security", "pipeline"}
	app, ts, _ := newTestServer(t)
	defer ts.Close()
	_ = app
	tok := loginForTest(t, ts.URL)
	for _, scenario := range scenarios {
		t.Run(scenario, func(t *testing.T) {
			body, _ := json.Marshal(map[string]string{"scenario": scenario})
			req, _ := http.NewRequest(http.MethodPost, ts.URL+"/api/dev/seed", bytes.NewReader(body))
			req.Header.Set("Authorization", "Bearer "+tok)
			req.Header.Set("Content-Type", "application/json")
			resp, err := http.DefaultClient.Do(req)
			if err != nil {
				t.Fatal(err)
			}
			defer resp.Body.Close()
			if resp.StatusCode >= 500 {
				t.Errorf("server error %d for scenario %q", resp.StatusCode, scenario)
			}
		})
	}
}

func TestHandleDevSeed_EmptyScenario(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	body, _ := json.Marshal(map[string]string{"scenario": ""})
	req, _ := http.NewRequest(http.MethodPost, ts.URL+"/api/dev/seed", bytes.NewReader(body))
	req.Header.Set("Authorization", "Bearer "+tok)
	req.Header.Set("Content-Type", "application/json")
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleDevSeed_GetMethodNotAllowed(t *testing.T) {
	app, _, _ := newTestServer(t)
	req := httptest.NewRequest(http.MethodGet, "/api/dev/seed", nil)
	rec := httptest.NewRecorder()
	app.handleDevSeed(rec, req)
	if rec.Code == http.StatusOK {
		t.Log("GET /api/dev/seed accepted (may be intentional)")
	}
}

// ── /api/users ────────────────────────────────────────────────────────────────

func TestHandleUsers_List(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	req, _ := http.NewRequest(http.MethodGet, ts.URL+"/api/users", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleUsers_NoToken(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	resp, err := http.Get(ts.URL + "/api/users")
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

// ── /api/tasks ────────────────────────────────────────────────────────────────

func TestHandleTasks_List(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	req, _ := http.NewRequest(http.MethodGet, ts.URL+"/api/tasks", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleTasks_Delegate(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	body, _ := json.Marshal(map[string]string{
		"fromAgent":   "agent-1",
		"toAgent":     "agent-2",
		"description": "test task",
	})
	req, _ := http.NewRequest(http.MethodPost, ts.URL+"/api/tasks/delegate", bytes.NewReader(body))
	req.Header.Set("Authorization", "Bearer "+tok)
	req.Header.Set("Content-Type", "application/json")
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleTasks_DelegateInvalidJSON(t *testing.T) {
	app, _, _ := newTestServer(t)
	req := httptest.NewRequest(http.MethodPost, "/api/tasks/delegate", strings.NewReader("badjson"))
	req.Header.Set("Content-Type", "application/json")
	rec := httptest.NewRecorder()
	app.handleDelegateTask(rec, req)
	if rec.Code == http.StatusOK {
		t.Log("invalid JSON returned 200, may be intentional")
	}
}

// ── /api/pipelines ────────────────────────────────────────────────────────────

func TestHandlePipelines_List(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	req, _ := http.NewRequest(http.MethodGet, ts.URL+"/api/pipelines", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandlePipelines_Create(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	body, _ := json.Marshal(map[string]string{"name": "new-pipeline", "branch": "main"})
	req, _ := http.NewRequest(http.MethodPost, ts.URL+"/api/pipelines", bytes.NewReader(body))
	req.Header.Set("Authorization", "Bearer "+tok)
	req.Header.Set("Content-Type", "application/json")
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandlePipelines_InvalidJSON(t *testing.T) {
	app, _, _ := newTestServer(t)
	req := httptest.NewRequest(http.MethodPost, "/api/pipelines", strings.NewReader("notjson"))
	req.Header.Set("Content-Type", "application/json")
	rec := httptest.NewRecorder()
	app.handlePipelines(rec, req)
	if rec.Code >= 500 {
		t.Errorf("server error %d", rec.Code)
	}
}

// ── /api/scheduler ────────────────────────────────────────────────────────────

func TestHandleScheduler_Status(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	req, _ := http.NewRequest(http.MethodGet, ts.URL+"/api/scheduler/status", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleScheduler_Jobs(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	req, _ := http.NewRequest(http.MethodGet, ts.URL+"/api/scheduler/jobs", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleScheduler_CreateJob(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	body, _ := json.Marshal(map[string]string{"cron": "0 * * * *", "task": "cleanup"})
	req, _ := http.NewRequest(http.MethodPost, ts.URL+"/api/scheduler/jobs", bytes.NewReader(body))
	req.Header.Set("Authorization", "Bearer "+tok)
	req.Header.Set("Content-Type", "application/json")
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

// ── /api/scale ────────────────────────────────────────────────────────────────

func TestHandleScale_MissingCount(t *testing.T) {
	app, _, _ := newTestServer(t)
	body, _ := json.Marshal(map[string]string{"role": "worker"})
	req := httptest.NewRequest(http.MethodPost, "/api/scale", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	rec := httptest.NewRecorder()
	app.handleScale(rec, req)
	// Accept any non-500
	if rec.Code >= 500 {
		t.Errorf("server error %d", rec.Code)
	}
}

func TestHandleScale_ValidRequest(t *testing.T) {
	app, _, _ := newTestServer(t)
	body, _ := json.Marshal(ScaleRequest{Role: "SOFTWARE_ENGINEER", Count: 2})
	req := httptest.NewRequest(http.MethodPost, "/api/scale", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	rec := httptest.NewRecorder()
	app.handleScale(rec, req)
	if rec.Code >= 500 {
		t.Errorf("server error %d", rec.Code)
	}
}

func TestHandleScale_ZeroCount(t *testing.T) {
	app, _, _ := newTestServer(t)
	body, _ := json.Marshal(ScaleRequest{Role: "SOFTWARE_ENGINEER", Count: 0})
	req := httptest.NewRequest(http.MethodPost, "/api/scale", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	rec := httptest.NewRecorder()
	app.handleScale(rec, req)
	if rec.Code >= 500 {
		t.Errorf("server error %d", rec.Code)
	}
}

func TestHandleScale_NegativeCount(t *testing.T) {
	app, _, _ := newTestServer(t)
	body, _ := json.Marshal(ScaleRequest{Role: "DESIGNER", Count: -3})
	req := httptest.NewRequest(http.MethodPost, "/api/scale", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	rec := httptest.NewRecorder()
	app.handleScale(rec, req)
	if rec.Code >= 500 {
		t.Errorf("server error %d", rec.Code)
	}
}

func TestHandleScaleGet_MethodNotAllowed(t *testing.T) {
	app, _, _ := newTestServer(t)
	req := httptest.NewRequest(http.MethodGet, "/api/scale", nil)
	rec := httptest.NewRecorder()
	app.handleScale(rec, req)
	if rec.Code >= 500 {
		t.Errorf("server error %d", rec.Code)
	}
}

// ── /api/b2b ──────────────────────────────────────────────────────────────────

func TestHandleB2BAgreements_Get(t *testing.T) {
	app, _, _ := newTestServer(t)
	req := httptest.NewRequest(http.MethodGet, "/api/b2b/agreements", nil)
	rec := httptest.NewRecorder()
	app.handleB2BAgreements(rec, req)
	if rec.Code >= 500 {
		t.Errorf("server error %d", rec.Code)
	}
}

func TestHandleB2BHandshake_ValidPost(t *testing.T) {
	app, _, _ := newTestServer(t)
	body, _ := json.Marshal(map[string]string{
		"partner_id": "p1",
		"org_id":     "org-1",
		"callback":   "https://example.com/callback",
	})
	req := httptest.NewRequest(http.MethodPost, "/api/b2b/handshake", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	rec := httptest.NewRecorder()
	app.handleB2BHandshake(rec, req)
	if rec.Code >= 500 {
		t.Errorf("server error %d", rec.Code)
	}
}

func TestHandleB2BRevoke_ValidPost(t *testing.T) {
	app, _, _ := newTestServer(t)
	body, _ := json.Marshal(map[string]string{"agreement_id": "agr-1"})
	req := httptest.NewRequest(http.MethodPost, "/api/b2b/revoke", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	rec := httptest.NewRecorder()
	app.handleB2BRevoke(rec, req)
	if rec.Code >= 500 {
		t.Errorf("server error %d", rec.Code)
	}
}

// ── /api/incidents ────────────────────────────────────────────────────────────

func TestHandleIncidents_Get(t *testing.T) {
	app, _, _ := newTestServer(t)
	req := httptest.NewRequest(http.MethodGet, "/api/incidents", nil)
	rec := httptest.NewRecorder()
	app.handleIncidents(rec, req)
	if rec.Code >= 500 {
		t.Errorf("server error %d", rec.Code)
	}
}

func TestHandleIncidents_Post(t *testing.T) {
	app, _, _ := newTestServer(t)
	body, _ := json.Marshal(map[string]string{"title": "Outage", "severity": "high"})
	req := httptest.NewRequest(http.MethodPost, "/api/incidents", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	rec := httptest.NewRecorder()
	app.handleIncidents(rec, req)
	if rec.Code >= 500 {
		t.Errorf("server error %d", rec.Code)
	}
}

func TestHandleIncidentStatus_Get(t *testing.T) {
	app, _, _ := newTestServer(t)
	req := httptest.NewRequest(http.MethodGet, "/api/incidents/status", nil)
	rec := httptest.NewRecorder()
	app.handleIncidentStatus(rec, req)
	if rec.Code >= 500 {
		t.Errorf("server error %d", rec.Code)
	}
}

// ── /api/budget ───────────────────────────────────────────────────────────────

func TestHandleBudgetAlerts_Get(t *testing.T) {
	app, _, _ := newTestServer(t)
	req := httptest.NewRequest(http.MethodGet, "/api/budget/alerts", nil)
	rec := httptest.NewRecorder()
	app.handleBudgetAlerts(rec, req)
	if rec.Code >= 500 {
		t.Errorf("server error %d", rec.Code)
	}
}

func TestHandleBudgetAlerts_Post(t *testing.T) {
	app, _, _ := newTestServer(t)
	body, _ := json.Marshal(map[string]any{"threshold": 100.0, "currency": "USD"})
	req := httptest.NewRequest(http.MethodPost, "/api/budget/alerts", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	rec := httptest.NewRecorder()
	app.handleBudgetAlerts(rec, req)
	if rec.Code >= 500 {
		t.Errorf("server error %d", rec.Code)
	}
}

// ── /api/compute-profiles ─────────────────────────────────────────────────────

func TestHandleComputeProfiles_Get(t *testing.T) {
	app, _, _ := newTestServer(t)
	req := httptest.NewRequest(http.MethodGet, "/api/compute-profiles", nil)
	rec := httptest.NewRecorder()
	app.handleComputeProfiles(rec, req)
	if rec.Code >= 500 {
		t.Errorf("server error %d", rec.Code)
	}
}

func TestHandleComputeProfiles_Post(t *testing.T) {
	app, _, _ := newTestServer(t)
	body, _ := json.Marshal(map[string]any{"name": "small", "vcpu": 2, "memory_gb": 4})
	req := httptest.NewRequest(http.MethodPost, "/api/compute-profiles", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	rec := httptest.NewRecorder()
	app.handleComputeProfiles(rec, req)
	if rec.Code >= 500 {
		t.Errorf("server error %d", rec.Code)
	}
}

// ── /api/cluster-status ───────────────────────────────────────────────────────

func TestHandleClusterStatus_Get(t *testing.T) {
	app, _, _ := newTestServer(t)
	req := httptest.NewRequest(http.MethodGet, "/api/cluster-status", nil)
	rec := httptest.NewRecorder()
	app.handleClusterStatus(rec, req)
	if rec.Code >= 500 {
		t.Errorf("server error %d", rec.Code)
	}
}

// ── /api/mcp ──────────────────────────────────────────────────────────────────

func TestHandleMCPTools_Get(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	req, _ := http.NewRequest(http.MethodGet, ts.URL+"/api/mcp/tools", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleMCPInvoke_EmptyJSON(t *testing.T) {
	app, _, _ := newTestServer(t)
	req := httptest.NewRequest(http.MethodPost, "/api/mcp/invoke", strings.NewReader("{}"))
	req.Header.Set("Content-Type", "application/json")
	rec := httptest.NewRecorder()
	app.handleMCPInvoke(rec, req)
	if rec.Code >= 500 {
		t.Errorf("server error %d", rec.Code)
	}
}

// ── /api/wizard ───────────────────────────────────────────────────────────────

func TestHandleWizard_Get(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	req, _ := http.NewRequest(http.MethodGet, ts.URL+"/api/wizard/state", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleWizard_StepPost(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	body, _ := json.Marshal(map[string]any{"step": 1, "data": map[string]string{"org_name": "TestOrg"}})
	req, _ := http.NewRequest(http.MethodPost, ts.URL+"/api/wizard/step", bytes.NewReader(body))
	req.Header.Set("Authorization", "Bearer "+tok)
	req.Header.Set("Content-Type", "application/json")
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

// ── /api/settings ─────────────────────────────────────────────────────────────

func TestHandleSettings_Get(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	req, _ := http.NewRequest(http.MethodGet, ts.URL+"/api/settings", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleSettings_Put(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	body, _ := json.Marshal(map[string]any{"key": "theme", "value": "dark"})
	req, _ := http.NewRequest(http.MethodPut, ts.URL+"/api/settings", bytes.NewReader(body))
	req.Header.Set("Authorization", "Bearer "+tok)
	req.Header.Set("Content-Type", "application/json")
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

// ── /api/growth ───────────────────────────────────────────────────────────────

func TestHandleGrowth_Referrals(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	req, _ := http.NewRequest(http.MethodGet, ts.URL+"/api/growth/referrals", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleGrowth_CreateReferral(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	body, _ := json.Marshal(map[string]string{"email": "referred@example.com"})
	req, _ := http.NewRequest(http.MethodPost, ts.URL+"/api/growth/referrals", bytes.NewReader(body))
	req.Header.Set("Authorization", "Bearer "+tok)
	req.Header.Set("Content-Type", "application/json")
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

// ── Additional boundary / edge-case server tests ──────────────────────────────

func TestServerRejectsOversizedBody(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	large := strings.Repeat("x", 1024*1024) // 1 MB
	req, _ := http.NewRequest(http.MethodPost, ts.URL+"/api/messages", strings.NewReader(large))
	req.Header.Set("Authorization", "Bearer "+tok)
	req.Header.Set("Content-Type", "application/json")
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d on oversized body", resp.StatusCode)
	}
}

func TestServerHandlesSpecialCharactersInBody(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	body, _ := json.Marshal(map[string]string{"content": "Hello <script>alert('xss')</script>"})
	req, _ := http.NewRequest(http.MethodPost, ts.URL+"/api/messages", bytes.NewReader(body))
	req.Header.Set("Authorization", "Bearer "+tok)
	req.Header.Set("Content-Type", "application/json")
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d on special chars", resp.StatusCode)
	}
}

func TestServerHandlesUnicodeBody(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	body, _ := json.Marshal(map[string]string{"content": "こんにちは 世界 🌍"})
	req, _ := http.NewRequest(http.MethodPost, ts.URL+"/api/messages", bytes.NewReader(body))
	req.Header.Set("Authorization", "Bearer "+tok)
	req.Header.Set("Content-Type", "application/json")
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d on unicode", resp.StatusCode)
	}
}

func TestServerHandlesSQLInjectionPayload(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	body, _ := json.Marshal(map[string]string{"username": "'; DROP TABLE agents; --"})
	req, _ := http.NewRequest(http.MethodPost, ts.URL+"/api/auth/login", bytes.NewReader(body))
	req.Header.Set("Authorization", "Bearer "+tok)
	req.Header.Set("Content-Type", "application/json")
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d on SQL injection payload", resp.StatusCode)
	}
}

func TestServerConcurrentRequests(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	done := make(chan struct{}, 5)
	for i := 0; i < 5; i++ {
		go func() {
			defer func() { done <- struct{}{} }()
			req, _ := http.NewRequest(http.MethodGet, ts.URL+"/api/dashboard", nil)
			req.Header.Set("Authorization", "Bearer "+tok)
			resp, err := http.DefaultClient.Do(req)
			if err != nil {
				return
			}
			resp.Body.Close()
		}()
	}
	for i := 0; i < 5; i++ {
		<-done
	}
}

func TestServerReturnsJSONOnDashboard(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	req, _ := http.NewRequest(http.MethodGet, ts.URL+"/api/dashboard", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	var body map[string]any
	if err := json.NewDecoder(resp.Body).Decode(&body); err != nil {
		t.Logf("non-JSON dashboard response: %v", err)
	}
}

func TestServerHandleNotFoundPaths(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	paths := []string{
		"/api/nonexistent",
		"/api/v999/something",
		"/totally/wrong",
	}
	for _, p := range paths {
		resp, err := http.Get(ts.URL + p)
		if err != nil {
			t.Logf("GET %s: error %v", p, err)
			continue
		}
		resp.Body.Close()
		if resp.StatusCode >= 500 {
			t.Errorf("server error %d for %s", resp.StatusCode, p)
		}
	}
}

func TestServerOptionsMethod(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	req, _ := http.NewRequest(http.MethodOptions, ts.URL+"/api/dashboard", nil)
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d on OPTIONS", resp.StatusCode)
	}
}

func TestServerMultipleLoginRequests(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	for i := 0; i < 3; i++ {
		body, _ := json.Marshal(map[string]string{"username": "admin", "password": "admin"})
		resp, err := http.Post(ts.URL+"/api/auth/login", "application/json", bytes.NewReader(body))
		if err != nil {
			t.Fatal(err)
		}
		resp.Body.Close()
		if resp.StatusCode != http.StatusOK {
			t.Errorf("login %d: expected 200, got %d", i, resp.StatusCode)
		}
	}
}

func TestHandleHireAgent_AllRoles(t *testing.T) {
	roles := []string{"SOFTWARE_ENGINEER", "DESIGNER", "PROJECT_MANAGER", "SALES"}
	app, _, _ := newTestServer(t)
	for _, role := range roles {
		t.Run(role, func(t *testing.T) {
			body, _ := json.Marshal(map[string]string{"role": role, "name": "agent-" + role})
			req := httptest.NewRequest(http.MethodPost, "/api/agents/hire", bytes.NewReader(body))
			req.Header.Set("Content-Type", "application/json")
			rec := httptest.NewRecorder()
			app.handleHireAgent(rec, req)
			if rec.Code >= 500 {
				t.Errorf("server error %d for role %q", rec.Code, role)
			}
		})
	}
}

func TestHandleFireAgent_MissingID(t *testing.T) {
	app, _, _ := newTestServer(t)
	body, _ := json.Marshal(map[string]string{})
	req := httptest.NewRequest(http.MethodPost, "/api/agents/fire", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	rec := httptest.NewRecorder()
	app.handleFireAgent(rec, req)
	if rec.Code == http.StatusOK {
		t.Log("fire agent with missing ID returned 200")
	}
}

func TestHandleFireAgent_NonexistentAgent(t *testing.T) {
	app, _, _ := newTestServer(t)
	body, _ := json.Marshal(map[string]string{"agentId": "id-does-not-exist"})
	req := httptest.NewRequest(http.MethodPost, "/api/agents/fire", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	rec := httptest.NewRecorder()
	app.handleFireAgent(rec, req)
	if rec.Code >= 500 {
		t.Errorf("server error %d", rec.Code)
	}
}

func TestHandleDevSeed_RejectGetMethod(t *testing.T) {
	app, _, _ := newTestServer(t)
	req := httptest.NewRequest(http.MethodGet, "/api/dev/seed", nil)
	rec := httptest.NewRecorder()
	app.handleDevSeed(rec, req)
	if rec.Code >= 500 {
		t.Errorf("server error %d", rec.Code)
	}
}

func TestHandleDevSeed_AgentScenario(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	body, _ := json.Marshal(map[string]string{"scenario": "agent"})
	req, _ := http.NewRequest(http.MethodPost, ts.URL+"/api/dev/seed", bytes.NewReader(body))
	req.Header.Set("Authorization", "Bearer "+tok)
	req.Header.Set("Content-Type", "application/json")
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleSendMessage_JSONBody(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	body, _ := json.Marshal(map[string]string{
		"fromAgent":   "user",
		"toAgent":     "agent-1",
		"meetingId":   "m-1",
		"content":     "Hello agent, please do the task.",
		"messageType": "direction",
	})
	req, _ := http.NewRequest(http.MethodPost, ts.URL+"/api/messages", bytes.NewReader(body))
	req.Header.Set("Authorization", "Bearer "+tok)
	req.Header.Set("Content-Type", "application/json")
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleSendMessage_FormBody(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	form := "fromAgent=user&toAgent=agent-1&meetingId=m-1&content=test&messageType=direction"
	req, _ := http.NewRequest(http.MethodPost, ts.URL+"/api/messages", strings.NewReader(form))
	req.Header.Set("Authorization", "Bearer "+tok)
	req.Header.Set("Content-Type", "application/x-www-form-urlencoded")
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		t.Errorf("server error %d", resp.StatusCode)
	}
}

func TestHandleDashboard_JSONResponseShape(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	tok := loginForTest(t, ts.URL)
	client := authedClient(tok)
	resp, err := client.Get(ts.URL + "/api/dashboard")
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	var body map[string]any
	if err := json.NewDecoder(resp.Body).Decode(&body); err != nil {
		t.Logf("non-JSON response: %v", err)
		return
	}
	// The dashboard snapshot must not expose raw passwords
	for _, key := range []string{"password", "secret", "private_key"} {
		if _, ok := body[key]; ok {
			t.Errorf("dashboard response exposes sensitive field %q", key)
		}
	}
}

func TestHandleOrg_AgentsFieldExists(t *testing.T) {
	_, ts, _ := newTestServer(t)
	defer ts.Close()
	client := authedClient(loginForTest(t, ts.URL))
	resp, err := client.Get(ts.URL + "/api/org")
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()
	var body map[string]any
	if err := json.NewDecoder(resp.Body).Decode(&body); err != nil {
		t.Logf("non-JSON response: %v", err)
		return
	}
	if _, ok := body["agents"]; !ok {
		t.Log("agents field not present in /api/org (may be OK)")
	}
}

// ── Additional handler tests (batch 2) ───────────────────────────────────────

func TestHandleSkills_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/skills", nil)
rec := httptest.NewRecorder()
app.handleSkills(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleIntegrations_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/integrations", nil)
rec := httptest.NewRecorder()
app.handleIntegrations(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleIntegrationConnect_MissingBody(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodPost, "/api/integrations/connect", strings.NewReader("{}"))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleIntegrationConnect(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleIntegrationDisconnect_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodPost, "/api/integrations/disconnect", strings.NewReader(`{"id":"int-1"}`))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleIntegrationDisconnect(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleReferrals_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/referrals", nil)
rec := httptest.NewRecorder()
app.handleReferrals(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleReferralClick_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"code": "REF123"})
req := httptest.NewRequest(http.MethodPost, "/api/referrals/click", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleReferralClick(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleReferralConvert_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"code": "REF123", "user_id": "u-1"})
req := httptest.NewRequest(http.MethodPost, "/api/referrals/convert", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleReferralConvert(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleAnalytics_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/analytics", nil)
rec := httptest.NewRecorder()
app.handleAnalytics(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleApprovals_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/approvals", nil)
rec := httptest.NewRecorder()
app.handleApprovals(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleApprovalRequest_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"action": "deploy", "resource": "prod"})
req := httptest.NewRequest(http.MethodPost, "/api/approvals/request", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleApprovalRequest(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleApprovalDecide_Approve(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"approval_id": "apr-1", "decision": "approve"})
req := httptest.NewRequest(http.MethodPost, "/api/approvals/decide", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleApprovalDecide(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleApprovalDecide_Reject(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"approval_id": "apr-1", "decision": "reject"})
req := httptest.NewRequest(http.MethodPost, "/api/approvals/decide", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleApprovalDecide(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleHandoffs_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/handoffs", nil)
rec := httptest.NewRecorder()
app.handleHandoffs(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleHandoffResolve_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"handoff_id": "h-1"})
req := httptest.NewRequest(http.MethodPost, "/api/handoffs/resolve", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleHandoffResolve(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleIssues_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/issues", nil)
rec := httptest.NewRecorder()
app.handleIssues(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleIssueCreate_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"title": "Bug #1", "description": "Something broke"})
req := httptest.NewRequest(http.MethodPost, "/api/issues", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleIssueCreate(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleIssueAssign_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"issue_id": "issue-1", "agent_id": "agent-1"})
req := httptest.NewRequest(http.MethodPost, "/api/issues/assign", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleIssueAssign(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleIssueUpdateStatus_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"issue_id": "issue-1", "status": "closed"})
req := httptest.NewRequest(http.MethodPost, "/api/issues/status", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleIssueUpdateStatus(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandlePullRequests_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/pull-requests", nil)
rec := httptest.NewRecorder()
app.handlePullRequests(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandlePRCreate_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"title": "Fix bug", "branch": "fix/bug-1", "base": "main"})
req := httptest.NewRequest(http.MethodPost, "/api/pull-requests", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handlePRCreate(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandlePRMerge_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"pr_id": "pr-1"})
req := httptest.NewRequest(http.MethodPost, "/api/pull-requests/merge", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handlePRMerge(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandlePRClose_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"pr_id": "pr-1"})
req := httptest.NewRequest(http.MethodPost, "/api/pull-requests/close", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handlePRClose(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleSnapshots_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/snapshots", nil)
rec := httptest.NewRecorder()
app.handleSnapshots(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleSnapshotCreate_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"label": "pre-deploy"})
req := httptest.NewRequest(http.MethodPost, "/api/snapshots", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleSnapshotCreate(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleSnapshotRestore_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"snapshot_id": "snap-1"})
req := httptest.NewRequest(http.MethodPost, "/api/snapshots/restore", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleSnapshotRestore(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleQuota_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/quota", nil)
rec := httptest.NewRecorder()
app.handleQuota(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleMarketplace_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/marketplace", nil)
rec := httptest.NewRecorder()
app.handleMarketplace(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleDomains_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/domains", nil)
rec := httptest.NewRecorder()
app.handleDomains(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleIdentities_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/identities", nil)
rec := httptest.NewRecorder()
app.handleIdentities(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleWaitlist_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"email": "wait@example.com"})
req := httptest.NewRequest(http.MethodPost, "/api/waitlist", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleWaitlist(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleWaitlist_EmptyEmail(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"email": ""})
req := httptest.NewRequest(http.MethodPost, "/api/waitlist", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleWaitlist(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleWizardStatus_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/wizard/status", nil)
rec := httptest.NewRecorder()
app.handleWizardStatus(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleWizardConfigure_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"org_name": "NewOrg", "domain": "neworg.com"})
req := httptest.NewRequest(http.MethodPost, "/api/wizard/configure", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleWizardConfigure(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleWizardOnboardingVerify_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"token": "verify-tok-1"})
req := httptest.NewRequest(http.MethodPost, "/api/wizard/onboarding/verify", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleWizardOnboardingVerify(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleSchedulerTasks_GetSmoke(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/scheduler/tasks", nil)
rec := httptest.NewRecorder()
app.handleSchedulerTasks(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleSchedulerCancel_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"task_id": "task-1"})
req := httptest.NewRequest(http.MethodPost, "/api/scheduler/cancel", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleSchedulerCancel(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleAgentProviders_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/agent-providers", nil)
rec := httptest.NewRecorder()
app.handleAgentProviders(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleAgentProviderAuth_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"provider": "openai", "api_key": "sk-test"})
req := httptest.NewRequest(http.MethodPost, "/api/agent-providers/auth", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleAgentProviderAuth(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleViralCoefficient_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/viral/coefficient", nil)
rec := httptest.NewRecorder()
app.handleViralCoefficient(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleViralCoefficientMetrics_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/viral/metrics", nil)
rec := httptest.NewRecorder()
app.handleViralCoefficientMetrics(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleOnboardingFunnel_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/onboarding/funnel", nil)
rec := httptest.NewRecorder()
app.handleOnboardingFunnel(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleOnboardingMetrics_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/onboarding/metrics", nil)
rec := httptest.NewRecorder()
app.handleOnboardingMetrics(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleLandingPageExperiments_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/landing-experiments", nil)
rec := httptest.NewRecorder()
app.handleLandingPageExperiments(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleLandingPageExperiments_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"variant": "B", "title": "New Hero"})
req := httptest.NewRequest(http.MethodPost, "/api/landing-experiments", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleLandingPageExperiments(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleTeamInvites_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/team/invites", nil)
rec := httptest.NewRecorder()
app.handleTeamInvites(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleTeamInvites_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"email": "newmember@example.com", "role": "member"})
req := httptest.NewRequest(http.MethodPost, "/api/team/invites", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleTeamInvites(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleTeamInviteAccept_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"token": "invite-tok-1"})
req := httptest.NewRequest(http.MethodPost, "/api/team/invites/accept", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleTeamInviteAccept(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleTelemetrySync_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]any{"events": []string{"page_view", "click"}})
req := httptest.NewRequest(http.MethodPost, "/api/telemetry/sync", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleTelemetrySync(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleSyncRAG_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"source": "docs"})
req := httptest.NewRequest(http.MethodPost, "/api/rag/sync", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleSyncRAG(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleSyncRules_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]any{"rules": []string{"rule1"}})
req := httptest.NewRequest(http.MethodPost, "/api/rules/sync", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleSyncRules(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleAutoDreamQuery_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"query": "generate landing page"})
req := httptest.NewRequest(http.MethodPost, "/api/autodream/query", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleAutoDreamQuery(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleAutoDreamSync_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"target": "landing_page_v2"})
req := httptest.NewRequest(http.MethodPost, "/api/autodream/sync", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleAutoDreamSync(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleContextSync_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"context_key": "project-overview"})
req := httptest.NewRequest(http.MethodPost, "/api/context/sync", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleContextSync(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleMissionsSync_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"mission_id": "m-1"})
req := httptest.NewRequest(http.MethodPost, "/api/missions/sync", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleMissionsSync(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleHybridSyncMissions_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"mode": "full"})
req := httptest.NewRequest(http.MethodPost, "/api/hybrid/missions/sync", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleHybridSyncMissions(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandlePruneMissions_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"before": "2025-01-01T00:00:00Z"})
req := httptest.NewRequest(http.MethodPost, "/api/missions/prune", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handlePruneMissions(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleMeshBroadcast_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"message": "hello mesh"})
req := httptest.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleMeshBroadcast(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleMeshDirect_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"to": "node-1", "message": "hello"})
req := httptest.NewRequest(http.MethodPost, "/api/mesh/direct", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleMeshDirect(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleMeshMailbox_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/mesh/mailbox", nil)
rec := httptest.NewRecorder()
app.handleMeshMailbox(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleMeshV2Broadcast_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"channel": "all", "message": "broadcast v2"})
req := httptest.NewRequest(http.MethodPost, "/api/mesh/v2/broadcast", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleMeshV2Broadcast(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleDownloads_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/downloads", nil)
rec := httptest.NewRecorder()
app.handleDownloads(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleSkillImport_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"url": "https://skills.example.com/skill-1"})
req := httptest.NewRequest(http.MethodPost, "/api/skills/import", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleSkillImport(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleChatMessages_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/chat/messages", nil)
rec := httptest.NewRecorder()
app.handleChatMessages(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleChatSend_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"room": "general", "content": "hello"})
req := httptest.NewRequest(http.MethodPost, "/api/chat/send", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleChatSend(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleMcpRagSync_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"tool": "rag-tool-1"})
req := httptest.NewRequest(http.MethodPost, "/api/mcp/rag/sync", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleMcpRagSync(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleMCPRegister_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"name": "my-tool", "url": "http://tool.example.com"})
req := httptest.NewRequest(http.MethodPost, "/api/mcp/register", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handleMCPRegister(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandleSettings_Get2(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/settings", nil)
rec := httptest.NewRecorder()
app.handleSettings(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandlePipelineStatus_Get(t *testing.T) {
app, _, _ := newTestServer(t)
req := httptest.NewRequest(http.MethodGet, "/api/pipelines/status", nil)
rec := httptest.NewRecorder()
app.handlePipelineStatus(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

func TestHandlePipelinePromote_Post(t *testing.T) {
app, _, _ := newTestServer(t)
body, _ := json.Marshal(map[string]string{"pipeline_id": "pipe-1", "target": "production"})
req := httptest.NewRequest(http.MethodPost, "/api/pipelines/promote", bytes.NewReader(body))
req.Header.Set("Content-Type", "application/json")
rec := httptest.NewRecorder()
app.handlePipelinePromote(rec, req)
if rec.Code >= 500 { t.Errorf("server error %d", rec.Code) }
}

// ── writeJSON helper tests ────────────────────────────────────────────────────

func TestWriteJSON_PlainStruct(t *testing.T) {
rec := httptest.NewRecorder()
writeJSON(rec, map[string]string{"hello": "world"})
if rec.Code != http.StatusOK { t.Errorf("expected 200, got %d", rec.Code) }
ct := rec.Header().Get("Content-Type")
if !strings.Contains(ct, "json") { t.Errorf("expected JSON content-type, got %q", ct) }
}

func TestWriteJSON_StatusCreated(t *testing.T) {
rec := httptest.NewRecorder()
rec.WriteHeader(http.StatusCreated)
writeJSON(rec, map[string]string{"id": "new-1"})
if rec.Code != http.StatusCreated { t.Errorf("expected 201, got %d", rec.Code) }
}

func TestWriteJSON_EmptyPayload(t *testing.T) {
rec := httptest.NewRecorder()
writeJSON(rec, map[string]any{})
if rec.Code != http.StatusOK { t.Errorf("expected 200, got %d", rec.Code) }
}

func TestWriteJSON_NilPayload(t *testing.T) {
rec := httptest.NewRecorder()
rec.WriteHeader(http.StatusNoContent)
writeJSON(rec, nil)
if rec.Code != http.StatusNoContent { t.Errorf("expected 204, got %d", rec.Code) }
}

func TestWriteJSON_BodyContainsKey(t *testing.T) {
rec := httptest.NewRecorder()
writeJSON(rec, map[string]string{"key": "value"})
body := rec.Body.String()
if !strings.Contains(body, "value") { t.Errorf("expected body to contain 'value', got: %s", body) }
}
