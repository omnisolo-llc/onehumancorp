import { test, expect } from '@playwright/test';

test.describe('Smart Pricing autonomous workflow', () => {
  const tenantId = `tenant-${Math.random().toString(36).substring(7)}`;
  const productId = `prod-${Math.random().toString(36).substring(7)}`;

  test('should display Smart Pricing approval and apply price change on approval', async ({ page, request }) => {
    // 1. Mock DB by creating tenant and product directly
    await request.post('http://127.0.0.1:8081/api/onboarding/start', {
      data: {
        organization_id: tenantId,
        business_type: 'Boutique',
        company_name: 'Priya Boutique'
      }
    });

    await request.post('http://127.0.0.1:8081/api/v1/product', {
      headers: {
        'x-tenant-id': tenantId,
        'x-user-id': 'admin'
      },
      data: {
        id: productId,
        name: 'Winter Scarf',
        description: 'A cozy scarf',
        price_cents: 5000, // $50.00
        inventory_count: 10,
        item_type: 'Product'
      }
    });

    // We can also trigger the cron via mesh endpoint (we know HubService handles it)
    await request.post('http://127.0.0.1:8081/api/mesh/publish', {
      headers: {
        'x-tenant-id': tenantId,
        'x-user-id': 'admin'
      },
      data: {
        topic: 'tenant.inventory.analyze_stagnant',
        event_id: `evt-${Date.now()}`,
        payload: {
          product_id: productId,
          product_name: 'Winter Scarf',
          price: 50.00
        }
      }
    });

    // Login via UI
    await page.goto('http://127.0.0.1:3000/login');
    await page.fill('input[type="email"]', `${tenantId}@example.com`);
    await page.fill('input[type="password"]', 'password123');
    await page.click('button[type="submit"]');

    // Make sure we are on dashboard
    await expect(page).toHaveURL(/.*\/dashboard.*/);

    // Wait for the API call to complete instead of using a hardcoded timeout
    await page.waitForResponse(response =>
      response.url().includes('/api/agents/approvals') && response.status() === 200
    );

    // Assert the Smart Pricing card is there
    await expect(page.locator('text=Smart Price Suggestion: Winter Scarf')).toBeVisible({ timeout: 10000 });

    // Approve
    await page.click('[data-testid="approve-run-sale"]');

    // The item should move to Activity Feed
    await page.click('button:has-text("Activity Feed")');
    await expect(page.locator('text=Smart Price Suggestion: Winter Scarf')).toBeVisible({ timeout: 10000 });
  });
});
