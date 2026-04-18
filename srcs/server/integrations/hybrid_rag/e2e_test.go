package hybrid_rag_test

import (
	"context"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/api"
	"github.com/onehumancorp/mono/srcs/server/integrations/hybrid_rag"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestHybridDelegation_E2E(t *testing.T) {
	// 1. Set up the Cloud Handler (Mocked Server)
	hub := orchestration.NewHub()

	// Create an in-memory SIPDB
	sipDB, err := orchestration.NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("Failed to create in-memory SIPDB: %v", err)
	}
	hub.SetSIPDB(sipDB)

	handler := api.HandleHybridDelegation(hub)
	server := httptest.NewServer(handler)
	defer server.Close()

	// 2. Set up the Local RAG Delegator (Client) pointing to the mock server
	delegator := hybrid_rag.NewLocalDelegator(server.URL, "dummy-key")

	// 3. Simulate a task exceeding local thresholds
	localCtx := hybrid_rag.RAGContext{
		OriginalQuery: "Analyze financial history",
		RawContent:    "Private Bank Statements for 2023",
		Embeddings:    []float64{0.1, 0.2, 0.3},
	}

	// 4. Perform delegation
	missionID, err := delegator.DelegateToCloud(context.Background(), localCtx)
	if err != nil {
		t.Fatalf("Failed to delegate mission: %v", err)
	}

	// 5. Verify the client received a valid MissionID
	if missionID == "" {
		t.Errorf("Expected valid MissionID, got empty string")
	}

	// The mock Cloud Handler returns a success status if parsing and 'inserting' (mocked) works.
	t.Logf("Successfully delegated mission, received ID: %s", missionID)
}
