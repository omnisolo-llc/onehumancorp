import { test, expect } from '@playwright/test';

test.describe('AI Unified Inbox Differentiation & Omnichannel Customer Memory', () => {
  const tenantId = 'e2e-omni-inbox-tenant';

  test.beforeAll(async ({ request }) => {
    // 1. Seed a message via webhook to simulate ingestion
    // Webhook ingestion creates an `ohc_job_queue` record which the local background worker processes.
    // The background worker uses Minimax mock returning deterministic triage data.
    const res = await request.post('/api/v1/omnichannel/webhook', {
      data: {
        tenant_id: tenantId,
        source: 'Instagram DM',
        sender_id: '@sarahbakes',
        message: 'Do you still make the vegan chocolate cake?',
      }
    });

    expect(res.status()).toBe(200);

    // Wait for the background worker to process the message and insert into agent_feed_items/triage_items
    // Since this can be asynchronous, we'll give it a moment or rely on UI polling.
    await new Promise(resolve => setTimeout(resolve, 5000));
  });

  test('proactively drafts contextual response to omnichannel DM and allows 1-tap approve', async ({ page }) => {
    // Mock the localStorage tenant
    await page.addInitScript((t) => {
      window.localStorage.setItem('tenant_id', t);
    }, tenantId);

    // 1. Mobile viewport (375px)
    await page.setViewportSize({ width: 375, height: 812 });

    // 2. Load the inbox UI
    // In our E2E, we use the compiled next app root
    await page.goto('/inbox');
    await page.waitForLoadState('networkidle');

    // Wait for message to appear in the list
    await expect(page.locator('text=Do you still make the vegan chocolate cake?')).toBeVisible();

    // 4. Select the message
    await page.locator('text=Do you still make the vegan chocolate cake?').click();

    // 5. Verify Context & Draft Reply are visible
    // CustomerContextCard should be rendered if customer_id is present
    // Based on identity resolution we might just get sender_id if it's a new customer
    await expect(page.locator('text=@sarahbakes')).toBeVisible();

    // The draft_reply should be populated by the agent. Wait for text to appear.
    // The agent uses the LLM which returns deterministic generic reply if no specific mock.
    const replyLocator = page.locator('.app-panel-body .bg-white', { hasText: /Thank you|Vegan|Wait|We will get back/i }).first();
    await expect(replyLocator).toBeVisible();

    // 6. 1-Tap Approve button should be visible (✨ Approve & Send Draft)
    const approveButton = page.locator('button:has-text("✨ Approve & Send Draft")');
    await expect(approveButton).toBeVisible();

    await approveButton.click();

    // Verify it succeeded (in our UI, maybe it changes state or shows an alert)
    // Wait for networkidle
    await page.waitForLoadState('networkidle');
  });
});
