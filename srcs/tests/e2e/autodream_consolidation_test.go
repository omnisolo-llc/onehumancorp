package e2e

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
)

func TestE2E_AutoDreamConsolidation(t *testing.T) {
	// This E2E test simulates the full CUJ for architectural consolidation.
	page := newPage(t)
	loginAsAdmin(t, page)

	// 1. Complete several tasks in shared_tasks_decomposition.
	// We use the REST API helper to seed tasks for speed, but the CUJ starts from login.
	seedDevEnvironment(t, "autodream_tasks_ready")

	// Navigate to Tasks page to see them
	navigateTo(t, page, "Tasks")
	page.WaitForLoadState(playwright.PageWaitForLoadStateOptions{State: playwright.LoadStateNetworkidle})

	// 2. Trigger the AutoDream worker's ingestCompletedTasks and consolidateArchitecturalInsights.
	// In a real environment, this runs on a ticker. For the test, we trigger it via an internal dev endpoint if available,
	// or we just wait for the next cycle if the ticker is short.
	// Here we simulate the trigger.
	status, _ := apiPOSTJSON(t, "/api/dev/trigger-autodream", nil)
	assert.Equal(t, 200, status)

	// 3. Verify that architectural insights appear in consolidated_memory.
	// We can check this via the "Insights" or "Memory" tab in the UI.
	navigateTo(t, page, "Insights")

	// Assert that the synthesized insight is visible on the page
	insightLocator := page.Locator(".architectural-insight-card")
	err := insightLocator.First().WaitFor(playwright.LocatorWaitForOptions{State: playwright.Enum("visible"), Timeout: playwright.Float(10000)})
	assert.NoError(t, err, "Architectural insight should be visible in the UI after consolidation")
}
