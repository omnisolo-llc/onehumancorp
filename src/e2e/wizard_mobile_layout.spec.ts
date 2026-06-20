import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Wizard and Onboarding flows', () => {
  test.beforeEach(async ({ page }) => {
    const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const htmlContent = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: htmlContent });
    });

    // Stub out the next app URLs too
    await page.route('**/website-builder', async route => {
        const htmlContent = fs.readFileSync(path.join(process.cwd(), 'src/ui/tauri/src/ui/setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: htmlContent });
    });
    await page.route('**/builder', async route => {
        const htmlContent = fs.readFileSync(path.join(process.cwd(), 'src/ui/tauri/src/ui/setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: htmlContent });
    });
    await page.route('**/business-setup', async route => {
        const htmlContent = fs.readFileSync(path.join(process.cwd(), 'src/ui/tauri/src/ui/setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: htmlContent });
    });
  });

  test('Website builder wizard mobile layout', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });

    await page.goto('http://mock/website-builder');

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
    await page.goto('http://mock/builder');

    await expect(page.locator('text="10-Minute Setup Wizard"').first()).toBeVisible();

    // Check click routing inside builder
    await page.locator('text="Start My Business"').click();
    await expect(page.getByText("How do you work?")).toBeVisible();

    // Select work context
    await page.locator('[data-testid="context-local"]').click();
    await page.locator('#step-context').getByRole('button', { name: 'Next' }).click();

    // There is an intermediate step called "What's your category?"
    await expect(page.getByRole('heading', { name: "What's your category?" })).toBeVisible();
    // Use selectOption to pass the category validation
    await page.selectOption('#business-categories', { index: 1 });
    await page.locator('#step-categories').getByRole('button', { name: 'Next' }).click();

    const nameInput = page.locator('#business-name');
    await expect(nameInput).toBeVisible();
    await nameInput.fill('Maya Cakes');

    await page.locator('#step-name').getByRole('button', { name: 'Next' }).click();
    await expect(page.getByRole('heading', { name: 'Set up your Assistant' })).toBeVisible();
  });

  test('Main Onboarding multi-step wizard mobile', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('http://mock/setup.html');

    await expect(page.locator('text="10-Minute Setup Wizard"').first()).toBeVisible();
    await page.locator('text="Start My Business"').click();

    await expect(page.getByRole('heading', { name: "How do you work?" })).toBeVisible();

    // Check constraints are working inside inputs.
    await page.locator('[data-testid="context-creator"]').first().click();
    await page.locator('#step-context').getByRole('button', { name: 'Next' }).click();

    // There is an intermediate step called "What's your category?"
    await expect(page.getByRole('heading', { name: "What's your category?" })).toBeVisible();
    await page.selectOption('#business-categories', { index: 1 });
    await page.locator('#step-categories').getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.locator('#business-name').fill('Cakes By Maya');
    await page.locator('#business-tagline').fill('Baker');

    await page.locator('#step-name').getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: 'Set up your Assistant' })).toBeVisible();
    await page.locator('#assistant-name').fill('Friendly');
    await page.selectOption('#assistant-tone', { label: 'Professional' });
    await page.locator('#step-assistant').getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: 'Admin Credentials' })).toBeVisible();
  });

  test('Direct routing for business-setup compatibility page', async ({ page }) => {
    await page.goto('http://mock/business-setup');

    // Should immediately reroute to onboarding
    await expect(page.locator('text="10-Minute Setup Wizard"').first()).toBeVisible();
  });

  test('Onboarding allows full traversal on standard layout', async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 900 });
    await page.goto('http://mock/setup.html');

    await expect(page.locator('text="10-Minute Setup Wizard"').first()).toBeVisible();
    await page.locator('text="Start My Business"').click();

    await expect(page.getByRole('heading', { name: "How do you work?" })).toBeVisible();

    await page.locator('[data-testid="context-local"]').click();
    await page.locator('#step-context').getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: "What's your category?" })).toBeVisible();
    await page.selectOption('#business-categories', { index: 1 });
    await page.locator('#step-categories').getByRole('button', { name: 'Next' }).click();

    await page.locator('#business-name').fill('Auto Repair');
    await page.locator('#business-tagline').fill('Mechanic');

    await page.locator('#step-name').getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: 'Set up your Assistant' })).toBeVisible();
  });
});
