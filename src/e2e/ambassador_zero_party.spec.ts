import { test, expect } from '@playwright/test';

test.describe('Ambassador Zero-Party Data Collection CUJ', () => {
  test('Owner sees updated customer preferences after Ambassador processes an omnichannel message', async ({ page, request }) => {
    // 1. Simulate the webhook from an Instagram DM where a customer mentions a preference
    const tenantId = 'e2e-tenant';
    const customerId = 'cust-zero-party-456';
    const webhookPayload = {
      tenant_id: tenantId,
      message: 'Hi, do you have any gluten-free options? I have celiac.',
      source: 'instagram',
      sender_id: customerId
    };

    const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || process.env.BASE_URL || '';

    // Post to the webhook to trigger the Ambassador agent
    const response = await request.post(`${apiBase}/api/agents/webhook`, {
      data: webhookPayload,
    });
    expect(response.ok()).toBeTruthy();

    // 2. We must wait for the async orchestration to process the message and LLM to extract tags.
    // In e2e tests against real systems, this might take a few seconds. We'll add a short delay and rely on Playwright's retries.
    await page.waitForTimeout(5000);

    // 3. Navigate to the Customer Profile page
    await page.goto(`/customers/${customerId}`);

    // 4. Verify the glassmorphism profile renders
    await expect(page.getByRole('heading', { name: 'Customer Profile' })).toBeVisible();

    // 5. Verify the preference was successfully extracted and saved to Customer360
    await expect(page.getByText('Known Preferences')).toBeVisible();

    // We expect the LLM to extract "gluten-free" and/or "celiac".
    // Playwright will retry this assertion until it becomes true or times out.
    await expect(page.locator('text=/gluten\\-free|celiac/i').first()).toBeVisible({ timeout: 15000 });
  });
});
