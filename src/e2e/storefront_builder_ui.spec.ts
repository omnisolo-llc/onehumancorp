import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Storefront Builder UI Premium Standards', () => {
  test('should verify premium UI tokens on the builder initial state', async ({ page }) => {
    // Navigate to the storefront builder
    await page.goto('/storefront-builder');

    // Wait for the main setup screen to load
    const setupScreen = page.locator('#setup-screen');
    await expect(setupScreen).toBeVisible({ timeout: 10000 });

    // Verify main background texture and color classes
    const bodyContainer = setupScreen.locator('..');
    await expect(bodyContainer).toHaveClass(/bg-\[#F5F5F7\]/);
    await expect(bodyContainer).toHaveClass(/bg-\[url.*\]/);

    // Verify typography
    const title = page.locator('h1', { hasText: 'Welcome to OHC Smart Builder' });
    await expect(title).toBeVisible();
    await expect(title).toHaveClass(/font-outfit/);
    await expect(title).toHaveClass(/text-\[#1D1D1F\]/);

    // Verify text area styling (mac glass container + specific border/focus ring classes)
    const bioInput = page.locator('#bio-input');
    await expect(bioInput).toBeVisible();
    await expect(bioInput).toHaveClass(/mac-glass-container/);
    await expect(bioInput).toHaveClass(/focus:ring-\[#0066FF\]/);
    await expect(bioInput).toHaveClass(/rounded-\[8px\]/);

    // Verify the Generate button starts out greyed
    const generateBtn = page.locator('#generate-btn');
    await expect(generateBtn).toHaveClass(/text-gray-400/);
    await expect(generateBtn).toBeDisabled();

    // Type enough text to activate the button
    await bioInput.fill("I sell custom ceramic coffee mugs and bowls");

    // Verify the button becomes active and uses the premium uniFi blue
    await expect(generateBtn).toBeEnabled();
    await expect(generateBtn).toHaveClass(/bg-\[#0066FF\]/);
  });

  test('should verify premium UI tokens on the generating state', async ({ page }) => {
    await page.goto('/storefront-builder');
    await page.locator('#bio-input').fill("I sell custom ceramic coffee mugs and bowls");
    await page.locator('#generate-btn').click();

    // Verify generating spinner styling
    const spinner = page.locator('.animate-spin');
    await expect(spinner).toBeVisible();
    await expect(spinner).toHaveClass(/border-\[#0066FF\]/);
  });

  test('should verify premium UI tokens on the live success state', async ({ page }) => {
    // We mock the fetch request for the generate call to jump right to the live state
    await page.route('**/api/v1/builder/generate', async route => {
      const json = { blocks: [{ type: 'Hero', props: { headline: 'Ceramics' } }] };
      await route.fulfill({ json });
    });

    await page.route('**/api/v1/builder/publish_draft', async route => {
      const json = { domain: 'ceramics-store' };
      await route.fulfill({ json });
    });

    await page.goto('/storefront-builder');
    await page.locator('#bio-input').fill("I sell custom ceramic coffee mugs and bowls");
    await page.locator('#generate-btn').click();

    // Wait for the next button to appear (1-tap launch) and click it
    const launchBtn = page.locator('#launch-btn');
    await expect(launchBtn).toBeVisible({ timeout: 10000 });
    await launchBtn.click();

    // Verify You're Live screen
    const liveTitle = page.locator('h1', { hasText: "You're Live!" });
    await expect(liveTitle).toBeVisible();
    await expect(liveTitle).toHaveClass(/text-\[#1D1D1F\]/);

    // Verify copy button token
    const copyBtn = page.locator('button', { hasText: 'Copy' });
    await expect(copyBtn).toHaveClass(/text-\[#0066FF\]/);

    // Verify bottom action button token
    const dashboardBtn = page.locator('button', { hasText: 'Go to Dashboard' });
    await expect(dashboardBtn).toHaveClass(/mac-glass-container/);
    await expect(dashboardBtn).toHaveClass(/rounded-\[8px\]/);
  });

  test('should verify bottom bar uses mac-glass-container in preview mode', async ({ page }) => {
    await page.route('**/api/v1/builder/generate', async route => {
      const json = { blocks: [{ type: 'Hero', props: { headline: 'Ceramics' } }] };
      await route.fulfill({ json });
    });

    await page.goto('/storefront-builder');
    await page.locator('#bio-input').fill("I sell custom ceramic coffee mugs and bowls");
    await page.locator('#generate-btn').click();

    // Verify the bottom sticky bar has the glass container class and correct border radii
    const bottomBar = page.locator('.absolute.bottom-0');
    await expect(bottomBar).toBeVisible();
    await expect(bottomBar).toHaveClass(/mac-glass-container/);
    await expect(bottomBar).toHaveClass(/rounded-b-\[16px\]/);

    // Verify Launch button
    const launchBtn = page.locator('#launch-btn');
    await expect(launchBtn).toHaveClass(/bg-\[#0066FF\]/);
    await expect(launchBtn).toHaveClass(/rounded-\[8px\]/);
  });

  test('should complete the entire storefront builder flow manually', async ({ page }) => {
    // E2E smoke test verifying it all pieces together
    await page.goto('/storefront-builder');

    const bioInput = page.locator('#bio-input');
    await expect(bioInput).toBeVisible();
    await bioInput.fill("I sell artisan baked breads and sourdoughs in Chicago");

    const generateBtn = page.locator('#generate-btn');
    await expect(generateBtn).toBeEnabled();
    await generateBtn.click();

    const launchBtn = page.locator('#launch-btn');
    await expect(launchBtn).toBeVisible({ timeout: 15000 }); // Wait for generate to finish
    await launchBtn.click();

    const liveTitle = page.locator('h1', { hasText: "You're Live!" });
    await expect(liveTitle).toBeVisible({ timeout: 15000 }); // Wait for publish to finish

    const copyBtn = page.locator('button', { hasText: 'Copy' });
    await expect(copyBtn).toBeVisible();
  });
});
