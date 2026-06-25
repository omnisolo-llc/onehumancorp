<<<<<<< HEAD
import { test, expect } from '../fixtures';
=======
import { test, expect } from '@playwright/test';
>>>>>>> 5aad3344 (Update prices to /9/9 per requirements)

test.describe('Autonomous AI Work Triage and Daily Work Generation', () => {
  test.use({ viewport: { width: 375, height: 812 } });

<<<<<<< HEAD
  test('injects a raw signal, surfaces in the UI, and owner approves it', async ({ page, request, loginAs, adminUser }) => {
    // 1. Log in to the application
    await loginAs(page, adminUser);

=======
  test('injects a raw signal, surfaces in the UI, and owner approves it', async ({ page, request }) => {
>>>>>>> 5aad3344 (Update prices to /9/9 per requirements)
    // 1. Inject raw signal to simulate Triage Engine processing
    const tenantId = 'default'; // In a real E2E, this aligns with the configured test tenant
    const payload = {
      source: "Instagram DM",
      payload: { text: "Do you do vegan cakes for Saturday?" }
    };

    const res = await request.post(`/api/dev/simulate-triage-item?tenant_id=${tenantId}`, {
      data: payload
    });
    expect(res.ok()).toBeTruthy();
    const result = await res.json();
    expect(result.success).toBe(true);
    const workItemId = result.id;

    // 2. Navigate to Daily Work Feed at mobile resolution
    await page.goto('/dashboard/daily-work');

    // Wait for the feed to load
    await expect(page.locator('text=Loading your work feed...')).not.toBeVisible({ timeout: 10000 });

    // 3. Verify the surfaced actionable card
<<<<<<< HEAD
    const card = page.locator(`[data-testid="triage-card-${workItemId}"]`);
    await expect(card).toBeVisible({ timeout: 10000 });

    // Find the approve button
    const approveButton = card.locator(`[data-testid="triage-approve-${workItemId}"]`);
=======
    const card = page.locator('[data-testid="daily-work-card"]').first();
    await expect(card).toBeVisible();

    // It should have the simulated context "Draft Reply" based on our mock
    await expect(card.locator('text=Draft Reply')).toBeVisible();

    // Find the approve button
    const approveButton = card.locator(`[data-testid="approve-${workItemId}"]`);
>>>>>>> 5aad3344 (Update prices to /9/9 per requirements)
    await expect(approveButton).toBeVisible();

    // Check touch target for mobile
    const boundingBox = await approveButton.boundingBox();
    expect(boundingBox?.width).toBeGreaterThanOrEqual(44);
    expect(boundingBox?.height).toBeGreaterThanOrEqual(44);

    // 4. Click Approve and verify state change (optimistic update should remove the card)
    await approveButton.click();

    // The card should disappear
<<<<<<< HEAD
    await expect(card).not.toBeVisible({ timeout: 5000 });
=======
    await expect(card).not.toBeVisible();
>>>>>>> 5aad3344 (Update prices to /9/9 per requirements)
  });
});
