import { test, expect } from './fixtures';

test.describe('Onboarding State Persistence', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should persist onboarding state across reloads', async ({ page }) => {
    // 0. Start from UI Login
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('maya@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    // Wait for Dashboard
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    // 1. Start onboarding
    await page.goto('/onboarding');
    await expect(page.getByRole('heading', { name: "What do you do?" })).toBeVisible();

    // 2. Enter data in step 1 and move to step 2
    await page.getByPlaceholder("e.g. Sell cakes, plumbing").fill("Baking");
    await page.getByRole('button', { name: /Next/i }).click();

    // 3. Step 2 visible
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Persistence Bakery");

    // Give it a moment to save draft (debounced)
    await page.waitForTimeout(1000);

    // 4. Reload the page (simulating coming back later or another device)
    // Clear local storage to ensure it's not just browser state
    await page.evaluate(() => localStorage.clear());
    await page.reload();

    // 5. Verify state is restored from backend
    // Since we cleared localStorage, if it's restored, it MUST come from the backend.
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await expect(page.getByPlaceholder("e.g. Maya's Cakes")).toHaveValue("Persistence Bakery");
  });
});
