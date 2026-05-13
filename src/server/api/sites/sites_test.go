package sites

import (
	"context"
	"testing"
)

func TestSitesService_GenerateSite(t *testing.T) {
	svc := &SitesService{}
	site, err := svc.GenerateSite(context.Background(), "tenant-1", "bakery")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if site.ID != "site-123" {
		t.Errorf("expected site-123, got %s", site.ID)
	}
}
