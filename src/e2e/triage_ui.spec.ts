import { test, expect } from './fixtures';

test.describe('Work Triage Agentic Inbox', () => {
  const tenantId = 'e2e-tenant';

  test('Owner reviews and approves a triage item', async ({ page }) => {
    await page.goto('/dashboard');

    await expect(page.locator('h2').filter({ hasText: 'Unified Agent Feed' })).toBeVisible({ timeout: 15000 });

    const triageCard = page.locator('[data-testid="triage-card-triage-test-1"]');
    await expect(triageCard).toBeVisible({ timeout: 10000 });

    await triageCard.click();

    await expect(page.locator('text=Maya requested a custom cake')).toBeVisible();
    await expect(page.locator('text=Draft Reply')).toBeVisible();

    const approveBtn = triageCard.locator('[data-testid="approve-btn"]');
    await expect(approveBtn).toBeVisible({ timeout: 5000 });
    await approveBtn.click();

    await expect(triageCard).not.toBeVisible();
  });

  test('Owner sees empty state when there are no items', async ({ page, request }) => {
    // Override tenant for this test
    const loginRes = await request.post('/api/auth/login', {
      data: { username: 'test@example.com', password: 'password123', organization_id: 'empty-tenant-triage-test' },
    });
    const { token } = await loginRes.json();
    await page.evaluate((t) => localStorage.setItem('token', t), token);
    await page.evaluate((t) => localStorage.setItem('tenant_id', t), 'empty-tenant-triage-test');

    await page.goto('/dashboard');

    await expect(page.locator('text=Business Analytics')).toBeVisible({ timeout: 15000 });

    await page.waitForTimeout(2000);
    await expect(page.locator('h2').filter({ hasText: 'Unified Agent Feed' })).not.toBeVisible();
  });



  test('Owner can dismiss a triage item', async ({ page }) => {
    await page.goto('/dashboard');

    const triageCard = page.locator('[data-testid="triage-card-triage-test-2"]');
    await expect(triageCard).toBeVisible({ timeout: 15000 });

    await triageCard.click();

    const dismissBtn = triageCard.locator('[data-testid="dismiss-btn"]');
    await expect(dismissBtn).toBeVisible({ timeout: 5000 });
    await dismissBtn.click();

    await expect(triageCard).not.toBeVisible();
  });

  test('Triage feed renders items correctly', async ({ page }) => {
    await page.goto('/dashboard');

    const triageCard = page.locator('[data-testid="triage-card-triage-test-1"]');
    await expect(triageCard).toBeVisible({ timeout: 15000 });

    await expect(triageCard.locator('text=Instagram DM')).toBeVisible();
    await expect(triageCard.locator('text=Urgent')).toBeVisible();
  });

  test('Triage detail shows correct information on click', async ({ page }) => {
    await page.goto('/dashboard');

    const triageCard = page.locator('[data-testid="triage-card-triage-test-2"]');
    await expect(triageCard).toBeVisible({ timeout: 15000 });

    await triageCard.click();
    await expect(page.locator('text=WhatsApp')).toBeVisible();
    await expect(page.locator('text=Question about delivery times')).toBeVisible();
  });


});
