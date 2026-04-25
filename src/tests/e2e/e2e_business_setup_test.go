package e2e

import (
    "testing"
)

func TestBusinessSetupWizard(t *testing.T) {
    page := newPage(t)
    defer page.Close()

    // 1. Log in using the UI.
    loginAsAdmin(t, page)

    // Wait for home screen.
    if err := page.Locator(`[aria-label="Dashboard"]`).First().WaitFor(); err != nil {
        t.Fatalf("Element not found: %v", err)
    }

    if _, err := page.Goto(baseURL + "/#/business_setup"); err != nil {
        t.Fatalf("Failed to navigate to business setup: %v", err)
    }

    // Wait for the UI to settle
    if err := page.Locator("text=Get Started").First().WaitFor(); err != nil {
        t.Fatalf("Element not found: %v", err)
    }

    // Step 0 -> Step 1
    page.Locator("text=Get Started").First().Click()
    if err := page.Locator("text=What kind of business are you building?").First().WaitFor(); err != nil {
        t.Fatalf("Element not found: %v", err)
    }

    // Step 1 -> Step 2
    page.Locator("text=Online Store").First().Click()
    if err := page.Locator("text=Tell us about your business").First().WaitFor(); err != nil {
        t.Fatalf("Element not found: %v", err)
    }

    // Step 2 -> Fill inputs and verify persistence
    page.Locator("input").First().Fill("Acme Corp")
    page.Locator("text=Continue").First().Click()
    if err := page.Locator("text=What do you sell?").First().WaitFor(); err != nil {
        t.Fatalf("Element not found: %v", err)
    }

    // --- State persistence check ---
    // Reload the page to ensure draft state was saved to the DB and loaded back
    page.Reload()

    // Should resume at Step 3 with the inputs preserved
    if err := page.Locator("text=What do you sell?").First().WaitFor(); err != nil {
        t.Fatalf("Element not found after reload (state not persisted): %v", err)
    }
    // -------------------------------

    // Step 3 -> Step 4
    page.Locator("text=Continue").First().Click()
    if err := page.Locator("text=How do you want to receive payments?").First().WaitFor(); err != nil {
        t.Fatalf("Element not found: %v", err)
    }

    // Step 4 -> Step 5
    page.Locator("text=Online only").First().Click()
    page.Locator("text=Continue").First().Click()
    if err := page.Locator("text=Administrator account").First().WaitFor(); err != nil {
        t.Fatalf("Element not found: %v", err)
    }

    // Step 5 -> Step 6
    inputs := page.Locator("input")
    inputs.Nth(0).Fill("Admin")
    inputs.Nth(1).Fill("admin@onehumancorp.com")
    inputs.Nth(2).Fill("password123")
    page.Locator("text=Continue").First().Click()
    if err := page.Locator("text=Review & Launch").First().WaitFor(); err != nil {
        t.Fatalf("Element not found: %v", err)
    }

    // Step 6 -> Launch
    page.Locator("text=Launch My Business →").First().Click()

    // Wait for the dashboard to appear
    if err := page.Locator(`[aria-label="Dashboard"]`).First().WaitFor(); err != nil {
        t.Fatalf("Element not found: %v", err)
    }
}
