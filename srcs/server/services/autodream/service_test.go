package autodream

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	pb "github.com/onehumancorp/mono/srcs/proto/autodream"
)

func TestAutoDreamService(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	// Create table
	_, err := provider.Exec(ctx, `
		CREATE TABLE autodream_findings (
			id TEXT PRIMARY KEY,
			timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			content TEXT NOT NULL,
			embedding TEXT
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	repo := db.NewAutoDreamRepository(provider)
	service := NewAutoDreamService(repo)

	// Test StoreFinding
	storeReq := &pb.StoreFindingRequest{
		Finding: &pb.Finding{
			Id:        "f1",
			Timestamp: time.Now().Format(time.RFC3339),
			Content:   "Test Content",
			Embedding: []float32{1.0, 0.0, 0.0},
		},
	}

	storeResp, err := service.StoreFinding(ctx, storeReq)
	if err != nil {
		t.Fatalf("StoreFinding failed: %v", err)
	}
	if !storeResp.Success {
		t.Errorf("expected success to be true")
	}

	// Test SearchFindings
	searchReq := &pb.SearchFindingsRequest{
		QueryEmbedding: []float32{1.0, 0.0, 0.0},
		Limit:          1,
	}

	searchResp, err := service.SearchFindings(ctx, searchReq)
	if err != nil {
		t.Fatalf("SearchFindings failed: %v", err)
	}

	if len(searchResp.Findings) != 1 {
		t.Fatalf("expected 1 finding, got %d", len(searchResp.Findings))
	}

	if searchResp.Findings[0].Id != "f1" {
		t.Errorf("expected finding id f1, got %s", searchResp.Findings[0].Id)
	}
}
