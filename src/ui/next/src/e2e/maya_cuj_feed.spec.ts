import { expect, test } from '@playwright/test';

test.describe('Maya CUJ: Unified Agent Feed - Instagram DM Reply', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Maya should see an AI-drafted Instagram DM and approve it with 1-tap', async ({ page }) => {
    test.setTimeout(180000);

    // 1. Login and navigate to dashboard
    await page.goto('/dashboard');

    // 2. Wait for the feed to load
    // We assume the feed is populated or we simulate a draft
    await page.getByTestId('simulate-ambassador-btn').click().catch(() => {});

    // 3. Find the Ambassador/Instagram DM card
    const card = page.locator('[data-testid="agent-feed-card"]').filter({ hasText: 'CUSTOMER MESSAGE' }).first();
    await expect(card).toBeVisible({ timeout: 30000 });

    // 4. Verify card content
    await expect(card).toContainText('Agent Draft');

    // 5. Approve & Send
    const approveBtn = card.getByTestId('approve-ambassador-reply');
    await expect(approveBtn).toBeVisible();

    // Check touch target size (>= 44px)
    const box = await approveBtn.boundingBox();
    if (box) {
      expect(box.width).toBeGreaterThanOrEqual(44);
      expect(box.height).toBeGreaterThanOrEqual(44);
    }

    await approveBtn.click();

    // 6. Verify transition state (green border, scale down)
    await expect(card).toHaveClass(/border-green-500/);
    await expect(card).toHaveClass(/scale-95/);

    // 7. Verify card disappears
    await expect(card).not.toBeVisible({ timeout: 5000 });
  });

  test('Maya should be able to edit an AI-drafted reply before sending', async ({ page }) => {
    test.setTimeout(180000);
    await page.goto('/dashboard');

    const card = page.locator('[data-testid="agent-feed-card"]').filter({ hasText: 'CUSTOMER MESSAGE' }).first();
    await expect(card).toBeVisible({ timeout: 30000 });

    // Click Edit
    await card.getByTestId('edit-ambassador-reply').click();

    // Verify textarea
    const textarea = card.getByTestId('edit-ambassador-reply-textarea');
    await expect(textarea).toBeVisible();
    await textarea.fill('Maya manually edited this reply.');

    // Save & Send
    await card.getByTestId('save-send-ambassador-reply').click();

    // Verify transition and disappearance
    await expect(card).toHaveClass(/border-green-500/);
    await expect(card).not.toBeVisible({ timeout: 5000 });
  });
});
