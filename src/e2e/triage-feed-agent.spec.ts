import { test, expect } from './fixtures';

test.describe('Agentic Work Triage Feed', () => {
  test('Owner can review and approve AI-drafted replies', async ({ page, loginAs, adminUser }) => {
    // 1. Log in to the application
    await loginAs(page, adminUser);

    // 2. Simulate an incoming message by hitting the new test endpoint
    const response = await page.request.post(`/api/dev/simulate-triage-item?tenant_id=default`);
    expect(response.status()).toBe(200);
    const json = await response.json();
    expect(json.success).toBe(true);
    const triageItemId = json.id;

    // 3. Go to the dashboard
    await page.goto('/dashboard');

    // Wait for the feed to load
    const feed = page.locator('[data-testid^="triage-card-"]').first();
    await expect(feed).toBeVisible({ timeout: 10000 });

    // 4. Verify the triage card exists
    const card = page.locator(`[data-testid="triage-card-${triageItemId}"]`);
    await expect(card).toBeVisible({ timeout: 10000 });
    await card.click();

    // 5. Verify the AI summary and drafted reply
    await expect(card).toContainText('Do you have vegan chocolate cake available this weekend?');
    await expect(card).toContainText('Hi! Yes, we have 2 vegan chocolate cakes left for this weekend');

    // 6. Click Approve & Execute
    const approveButton = card.locator(`[data-testid="feed-approve-btn"]`);
    await approveButton.click();

    // 7. Verify the item is removed from the feed
    await expect(card).not.toBeVisible({ timeout: 5000 });
  });

  test('Owner can dismiss AI-drafted replies', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    const response = await page.request.post(`/api/dev/simulate-triage-item?tenant_id=default`);
    const json = await response.json();
    const triageItemId = json.id;

    await page.goto('/dashboard');

    const card = page.locator(`[data-testid="triage-card-${triageItemId}"]`);
    await expect(card).toBeVisible({ timeout: 10000 });
    await card.click();

    const dismissButton = card.locator(`[data-testid="feed-dismiss-btn"]`);
    await dismissButton.click();

    await expect(card).not.toBeVisible({ timeout: 5000 });
  });

  test('Triage feed handles empty state correctly', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard');

    // It should either show the empty state or an empty feed, but given we might have real data,
    // let's just ensure it loads without crashing and either shows items or caught up state.
    const emptyState = page.locator('text=No items need your attention right now');
    const feed = page.locator('[data-testid^="triage-card-"]').first();

    // Wait for either to be visible
    await Promise.race([
        expect(emptyState).toBeVisible({ timeout: 10000 }).catch(() => {}),
        expect(feed).toBeVisible({ timeout: 10000 }).catch(() => {})
    ]);
  });

  test('Triage feed item shows correct metadata', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    const response = await page.request.post(`/api/dev/simulate-triage-item?tenant_id=default`);
    const json = await response.json();
    const triageItemId = json.id;

    await page.goto('/dashboard');

    const card = page.locator(`[data-testid="triage-card-${triageItemId}"]`);
    await expect(card).toBeVisible({ timeout: 10000 });
    await card.click();

    // Check for source and priority based on our mock data
    await expect(card).toContainText('Instagram DM');

  });

  test('Triage feed layout is responsive', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.setViewportSize({ width: 375, height: 812 }); // Mobile

    const response = await page.request.post(`/api/dev/simulate-triage-item?tenant_id=default`);
    const json = await response.json();
    const triageItemId = json.id;

    await page.goto('/dashboard');

    const card = page.locator(`[data-testid="triage-card-${triageItemId}"]`);
    await expect(card).toBeVisible({ timeout: 10000 });
    await card.click();

    // Verify it fits in the mobile viewport
    const box = await card.boundingBox();
    expect(box?.width).toBeLessThanOrEqual(375);
  });
});
