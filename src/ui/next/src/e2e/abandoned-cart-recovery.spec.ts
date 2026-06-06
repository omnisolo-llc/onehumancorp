
import { test, expect } from '@playwright/test';

test.describe('Automated Cart Recovery Agent', () => {
  test('Owner approves automated cart recovery draft from feed', async ({ page, request }) => {
    // 1. Log in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('maya@ohc.test');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Login' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    // 2. Trigger the Abandoned Cart webhook payload
    const tenantId = 'e2e-tenant';
    const webhookPayload = {
      tenant_id: tenantId,
      source: 'system',
      message: 'abandoned_cart',
      cart_value: '50.00',
      customer_name: 'Sarah'
    };

    const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || process.env.BASE_URL || '';

    // We expect this to hit the real backend
    const response = await request.post(`${apiBase}/api/agents/webhook`, {
      data: webhookPayload,
    });

    // In local dev without backend, this will fail. That is intended by the reviewer.
    expect(response.ok()).toBeTruthy();

    // 3. Navigate to Team Page
    await page.goto('/team');
    await expect(page.getByRole('heading', { name: 'Your Team', exact: true })).toBeVisible();

    // Navigate to The Ambassador
    await page.getByRole('button', { name: 'The Ambassador' }).first().click();
    await expect(page.getByRole('heading', { name: 'The Ambassador' })).toBeVisible({ timeout: 5000 });

    // strictly expect the UI to show the dynamic cart information
    const inquiryLocator = page.getByText('Sarah left $50.00 in their cart.');

    // We must expect it directly so it fails if the webhook didn't process
    await expect(inquiryLocator).toBeVisible({ timeout: 15000 });

    const approveButton = page.getByRole('button', { name: 'Approve' }).first();
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // Validate removal of the card after approval
    await expect(inquiryLocator).toBeHidden();
  });
});
