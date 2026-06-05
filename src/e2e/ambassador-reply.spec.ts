import { test, expect } from './fixtures';

test.describe('Ambassador Auto-Responder CUJ', () => {
  test('Owner approves Ambassador drafted reply to a customer message', async ({ page, request }) => {
    test.skip(process.env.CI === 'true', 'Skip in CI until webhook endpoint runs locally');

    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 15000 });

    const tenantId = 'e2e-tenant';
    const webhookPayload = {
      tenant_id: tenantId,
      message: 'Do you have vegan chocolate cake available for Saturday?',
      source: 'instagram'
    };

    const response = await request.post('/api/agents/webhook', {
      data: webhookPayload,
    });
    expect(response.ok()).toBeTruthy();

    await page.goto('/team');

    await page.getByRole('button', { name: 'The Ambassador' }).first().click();

    await expect(page.getByRole('heading', { name: 'The Ambassador' })).toBeVisible({ timeout: 5000 });

    const inquiryLocator = page.getByText('Do you have vegan chocolate cake available for Saturday?').first();
    await expect(inquiryLocator).toBeVisible({ timeout: 15000 });

    await page.getByRole('button', { name: 'Approve' }).first().click();

    await expect(page.getByText('Do you have vegan chocolate cake available for Saturday?')).toBeHidden();
  });
});
