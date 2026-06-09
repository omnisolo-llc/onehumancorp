import { test, expect } from '@playwright/test';

test.describe('Sales Pipeline E2E', () => {
  test('Incoming high-intent message creates a Lead and Opportunity, and shows in Pipeline', async ({ page }) => {
    const tenantId = 'test-pipeline-tenant';
    await page.goto('/dashboard');
    await page.evaluate((tid) => {
      localStorage.setItem('tenant_id', tid);
    }, tenantId);

    // Simulate an incoming webhook message
    const webhookRes = await page.request.post('/api/v1/webhooks/meta', {
      data: {
        object: "whatsapp_business_account",
        entry: [{
          id: "12345",
          changes: [{
            value: {
              messaging_product: "whatsapp",
              metadata: { display_phone_number: "15555555555" },
              messages: [{
                from: "15551234567",
                id: "msg_abc123",
                timestamp: "1600000000",
                type: "text",
                text: { body: "I am interested in getting a quote for a custom branding project." }
              }]
            }
          }]
        }]
      }
    });

    // Check if the webhook responded successfully
    expect(webhookRes.ok()).toBeTruthy();

    // Wait for the message processing pipeline to complete
    await page.waitForTimeout(3000);

    // Navigate to pipeline
    await page.goto('/pipeline');
    await page.waitForSelector('h3:has-text("Sales Pipeline")');

    // Find the new opportunity in the Qualified column
    const qualifiedCol = page.locator('div.snap-center').filter({ hasText: 'Qualified' });

    // Assert there's at least one opportunity card
    await expect(qualifiedCol.locator('[data-testid^="pipeline-card-"]')).toHaveCount(1);

    // Optionally check if the title mentions "Quote"
    const oppCard = qualifiedCol.locator('[data-testid^="pipeline-card-"]').first();
    await expect(oppCard).toContainText('Quote');
  });
});
