import { test, expect } from '@playwright/test';

test.describe('Incident Room', () => {
  test('should display incidents and allow execution of resolution plan', async ({ request, page }) => {
    // Seed an incident via API for the test tenant
    const tenantId = 'e2e-test-tenant';

    // Attempt to seed data using backend endpoint directly.
    // Usually playwright has a setup phase, we'll try API direct.
    await request.post(`/api/incidents?tenant_id=${tenantId}`, {
      data: {
        title: 'Espresso Machine Broken',
        description: 'Water leaking from back panel',
      }
    });

    // We set tenantId in localStorage before navigating
    await page.goto('/login'); // Setup storage context
    await page.evaluate((tid) => localStorage.setItem('tenant_id', tid), tenantId);

    // 1. Visit the Incident Room page
    await page.goto('/incident-room');

    // 2. Wait for loading to finish
    await page.waitForSelector('text=Loading incidents...', { state: 'hidden' });

    // 3. Find the first incident card
    const incidentCard = page.locator('[data-testid^="incident-card-"]').first();
    await expect(incidentCard).toBeVisible({ timeout: 10000 });

    // 4. Click the card to open the bottom sheet
    await incidentCard.click();

    // 5. Verify bottom sheet is open
    await expect(page.locator('text=Resolution Plan')).toBeVisible();
    await expect(page.locator('text=Executive Summary')).toBeVisible();
    await expect(page.locator('text=Proposed Actions')).toBeVisible();

    // 6. Click "Execute Plan"
    const executeBtn = page.locator('[data-testid="execute-plan-btn"]');
    await expect(executeBtn).toBeVisible();
    await executeBtn.click();

    // 7. Verify success message
    await expect(page.locator('text=Plan executed successfully.')).toBeVisible();
  });

  test('should display empty state when no incidents are available', async ({ page }) => {
    const tenantId = 'empty-tenant';
    await page.goto('/login');
    await page.evaluate((tid) => localStorage.setItem('tenant_id', tid), tenantId);

    await page.goto('/incident-room');
    await page.waitForSelector('text=Loading incidents...', { state: 'hidden' });

    await expect(page.locator('text=No active incidents.')).toBeVisible();
  });

  test('should close bottomsheet on cancel', async ({ request, page }) => {
    const tenantId = 'e2e-test-tenant-cancel';
    await request.post(`/api/incidents?tenant_id=${tenantId}`, {
      data: {
        title: 'Espresso Machine Broken',
        description: 'Water leaking from back panel',
      }
    });

    await page.goto('/login');
    await page.evaluate((tid) => localStorage.setItem('tenant_id', tid), tenantId);

    await page.goto('/incident-room');
    await page.waitForSelector('text=Loading incidents...', { state: 'hidden' });

    const incidentCard = page.locator('[data-testid^="incident-card-"]').first();
    await incidentCard.click();

    await expect(page.locator('text=Resolution Plan')).toBeVisible();
    await page.locator('text=Cancel').click();

    await expect(page.locator('text=Resolution Plan')).toBeHidden();
  });
});
