import { test, expect } from '@playwright/test';

test.describe('Unified Agent Feed (Mobile MVP) - Real E2E Flow', () => {
  test.use({ viewport: { width: 375, height: 812 } }); // Mobile resolution

  test('generates an action via background task and approves it in the UI', async ({ page, request }) => {
    // 1. Simulate the webhook/agent action background task
    const tenantId = 'default';

    const response = await request.post(`/api/dev/simulate-agent-feed-item?tenant_id=${tenantId}`);
    expect(response.ok()).toBeTruthy();
    const data = await response.json();
    const itemId = data.id;

    // 2. Navigate to the dashboard where UnifiedAgentFeed is rendered
    await page.goto('/dashboard');

    // Wait for the feed section
    const feedSection = page.locator('section[aria-label="Unified Agent Feed"]');
    await expect(feedSection).toBeVisible();

    // The backend endpoint creates an item with context "A new simulated event needs your attention."
    const simulatedCardText = page.locator('text=A new simulated event needs your attention.').first();
    await expect(simulatedCardText).toBeVisible({ timeout: 15000 });

    // Look for the "Approve" button within the card that just popped up
    const card = page.locator('div.glassmorphism').filter({ hasText: 'A new simulated event needs your attention.' }).first();
    const approveButton = card.locator('button', { hasText: 'Approve' }).first();
    await expect(approveButton).toBeVisible();

    // Check touch targets
    const btnBox = await approveButton.boundingBox();
    expect(btnBox?.width).toBeGreaterThanOrEqual(44);
    expect(btnBox?.height).toBeGreaterThanOrEqual(44);

    // 3. Click the Approve button
    await approveButton.click();

    // Verify it disappears (UI optimistic update or refetch)
    await expect(simulatedCardText).not.toBeVisible({ timeout: 15000 });
  });
});
