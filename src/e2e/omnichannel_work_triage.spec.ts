import { test, expect } from '@playwright/test';

test.describe('Omnichannel Work Triage CUJ', () => {
  test.use({ viewport: { width: 375, height: 812 } }); // Mobile viewport as required

  test('Maya can see and reply to WhatsApp and IG DM inquiries in Work Triage', async ({ page }) => {
    test.setTimeout(180000);

    // 1. Log in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'e2e-tenant');

    // 2. Seed omnichannel inquiries
    const seedData = [
      {
        source: 'Instagram DM',
        priority: 'high',
        context: 'Message: Customer asked about vegan cakes.',
        action_type: 'Draft Reply',
        action_payload: 'Yes, we have vegan options.',
        customer_id: 'maya_ig_cust'
      },
      {
        source: 'WhatsApp',
        priority: 'medium',
        context: 'Message: Customer wants a custom quote.',
        action_type: 'Draft Quote',
        action_payload: 'Drafting custom quote for $50.',
        customer_id: 'maya_wa_cust'
      }
    ];

    for (const data of seedData) {
      await page.request.post(`/api/v1/triage/create?tenant_id=${encodeURIComponent(tenantId)}`, {
        data
      });
    }

    // 3. Navigate to Triage Feed from Home Dashboard
    await page.goto('/triage');
    await expect(page.locator('body')).toContainText(/Work Triage/, { timeout: 15000 });

    // 4. Verify Instagram DM is visible
    const igCard = page.locator('div[data-testid^="triage-card-"]', { hasText: 'Instagram DM' }).first();
    await expect(igCard).toBeVisible();

    // 5. Verify WhatsApp is visible
    const waCard = page.locator('div[data-testid^="triage-card-"]', { hasText: 'WhatsApp' }).first();
    await expect(waCard).toBeVisible();

    // 6. Review and Approve Draft Reply
    const igTestId = await igCard.getAttribute('data-testid');
    const igHeader = igCard.locator(`[data-testid="triage-card-header-${igTestId?.replace("triage-card-", "")}"]`);
    await igHeader.click();

    const reviewBtn = igCard.locator(`button[data-testid="triage-review-btn-${igTestId?.replace("triage-card-", "")}"]`);
    await reviewBtn.waitFor({ state: 'visible' });
    await reviewBtn.click();

    const saveBtn = igCard.locator(`button[data-testid="triage-save-btn-${igTestId?.replace("triage-card-", "")}"]`);
    await saveBtn.waitFor({ state: 'visible' });
    await saveBtn.click();

    await expect(igCard).not.toBeVisible({ timeout: 15000 });
  });
});
