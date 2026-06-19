import { test, expect } from '@playwright/test';

test.describe('Business Setup Wizard UI Improvements', () => {
  test.beforeEach(async ({ page }) => {
    await page.addInitScript(() => {
      localStorage.clear();
    });
  });

  test('password toggle, enter key advance, and auto-focus', async ({ page }) => {
    await page.goto('/setup.html');
    await page.waitForLoadState('networkidle');

    // Start My Business (manual flow)
    await page.locator('button[data-testid="next-step-btn"][data-next="step-context"]').click();
    await expect(page.locator('#step-context')).toHaveClass(/active/);

    // Step 1: Context
    await page.locator('div.persona-chip').first().click(); // sets Storefront and Baker
    await page.locator('#step-context button[data-testid="next-step-btn"]').click();
    await expect(page.locator('#step-categories')).toHaveClass(/active/);

    // Step 2: Categories
    await page.locator('#business-categories').selectOption({ index: 1 });
    await page.locator('#step-categories button[data-testid="next-step-btn"]').click();
    await expect(page.locator('#step-name')).toHaveClass(/active/);

    // Step 3: Name
    await page.locator('#business-name').fill('My Awesome Bakery');
    await page.locator('#step-name button[data-testid="next-step-btn"]').click();
    await expect(page.locator('#step-assistant')).toHaveClass(/active/);

    // Step 4: Assistant
    await page.locator('#assistant-name').fill('BakeBot');
    await page.locator('#assistant-tone').selectOption({ index: 1 });
    await page.locator('#step-assistant button[data-testid="next-step-btn"]').click();
    await expect(page.locator('#step-admin')).toHaveClass(/active/);

    // Step 5: Admin Setup - Here we test the improvements
    const emailInput = page.locator('#admin-email');
    const passwordInput = page.locator('#admin-password');
    const toggleBtn = page.locator('#toggle-password-visibility');

    // Auto-focus check
    await expect(emailInput).toBeFocused({ timeout: 10000 });

    await emailInput.fill('admin@example.com');
    await passwordInput.fill('password123');

    // Toggle Password Visibility check
    await expect(passwordInput).toHaveAttribute('type', 'password');
    await toggleBtn.click();
    await expect(passwordInput).toHaveAttribute('type', 'text');
    await toggleBtn.click();
    await expect(passwordInput).toHaveAttribute('type', 'password');

    // Enter Key Advance check
    await passwordInput.focus();
    await page.keyboard.press('Enter', { delay: 100 });
    await page.waitForTimeout(500);

    // Should advance to step-offer
    await expect(page.locator('#step-offer')).toHaveClass(/active/);

    // Auto-focus on step-offer
    const offerInput = page.locator('#first-offer');
    await expect(offerInput).toBeFocused({ timeout: 10000 });
  });
});
