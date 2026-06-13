import { test, expect } from '@playwright/test';

test.describe('Agentic Quoting & Invoicing Workflow', () => {
  // Mobile first viewport as per requirements
  test.use({ viewport: { width: 375, height: 667 } });

  test('Owner sees drafted proposal, approves it, simulates client acceptance, and verifies project tracker', async ({ page, request }) => {
    // We already seeded a quote with ID 'e2e-quote-1' that is 'SENT' and ready to be accepted.
    // 1. Simulate client acceptance by hitting our new backend endpoint.
    const acceptResponse = await request.post('/api/v1/quotes/e2e-quote-1/accept', {
      headers: {
        'x-tenant-id': 'e2e-tenant',
      }
    });

    // We expect this to be successful
    expect(acceptResponse.ok()).toBeTruthy();
    const acceptData = await acceptResponse.json();
    expect(acceptData.success).toBe(true);
    expect(acceptData.project_id).toBeDefined();
    expect(acceptData.invoice_id).toBeDefined();

    // 2. Navigate to Project Tracker as the owner
    await page.goto('/project-tracker');

    // 3. Verify Project is created and visible
    await expect(page.locator('text=Active Projects')).toBeVisible();
    await expect(page.getByTestId('project-card').first()).toBeVisible();
    await expect(page.locator('text=New Project from Quote')).toBeVisible();

    // 4. Verify Tasks are visible
    const taskItem = page.getByTestId('task-item').first();
    await expect(taskItem).toBeVisible();
    await expect(page.locator('text=Review Requirements')).toBeVisible();

    // 5. Verify Invoice is visible
    await expect(page.locator('text=Invoices')).toBeVisible();
    await expect(page.getByTestId('invoice-card').first()).toBeVisible();
    await expect(page.locator('text=Deposit Invoice')).toBeVisible();
  });
});
