import { test, expect } from '@playwright/test';

test.describe('Work Triage Mobile Feed', () => {
  test.use({
    viewport: { width: 375, height: 667 }, // Mobile viewport
  });

  test('Should render correctly and handle approval flow', async ({ page }) => {
    const tenantId = 'e2e-tenant';
    await page.goto(`/triage?tenant_id=${tenantId}`);

    // Wait for the feed to load
    await expect(page.locator('[data-testid="triage-card-triage-test-1"]')).toBeVisible();

    // Verify touch targets and layout
    const reviewBtn = page.locator('[data-testid="triage-review-btn-triage-test-1"]');
    await expect(reviewBtn).toBeVisible();
    const btnBox = await reviewBtn.boundingBox();
    expect(btnBox?.width).toBeGreaterThanOrEqual(44);
    expect(btnBox?.height).toBeGreaterThanOrEqual(44);

    // Verify mobile layout constraint (no horizontal scroll)
    const bodyBox = await page.locator('body').boundingBox();
    expect(bodyBox?.width).toBeLessThanOrEqual(375);

    // Simulate clicking Review Draft
    await reviewBtn.click();
    await expect(page.locator('[data-testid="triage-edit-textarea-triage-test-1"]')).toBeVisible();

    // Simulate Saving / Approving
    const saveBtn = page.locator('[data-testid="triage-save-btn-triage-test-1"]');
    await saveBtn.click();

    // The item should disappear from the feed after approval
    await expect(page.locator('[data-testid="triage-card-triage-test-1"]')).toBeHidden();
  });
});
