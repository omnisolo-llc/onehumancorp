import { test, expect } from './fixtures';

test.describe('Department Orchestration CUJ', () => {

  test('Persona: Maya the Baker configures her Ambassador agent', async ({ page, request, tenantId }) => {
    // 1. Owner opens the Team page
    await page.goto('/team');
    await expect(page.getByRole('heading', { name: /Your Team/i })).toBeVisible();

    // 2. Owner clicks on "The Ambassador" (Customer Success department)
    await page.getByRole('button', { name: /The Ambassador/i }).click();
    await expect(page.getByRole('heading', { name: /The Ambassador/i })).toBeVisible();

    // 3. Verify the "Review all messages before sending" toggle exists
    await expect(page.getByText('Review all messages before sending')).toBeVisible();

    // 4. Toggle it to "Auto-execute"
    // We click the button to turn it OFF.
    await page.locator('div').filter({ hasText: 'Review all messages before sending' }).getByRole('button').click();

    // Give it a moment to save
    await page.waitForTimeout(1000);

    // 5. Trigger an inbound message event
    const webhookRes1 = await request.post('/api/agents/webhook', {
      data: {
        tenant_id: tenantId,
        message: "Do you do vegan cakes?",
        source: "instagram"
      }
    });
    expect(webhookRes1.ok()).toBeTruthy();

    // 6. Wait briefly and verify that because the setting is "Auto-execute", the inbox remains empty of new pending drafts.
    await page.waitForTimeout(2000);
    await page.reload();
    await page.getByRole('button', { name: /The Ambassador/i }).click();
    await expect(page.getByText('All caught up!')).toBeVisible();

    // 7. Toggle the setting back to "Draft-for-review".
    await page.locator('div').filter({ hasText: 'Review all messages before sending' }).getByRole('button').click();
    await page.waitForTimeout(1000);

    // 8. Send another message via the webhook
    const webhookRes2 = await request.post('/api/agents/webhook', {
      data: {
        tenant_id: tenantId,
        message: "What are your prices?",
        source: "instagram"
      }
    });
    expect(webhookRes2.ok()).toBeTruthy();

    // 9. Reload or poll the page and verify a pending approval (Draft email for review) now appears in the UI.
    await page.waitForTimeout(2000);
    await page.reload();
    await page.getByRole('button', { name: /The Ambassador/i }).click();
    await expect(page.getByText('Incoming message')).toBeVisible();

  });
});
