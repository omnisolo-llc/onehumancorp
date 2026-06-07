import { test, expect } from '@playwright/test';

test.describe('Quick Creation FAB', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to dashboard directly (using bypass or standard auth)
    await page.goto('/dashboard');
    // Ensure we're logged in (assume auto-login or seed is used)
  });

  test('should open FAB modal, take intent, and prefill offering creation', async ({ page }) => {
    // 1. Check FAB is visible
    const fab = page.locator('[data-testid="fab-create-offering"]');
    await expect(fab).toBeVisible();

    // 2. Click FAB
    await fab.click();

    // 3. Modal should open
    const intentInput = page.locator('[data-testid="offering-intent-input"]');
    await expect(intentInput).toBeVisible();

    // 4. Enter intent
    const intentText = '1 hour beginner guitar lesson';
    await intentInput.fill(intentText);

    // 5. Submit
    const submitBtn = page.locator('[data-testid="submit-intent-btn"]');
    await submitBtn.click();

    // 6. Should navigate to /products/new with intent
    await expect(page).toHaveURL(/\/products\/new\?intent=.*guitar.*lesson/i);

    // 7. Check if page auto-filled (or is loading and then fills)
    // We'll mock or wait for the title input to become the generated value
    const titleInput = page.locator('input[value*="Guitar"]');
    await expect(titleInput).toBeVisible({ timeout: 10000 });
  });
});
