package dashboard

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/agents"
	"github.com/onehumancorp/mono/srcs/server/billing"
	"github.com/onehumancorp/mono/srcs/server/domain"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

type fakeWorkerController struct {
	provisioned   []string
	deprovisioned []string
	provisionErr  error
}

func (f *fakeWorkerController) EnsureProvisioned(_ context.Context, agent orchestration.Agent) error {
	f.provisioned = append(f.provisioned, agent.ID)
	return f.provisionErr
}

func (f *fakeWorkerController) Deprovision(_ context.Context, agentID string) error {
	f.deprovisioned = append(f.deprovisioned, agentID)
	return nil
}

func TestHandleHireAgent_ProvisionedBuiltinWorker(t *testing.T) {
	org := domain.NewSoftwareCompany("test-org", "Test Org", "CEO", time.Now().UTC())
	hub := orchestration.NewHub()
	defer hub.Close()

	app := &Server{
		org: org,
		roleProfileCache: map[string]domain.RoleProfile{
			string(domain.RoleSoftwareEngineer): {Role: domain.RoleSoftwareEngineer},
		},
		hub:                   hub,
		tracker:               billing.NewTracker(billing.DefaultCatalog),
		agentProviderRegistry: agents.DefaultRegistry(),
		workerController:      &fakeWorkerController{},
	}
	defer app.tracker.Close()

	rec := httptest.NewRecorder()
	req := httptest.NewRequest(http.MethodPost, "/api/agents/hire", bytes.NewBufferString(`{"name":"Alice","role":"SOFTWARE_ENGINEER"}`))

	app.handleHireAgent(rec, req)

	if rec.Code != http.StatusOK {
		t.Fatalf("expected 200, got %d: %s", rec.Code, rec.Body.String())
	}

	controller := app.workerController.(*fakeWorkerController)
	if len(controller.provisioned) != 1 {
		t.Fatalf("expected 1 provisioned worker, got %d", len(controller.provisioned))
	}

	var snapshot dashboardSnapshot
	if err := json.Unmarshal(rec.Body.Bytes(), &snapshot); err != nil {
		t.Fatalf("decode snapshot: %v", err)
	}
	if len(snapshot.Agents) != 1 {
		t.Fatalf("expected 1 agent, got %d", len(snapshot.Agents))
	}
	if !snapshot.Agents[0].Managed {
		t.Fatalf("expected hired builtin agent to be managed")
	}
	if snapshot.Agents[0].Region == "" {
		t.Fatalf("expected hired builtin agent to have a runtime region")
	}
}

func TestHandleFireAgent_DeprovisionsWorker(t *testing.T) {
	org := domain.NewSoftwareCompany("test-org", "Test Org", "CEO", time.Now().UTC())
	hub := orchestration.NewHub()
	defer hub.Close()
	controller := &fakeWorkerController{}

	app := &Server{
		org: org,
		roleProfileCache: map[string]domain.RoleProfile{
			string(domain.RoleSoftwareEngineer): {Role: domain.RoleSoftwareEngineer},
		},
		hub:                   hub,
		tracker:               billing.NewTracker(billing.DefaultCatalog),
		agentProviderRegistry: agents.DefaultRegistry(),
		workerController:      controller,
	}
	defer app.tracker.Close()

	hub.RegisterAgent(orchestration.Agent{
		ID:             org.ID + "-agent-1",
		Name:           "Alice",
		Role:           string(domain.RoleSoftwareEngineer),
		OrganizationID: org.ID,
		Status:         orchestration.StatusIdle,
		ProviderType:   string(agents.ProviderTypeBuiltin),
		Region:         "process",
		Managed:        true,
	})

	rec := httptest.NewRecorder()
	req := httptest.NewRequest(http.MethodPost, "/api/agents/fire", bytes.NewBufferString(`{"agentId":"`+org.ID+`-agent-1"}`))

	app.handleFireAgent(rec, req)

	if rec.Code != http.StatusOK {
		t.Fatalf("expected 200, got %d: %s", rec.Code, rec.Body.String())
	}
	if len(controller.deprovisioned) != 1 || controller.deprovisioned[0] != org.ID+"-agent-1" {
		t.Fatalf("expected deprovision for %s-agent-1, got %#v", org.ID, controller.deprovisioned)
	}
	if _, ok := hub.Agent(org.ID + "-agent-1"); ok {
		t.Fatalf("expected fired agent to be removed from hub")
	}
}
