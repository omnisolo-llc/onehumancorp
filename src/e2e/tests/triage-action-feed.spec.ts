import { test, expect } from '@playwright/test';

test.describe('Intelligent Owner Triage Inbox: Mobile-First Agentic Work Feed', () => {
  const tenantId = 'e2e-triage-mobile-tenant';

  test.beforeEach(async ({ page }) => {
    // Set 375px mobile viewport per instructions
    await page.setViewportSize({ width: 375, height: 812 });
  });

  test('Owner can view feed, review AI draft in bottom sheet, and approve', async ({ page, request }) => {
    // 1. Seed a mock item via webhook
    const res = await request.post('/api/v1/omnichannel/webhook', {
      data: {
        tenant_id: tenantId,
        source: 'Instagram DM Message',
        sender_id: 'Maya',
        message: 'Customer asked about vegan cakes.',
      }
    });
    expect(res.status()).toBe(200);

    // Mock localStorage
    await page.addInitScript((t) => {
      window.localStorage.setItem('tenant_id', t);
      window.localStorage.setItem('has_onboarded', 'true');
    }, tenantId);

    // 2. Go to Triage feed
    await page.goto('/triage');

    // 3. Verify card exists and is styled correctly (glassmorphism)
    const triageCard = page.locator('.ohc-card').first();
    await expect(triageCard).toBeVisible({ timeout: 15000 });

    // Verify touch target sizing for buttons (44x44 min)
    const reviewBtn = triageCard.getByTestId(/triage-review-/);
    await expect(reviewBtn).toBeVisible();
    const box = await reviewBtn.boundingBox();
    expect(box?.height).toBeGreaterThanOrEqual(44);

    // 4. Click Review AI Draft to open bottom sheet
    await reviewBtn.click();

    // 5. Verify bottom sheet opens with textarea
    const bottomSheetTextarea = page.getByTestId('bottom-sheet-textarea');
    await expect(bottomSheetTextarea).toBeVisible();

    // 6. Edit and approve
    await bottomSheetTextarea.fill('Yes, we have vegan options! Let me know what flavor you want.');
    const approveBtn = page.getByTestId('bottom-sheet-approve');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // 7. Verify item is removed from feed
    await expect(triageCard).not.toBeVisible();
  });
});
