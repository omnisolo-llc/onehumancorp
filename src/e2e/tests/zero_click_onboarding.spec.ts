import { test, expect } from '@playwright/test';

test.describe('Zero-Click Onboarding to Agent Feed', () => {

  test.beforeEach(async ({ page }) => {
    const fs = require('fs');
    const path = require('path');
    const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });
    // mock the tauri backend
    await page.addInitScript(() => {
        (window as any).__TAURI__ = {
            core: {
                invoke: async (cmd, args) => {
                    if (cmd === 'start_onboarding') {
                        return { success: true };
                    }
                    if (cmd === 'process_intake') {
                        if (args.input && args.input.includes("fail network request")) {
                            throw new Error("Network request failed");
                        }
                        if (!args.input || args.input.trim() === '') {
                             throw new Error("Empty input");
                        }
                        return {
                            business_name: "Mock Instant Business",
                            business_type: "Local Service",
                            categories: ["Mock Category"],
                            location: "San Francisco",
                            target_audience: "Anyone",
                            initial_products: [ { name: "Mock Product", price: "10.00" } ]
                        };
                    }
                    return null;
                }
            }
        };
    });
  });

  test('User completes instant onboarding and sees success screen', async ({ page }) => {
    // Navigate to the setup route
    await page.goto('http://mock/setup.html');

    // Make sure we're on a mobile viewport
    await page.setViewportSize({ width: 375, height: 812 });

    // The button should be disabled initially
    const generateBtn = page.locator('#generate-storefront-btn');
    await expect(generateBtn).toBeDisabled();

    // Fill the instant bio
    const bioInput = page.locator('#instant-bio');
    await expect(bioInput).toBeVisible();
    await bioInput.fill('I run a mobile dog grooming service in Austin');

    // The button should now be enabled
    await expect(generateBtn).toBeEnabled();

    // Click the generate button
    await generateBtn.click();

    // It should transition to the loading state
    const successHeading = page.getByRole('heading', { name: /You're Live!/ });
    await expect(successHeading).toBeVisible({ timeout: 10000 });

    // Check horizontal scroll by verifying document width equals window innerWidth
    const hasHorizontalScroll = await page.evaluate(() => {
        return document.documentElement.scrollWidth > window.innerWidth;
    });
    expect(hasHorizontalScroll).toBeFalsy();
  });

  test('Instant Build prevents empty submissions', async ({ page }) => {
    await page.goto('http://mock/setup.html');

    const generateBtn = page.locator('#generate-storefront-btn');
    const bioInput = page.locator('#instant-bio');

    // Button disabled by default
    await expect(generateBtn).toBeDisabled();

    // Type and then clear
    await bioInput.fill('Some text');
    await expect(generateBtn).toBeEnabled();

    await bioInput.fill('');
    await expect(generateBtn).toBeDisabled();
  });

  test('Instant Build handles API errors gracefully', async ({ page }) => {
    await page.goto('http://mock/setup.html');

    const generateBtn = page.locator('#generate-storefront-btn');
    const bioInput = page.locator('#instant-bio');
    const errorMsg = page.locator('#instant-error');

    await expect(errorMsg).toBeHidden();

    // Fill a prompt that triggers our mock error
    await bioInput.fill('fail network request');
    await generateBtn.click();

    // Should display the error
    await expect(errorMsg).toBeVisible();
    await expect(errorMsg).toHaveText('Network request failed');

    // Button should be re-enabled
    await expect(generateBtn).toBeEnabled();
  });
});
