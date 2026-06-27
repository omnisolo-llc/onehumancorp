import { test, expect } from '@playwright/test';

test.describe('Ambassador Agent Workflow', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('simulate ambassador draft, verify in feed, edit, and approve', async ({ page }) => {
    // We will just hit the real UI here but it needs the backend running.
    // If the backend is mocked, we can do it via routes.
    // Assuming backend is running because of `bazel test //src/e2e:playwright` wrapper.

    // 1. Navigate to the agent feed
    await page.goto('/feed');

    // Ensure we are caught up initially, or just click the button right away
    // 2. Trigger the simulation (using webhook to simulate real flow instead of the simulate button)
    const apiUrl = process.env.VITE_API_URL || 'http://127.0.0.1:18789';
    const response = await page.request.post(`${apiUrl}/api/v1/webhooks/omnichannel`, {
      data: {
        tenant_id: 'test-tenant',
        source: 'instagram',
        identifier: '@customer_webhook_test',
        message: 'Do you have vegan chocolate cake available for Saturday?'
      }
    });
    expect(response.status()).toBe(200);

    // Wait for the triage worker to process and websocket to push the card
    // We can also trigger the legacy simulation button if the environment doesn't have the full worker running,
    // but the task asks to simulate webhook ingestion. We'll wait a bit.

    // 3. Verify the action card appears
    const feedCard = page.getByTestId('agent-feed-card').filter({ hasText: 'Do you have vegan chocolate cake available for Saturday?' }).first();
    await expect(feedCard).toBeVisible({ timeout: 15000 });

    // Verify specific Ambassador UI elements
    await expect(feedCard).toContainText('CUSTOMER MESSAGE');
    // Note: LLM draft response text may vary because it actually goes through the router in the webhook path now,
    // so we shouldn't strictly assert the exact string unless it's deterministically mocked.
    // We will assert the draft area is present.
    await expect(feedCard).toContainText('Agent Draft');

    // 4. Click 'Edit'
    const editBtn = feedCard.getByTestId('feed-edit-btn');
    await expect(editBtn).toContainText('Edit');
    await editBtn.click();

    // Verify textarea appears
    const textarea = page.getByTestId('feed-edit-input');
    await expect(textarea).toBeVisible();

    // Modify the text
    await textarea.fill('Yes we do! We have 3 left for this Saturday. I can set aside one for you.');

    const saveBtn = page.getByTestId('feed-save-edit-btn');
    await saveBtn.click();

    // Wait for the textarea to be hidden
    await expect(textarea).not.toBeVisible();
    await expect(feedCard).toContainText('Yes we do! We have 3 left for this Saturday. I can set aside one for you.');

    // 5. Click 'Approve & Send Draft'
    const approveBtn = feedCard.getByTestId('feed-approve-btn');
    await expect(approveBtn).toContainText('Send Draft');
    await approveBtn.click();

  });
});
