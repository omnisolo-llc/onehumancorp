import { test, expect } from '@playwright/test';
import { v4 as uuidv4 } from 'uuid';

test.describe('Omnichannel Unified Customer Memory Graph UI', () => {
  const tenantId = `tenant_${uuidv4().substring(0, 8)}`;
  const customerId = uuidv4();

  test('should show error when customerId is missing from query parameters', async ({ page }) => {
    await page.route('**/api/inbox/summary/**', async route => {
      await route.fulfill({ status: 404, body: 'Not found' });
    });
    await page.goto(`/customer/memory-graph?tenantId=${tenantId}`);
    await expect(page.getByText('Customer not found.').or(page.getByText('Missing customer ID'))).toBeVisible();
  });

  test('should show error when tenantId is missing from query parameters', async ({ page }) => {
    await page.goto(`/customer/memory-graph?customerId=${customerId}`);
    await expect(page.getByText('Customer not found.').or(page.getByText('Missing tenant ID'))).toBeVisible();
  });

  test('should be responsive and show error text on mobile viewport (375x667)', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto(`/customer/memory-graph?tenantId=${tenantId}&customerId=${customerId}`);
    await expect(page.locator('text=Failed to fetch customer history.').or(page.locator('text=Loading customer history...'))).toBeVisible();
    await expect(page.getByText('Customer not found.')).toBeVisible();
  });

  test('should be responsive and show error text on tablet viewport (768x1024)', async ({ page }) => {
    await page.setViewportSize({ width: 768, height: 1024 });
    await page.goto(`/customer/memory-graph?tenantId=${tenantId}&customerId=${customerId}`);
    await expect(page.locator('text=Failed to fetch customer history.').or(page.locator('text=Loading customer history...'))).toBeVisible();
    await expect(page.getByText('Customer not found.')).toBeVisible();
  });

  test('should show loading state temporarily before network failure resolves', async ({ page }) => {
    await page.route('**/api/inbox/summary/**', async route => {
      await new Promise(resolve => setTimeout(resolve, 500));
      await route.fulfill({ status: 404, body: 'Not found' });
    });

    await page.goto(`/customer/memory-graph?tenantId=${tenantId}&customerId=${customerId}`);
    await expect(page.locator('text=Loading customer history...')).toBeVisible();
    await expect(page.getByText('Customer not found.')).toBeVisible();
  });

  test('should display the memory graph correctly for an existing customer', async ({ page, request }) => {
    // We mock the API to return a successful 200 response with timeline data to test the happy path UI
    await page.route('**/api/inbox/summary/**', async route => {
      await route.fulfill({
        status: 200,
        json: {
          customer_name: "John Doe",
          summary: "This is a great customer.",
          interactions: [
             { channel: "email", description: "Sent an inquiry about cakes", created_at: "2026-07-14T10:00:00Z" },
             { channel: "ig_dm", description: "Followed up via Instagram", created_at: "2026-07-15T11:30:00Z" }
          ],
          total_interactions: 2,
          segments: ["Returning"],
          preferences: ["Vegan"]
        }
      });
    });

    // Navigate to the memory graph page
    await page.goto(`/customer/memory-graph?tenantId=${tenantId}&customerId=${customerId}`);

    // Expect the data to be rendered correctly
    await expect(page.getByText('Customer Context')).toBeVisible();
    await expect(page.getByText('This is a great customer.')).toBeVisible();
    await expect(page.getByText('Sent an inquiry about cakes')).toBeVisible();
    await expect(page.getByText('Followed up via Instagram')).toBeVisible();
    await expect(page.getByText('2 total interactions recorded.')).toBeVisible();
  });
});
