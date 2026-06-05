import { test, expect } from '@playwright/test';

test.describe('Ambassador Auto-Responder CUJ', () => {
  test('Owner connects Meta Graph API and approves Ambassador drafted reply', async ({ page, request }) => {
    test.skip(process.env.OHC_API_ONLY_E2E === 'true', 'API-only E2E tests do not run frontend routes');
    await page.goto('http://localhost:3000/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Login' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    await page.goto('http://localhost:3000/integrations');

    page.on('dialog', dialog => dialog.accept());

    const metaCard = page.locator('div').filter({ hasText: 'Meta Graph API' }).first();
    const connectMetaButton = metaCard.locator('button:has-text("Connect")');
    await connectMetaButton.click();

    await expect(metaCard.locator('button:has-text("Manage")')).toBeVisible();

    const tenantId = 'team-default';
    const webhookPayload = {
      tenant_id: tenantId,
      message: 'Do you have vegan chocolate cake available for Saturday?',
      source: 'instagram'
    };

    const response = await request.post('http://localhost:8080/api/agents/webhook', {
      data: webhookPayload,
    });

    expect(response.ok()).toBeTruthy();

    await page.goto('http://localhost:3000/team');
    await expect(page.getByRole('heading', { name: 'Your Team', exact: true })).toBeVisible();

    await page.getByRole('button', { name: 'The Ambassador' }).first().click();

    await expect(page.getByRole('heading', { name: 'The Ambassador' })).toBeVisible({ timeout: 5000 });

    const inquiryLocator = page.getByText('Do you have vegan chocolate cake available for Saturday?').first();
    await expect(inquiryLocator).toBeVisible({ timeout: 15000 });

    await page.getByRole('button', { name: 'Approve' }).first().click();

    await expect(page.getByText('Do you have vegan chocolate cake available for Saturday?')).toBeHidden();
  });
});
