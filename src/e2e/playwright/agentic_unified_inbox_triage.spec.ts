import { test, expect } from '../fixtures';

test.describe('Agentic Unified Inbox Triage (One-Tap Triage)', () => {
  test.use({ viewport: { width: 375, height: 812 } }); // Strictly mobile viewport (375px width)

  test('Owner can perform One-Tap Triage on AI-generated Triage Card from Command Center', async ({ page, loginAs, adminUser }) => {
    // 1. Owner logs into the OHC mobile app (375px width)
    await loginAs(page, adminUser);

    // Seed the triage item to simulate a new multi-channel message
    // Uses the existing simulation endpoint that creates an Instagram DM triage item with a "Draft Reply" including a deposit link
    const response = await page.request.post(`/api/dev/simulate-triage-item?tenant_id=default`);
    expect(response.status()).toBe(200);
    const json = await response.json();
    expect(json.success).toBe(true);
    const triageItemId = json.id;

    // 2. Navigates to the Home Command Center (Dashboard)
    await page.goto('/dashboard');

    // Wait for the unified agent feed to load
    const feed = page.locator('#unified-agent-feed-section');
    await expect(feed).toBeVisible({ timeout: 15000 });

    // 3. Sees an AI-generated Triage Card summarizing a new multi-channel message with a proposed action
    const card = page.locator(`[data-testid="triage-card-${triageItemId}"]`);
    await expect(card).toBeVisible({ timeout: 15000 });

    // Verify it's perfectly functional on a mobile screen without horizontal scrolling
    const boundingBox = await card.boundingBox();
    expect(boundingBox).not.toBeNull();
    if (boundingBox) {
        expect(boundingBox.width).toBeLessThanOrEqual(375);
    }

    // Verify horizontal scrolling doesn't exist on the page
    const hasHorizontalScroll = await page.evaluate(() => {
        return document.documentElement.scrollWidth > window.innerWidth;
    });
    expect(hasHorizontalScroll).toBe(false);

    // Verify the visual cues for Deposit Link exist
    await expect(card).toContainText('Instagram DM Triage');
    await expect(card).toContainText('Customer Message');
    await expect(card).toContainText('vegan chocolate cake');
    await expect(card).toContainText('Payment Link'); // The green badge added to the UI

    // 4. Clicks "Approve & Send" — system fires API and moves item
    const approveBtn = card.locator('[data-testid="feed-approve-btn"]');
    await expect(approveBtn).toBeVisible();
    await expect(approveBtn).toContainText('Approve & Send');

    // Verify touch target is at least 44x44px
    const btnBox = await approveBtn.boundingBox();
    expect(btnBox).not.toBeNull();
    if (btnBox) {
        expect(btnBox.width).toBeGreaterThanOrEqual(44);
        expect(btnBox.height).toBeGreaterThanOrEqual(44);
    }

    await approveBtn.click();

    // Verify optimistic UI update / backend update completes (card should disappear)
    await expect(card).not.toBeVisible({ timeout: 10000 });
  });
});
