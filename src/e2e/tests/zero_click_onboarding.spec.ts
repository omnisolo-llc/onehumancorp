import { test, expect } from '@playwright/test';

test.describe('Zero-Click Onboarding to Agent Feed', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Test 1: Complete flow for a home bakery, verifying Review Product card', async ({ page }) => {
    await page.goto('/zero-click-builder');
    await expect(page.locator('text=What kind of business do you want to build')).toBeVisible();

    const chatInput = page.getByPlaceholder('e.g. I am a home baker in Austin selling custom vegan cakes.');
    await expect(chatInput).toBeVisible();

    await chatInput.fill('I am a home baker in Austin selling custom vegan cakes.');
    await chatInput.press('Enter');

    await expect(page.locator('text=Your business is live!')).toBeVisible({ timeout: 30000 });

    const launchBtn = page.getByRole('button', { name: /Launch My Store/i });
    await expect(launchBtn).toBeVisible();
    await launchBtn.click();

    await page.waitForURL('**/dashboard**', { timeout: 30000 });
    await expect(page.locator('text=Feed')).toBeVisible({ timeout: 10000 });

    // Verify Review Product action card is in the feed
    await expect(page.locator('text=Review Product: Custom Vegan Cake')).toBeVisible({ timeout: 15000 });
  });

  test('Test 2: Complete flow for a mobile dog grooming service', async ({ page }) => {
    await page.goto('/zero-click-builder');
    await expect(page.locator('text=What kind of business do you want to build')).toBeVisible();

    const chatInput = page.getByPlaceholder('e.g. I am a home baker in Austin selling custom vegan cakes.');
    await chatInput.fill('I run a mobile dog grooming service in Austin.');
    await chatInput.press('Enter');

    await expect(page.locator('text=Your business is live!')).toBeVisible({ timeout: 30000 });

    const launchBtn = page.getByRole('button', { name: /Launch My Store/i });
    await expect(launchBtn).toBeVisible();
    await launchBtn.click();

    await page.waitForURL('**/dashboard**', { timeout: 30000 });
    await expect(page.locator('text=Feed')).toBeVisible({ timeout: 10000 });
  });

  test('Test 3: Complete flow for an online music tutor', async ({ page }) => {
    await page.goto('/zero-click-builder');

    // Test the chip click functionality instead of typing
    const chip = page.locator('text=I am an online music tutor');
    await expect(chip).toBeVisible();
    await chip.click();

    const chatInput = page.getByPlaceholder('e.g. I am a home baker in Austin selling custom vegan cakes.');
    await chatInput.press('Enter');

    await expect(page.locator('text=Your business is live!')).toBeVisible({ timeout: 30000 });

    const launchBtn = page.getByRole('button', { name: /Launch My Store/i });
    await expect(launchBtn).toBeVisible();
    await launchBtn.click();

    await page.waitForURL('**/dashboard**', { timeout: 30000 });
    await expect(page.locator('text=Feed')).toBeVisible({ timeout: 10000 });
  });

  test('Test 4: Verify the transition drops the user straight into the dashboard', async ({ page }) => {
    await page.goto('/zero-click-builder');

    const chatInput = page.getByPlaceholder('e.g. I am a home baker in Austin selling custom vegan cakes.');
    await chatInput.fill('I am a freelance photographer.');
    await chatInput.press('Enter');

    await expect(page.locator('text=Your business is live!')).toBeVisible({ timeout: 30000 });

    const launchBtn = page.getByRole('button', { name: /Launch My Store/i });
    await launchBtn.click();

    // Verify it doesn't drop into settings or empty state, but right to dashboard
    await expect(page).toHaveURL(/.*\/dashboard/);
    await expect(page.locator('text=Feed')).toBeVisible({ timeout: 10000 });
  });

  test('Test 5: Verify loading states and elements during processing', async ({ page }) => {
    await page.goto('/zero-click-builder');

    const chatInput = page.getByPlaceholder('e.g. I am a home baker in Austin selling custom vegan cakes.');
    await chatInput.fill('Test loading states.');
    await chatInput.press('Enter');

    // It should immediately show the overlay loading state
    await expect(page.locator('text=Building Your Business...')).toBeVisible();
    await expect(page.locator('text=Provisioning workspace, products, and agents.')).toBeVisible();

    await expect(page.locator('text=Your business is live!')).toBeVisible({ timeout: 30000 });
  });
});
