import { test, expect } from './fixtures';

test.describe('Ambassador Auto-Responder CUJ', () => {
  test('Owner approves Ambassador drafted reply to a customer message', async ({ page, request }) => {
    test.skip(process.env.CI === 'true', 'Skip in CI until webhook endpoint runs locally');

    // We assume the user is already logged in due to the global setup / fixtures.
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 15000 });

    // Note: If /integrations page does not mock the OAuth connect correctly in the new app structure,
    // we may skip the Meta Graph connect step to focus purely on the Agent logic.

    // 2. Trigger the Ambassador's draft reply via a real API call
    // The CustomerSuccess agent listens for tenant.message.received, which is triggered via the webhook endpoint
    const tenantId = 'e2e-tenant'; // Match the standard fixture tenant
    const webhookPayload = {
      tenant_id: tenantId,
      message: 'Do you have vegan chocolate cake available for Saturday?',
      source: 'instagram'
    };

    const response = await request.post('/api/agents/webhook', {
      data: webhookPayload,
    });
    expect(response.ok()).toBeTruthy();

    // 3. Navigate to Team Page / Agents Approval Inbox
    await page.goto('/team');
    // Using generic wait, it could be "Your Team" or "AI Departments" depending on the app's current labels.

    // Navigate to The Ambassador
    await page.getByRole('button', { name: 'The Ambassador' }).first().click();

    // Ensure we are viewing the Ambassador inbox specifically
    await expect(page.getByRole('heading', { name: 'The Ambassador' })).toBeVisible({ timeout: 5000 });

    // Wait for the specific inquiry text to appear, indicating the drafted card is loaded
    const inquiryLocator = page.getByText('Do you have vegan chocolate cake available for Saturday?').first();
    await expect(inquiryLocator).toBeVisible({ timeout: 15000 });

    // Click Approve
    await page.getByRole('button', { name: 'Approve' }).first().click();

    // Validate empty state or removal
    await expect(page.getByText('Do you have vegan chocolate cake available for Saturday?')).toBeHidden();
  });
});
