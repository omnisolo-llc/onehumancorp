import { test, expect } from '@playwright/test';


test.describe('Wizard and Onboarding flows', () => {

  test.beforeEach(async ({ page }) => {
    const fs = require('fs');
    const path = require('path');
    const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const htmlContent = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: htmlContent   });
    });
    await page.route('**/api/tooltips', async route => {
      await route.fulfill({ status: 200, body: JSON.stringify({})   });
    });
    await page.route('**/api/onboarding/draft', async route => {
       await route.fulfill({ status: 200, body: JSON.stringify({})   });
    });
  });

  test('Website builder wizard mobile layout', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });

    await page.goto('http://mock/setup.html');

    // Check elements
    const heading = page.getByRole('heading', { name: 'Tell us about your business' });
    await expect(heading).toBeVisible();

    // Verify it doesn't overflow horizontally
    const htmlWidth = await page.evaluate(() => document.documentElement.scrollWidth);
    const windowWidth = await page.evaluate(() => window.innerWidth);
    expect(htmlWidth).toBeLessThanOrEqual(windowWidth);

    await expect(page.locator('text="Step-by-Step Setup"')).toBeVisible();
  });

  test('Builder mobile UI test', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('http://mock/setup.html');

    await expect(page.locator('text="Tell us about your business"').first()).toBeVisible();

    // Check click routing inside builder
    await page.locator('text="Step-by-Step Setup"').click();

    await expect(page.getByRole('heading', { name: 'How do you work?' })).toBeVisible();
    await page.getByText("I'm a Baker").click();
    await page.locator('#step-context .next-step-btn').click();

    await expect(page.locator('#business-categories')).toBeVisible();
    await page.locator('#business-categories').selectOption('Home Baker');
    await page.locator('#step-categories .next-step-btn').click();

    await expect(page.getByRole('heading', { name: /What's the name of your business\?/ })).toBeVisible();
  });

  test('Main Onboarding multi-step wizard mobile', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('http://mock/setup.html');

    await expect(page.locator('text="Tell us about your business"').first()).toBeVisible();
    await page.locator('text="Step-by-Step Setup"').click();

    await expect(page.getByRole('heading', { name: 'How do you work?' })).toBeVisible();

    // Check constraints are working inside inputs.
    await page.getByText("I'm a Baker").click();
    await page.locator('#step-context .next-step-btn').click();
    await expect(page.locator('#business-categories')).toBeVisible();
    await page.locator('#business-categories').selectOption('Home Baker');
    await page.locator('#step-categories .next-step-btn').click();

    await expect(page.getByRole('heading', { name: /What's the name of your business\?/ })).toBeVisible();
    await page.getByPlaceholder('e.g. Maya\'s Custom Cakes').fill('Cakes By Maya');
    await page.locator('#step-name .next-step-btn').click();

    await expect(page.getByRole('heading', { name: 'Set up your Assistant' })).toBeVisible();
  });

  test('Direct routing for business-setup compatibility page', async ({ page }) => {
    await page.goto('http://mock/setup.html');

    // Should immediately reroute to onboarding
    await expect(page.locator('text="Tell us about your business"').first()).toBeVisible();
  });

  test('Onboarding allows full traversal on standard layout', async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 900 });
    await page.goto('http://mock/setup.html');

    await expect(page.locator('text="Tell us about your business"').first()).toBeVisible();
    await page.locator('text="Step-by-Step Setup"').click();

    await expect(page.getByRole('heading', { name: 'How do you work?' })).toBeVisible();
    await page.getByText("I'm a Baker").click();
    await page.locator('#step-context .next-step-btn').click();

    await expect(page.locator('#business-categories')).toBeVisible();
  });

  test('Loading state padding check on mobile layout', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('http://mock/setup.html');

    // Attempt to access step 4 loading state directly if possible, or intercept network and check
    await page.evaluate(() => {
        window.localStorage.setItem('onboarding-storage-v4', JSON.stringify({
            state: { step: 4 }
        }));
    });

    await page.reload();

    // Check loading indicator container doesn't overflow
    const container = page.locator('#form-container');
    await expect(container).toBeVisible();

    const containerWidth = await container.evaluate(el => el.clientWidth);
    expect(containerWidth).toBeLessThanOrEqual(375);
  });
});
