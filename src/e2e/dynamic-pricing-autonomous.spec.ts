import { test, expect } from '@playwright/test';
import { memberPage as pageTest } from './fixtures';

test.describe('Dynamic Pricing AI Advisory Card', () => {
  pageTest('should display and approve a dynamic pricing recommendation', async ({ page }) => {
    // Navigate to Agent Feed (assuming it's on the dashboard/home or a specific route)
    await page.goto('/feed');

    // We might need to seed a specific feed item via API to ensure it exists for the test,
    // or rely on a fixture. For simplicity, we'll wait for the feed to load.

    // Check if the card is displayed
    const approveBtn = page.getByTestId('feed-approve-pricing-btn');

    // Wait for it, but since it might not be seeded in this basic test, we'll just check if the feed loads
    // In a real scenario we'd seed it via API first

    // Let's seed it via a raw POST request if possible, or just skip the exact click if not present
    const response = await page.request.post('/api/inbox/action_required', {
      data: {
        event_source: "Pricing Agent",
        proposed_action: {
           type: "dynamic_pricing_recommendation",
           target_id: "test-product-123",
           recommendation: "Test recommend",
           action: "create_rule",
           rule_config: {
             name: "Clearance: Test",
             type: "InventoryThreshold",
             config: { threshold: 10, adjustment_percent: -15.0 }
           }
        }
      }
    });

    expect(response.ok()).toBeTruthy();

    await page.reload();

    await expect(page.getByText('Test recommend')).toBeVisible();
    await approveBtn.first().click();

    // Verify it disappears from the feed
    await expect(page.getByText('Test recommend')).not.toBeVisible();
  });
});
