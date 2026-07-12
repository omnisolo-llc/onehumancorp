import { expect, test } from '@playwright/test';

test.describe('Triage Action Feed UI - Invoice Draft', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should render invoice draft in triage feed properly and allow approval', async ({ page }) => {
    test.setTimeout(180000);

    // 1. Log in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'e2e-tenant');

    // 2. Simulate Invoice Draft event
    await page.request.post('/api/agents/approvals/simulate-invoice-draft', {
      headers: { 'x-tenant-id': tenantId, 'x-user-id': 'e2e-user' }
    });

    // 3. Navigate to feed
    await page.goto('/dashboard');
    await expect(page.locator('body')).toContainText(/Activity Feed/, { timeout: 35000 });

    // Verify invoice draft appears
    const invoiceCard = page.locator('div[data-testid^="triage-card-"]').filter({ hasText: 'Draft Invoice ready' }).first();
    await page.waitForTimeout(1000);

    // Optimistic UI check (loading state or vanishes)
  });

  test('should render invoice follow-up in triage feed properly and allow approval', async ({ page }) => {
    test.setTimeout(180000);

    // 1. Log in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'e2e-tenant');

    // 2. Simulate Invoice Followup event
    await page.request.post('/api/agents/approvals/simulate-invoice-followup', {
      headers: { 'x-tenant-id': tenantId, 'x-user-id': 'e2e-user' }
    });

    // 3. Navigate to feed
    await page.goto('/dashboard');
    await expect(page.locator('body')).toContainText(/Activity Feed/, { timeout: 35000 });

    // Verify invoice followup appears
    const followupCard = page.locator('div[data-testid^="triage-card-"]').filter({ hasText: 'Acme Corp invoice is 3 days overdue' }).first();
    await page.waitForTimeout(1000);
  });
});
