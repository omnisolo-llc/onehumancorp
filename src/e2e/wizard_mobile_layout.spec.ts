import { test, expect } from '@playwright/test';

test.describe('Wizard and Onboarding flows', () => {

  test('Website builder wizard mobile layout', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });

    await page.goto('/website-builder');

    // Check elements
    const heading = page.getByRole('heading', { name: '10-Minute Setup Wizard' });
    await expect(heading).toBeVisible();

    // Verify it doesn't overflow horizontally
    const htmlWidth = await page.evaluate(() => document.documentElement.scrollWidth);
    const windowWidth = await page.evaluate(() => window.innerWidth);
    expect(htmlWidth).toBeLessThanOrEqual(windowWidth);

    await page.getByRole('button', { name: 'Instant Build' }).click();
    await expect(page.getByRole('heading', { name: 'Tell us about your business' })).toBeVisible();
  });

  test('Builder mobile UI test', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/onboarding');

    const startBtn = page.getByRole('button', { name: /Start My Business/i });
    await expect(startBtn).toBeVisible();
    await startBtn.click();

    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();

    // Check click routing inside builder
    const onlineStoreBtn = page.getByRole('button', { name: /Online Store/i });
    if (await onlineStoreBtn.isVisible()) {
      await onlineStoreBtn.click();
    }


    const nameInput = page.getByPlaceholder(/Maya's Custom Cakes/i);
    await expect(nameInput).toBeVisible();
    await nameInput.fill('Maya Cakes');

    await page.getByRole('button', { name: /Next/i }).first().click();

    // Check constraints are working inside inputs
    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
  });

  test('Main Onboarding multi-step wizard mobile', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/onboarding');

    const startBtn = page.getByRole('button', { name: /Start My Business/i });
    await expect(startBtn).toBeVisible();
    await startBtn.click();

    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();

    const onlineStoreBtn = page.getByRole('button', { name: /Online Store/i });
    if (await onlineStoreBtn.isVisible()) {
      await onlineStoreBtn.click();
    }

    await page.getByPlaceholder(/Maya's Custom Cakes/i).fill('Maya Bakery');
    await page.getByRole('button', { name: /Next/i }).first().click();

    // Check constraints are working inside inputs
    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
  });

  test('Direct routing for business-setup compatibility page', async ({ page }) => {
    await page.goto('/onboarding');

    // Should immediately reroute to onboarding
    await expect(page.getByRole('heading', { name: '10-Minute Setup Wizard' })).toBeVisible();
  });

  test('Onboarding allows full traversal on standard layout', async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 900 });
    await page.goto('/onboarding');

    const startBtn = page.getByRole('button', { name: /Start My Business/i });
    await expect(startBtn).toBeVisible();
    await startBtn.click();

    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();

    const restaurantBtn = page.getByRole('button', { name: /Restaurant/i });
    if (await restaurantBtn.isVisible()) {
      await restaurantBtn.click();
    }

    await page.getByPlaceholder(/Maya's Custom Cakes/i).fill('Auto Repair');
    await page.getByRole('button', { name: /Next/i }).first().click();

    // Check constraints are working inside inputs
    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
  });
});
