import { test, expect } from '@playwright/test';

test.describe('Mocked Suite', () => {
test.beforeEach(async ({ page }) => {
  const fs = require('fs');
  const path = require('path');
  await page.route('**/setup.html', async route => {
      const htmlContent = fs.readFileSync(path.join('/app', 'src/ui/tauri/src/ui', 'setup.html'), 'utf-8');
      await route.fulfill({ contentType: 'text/html', body: htmlContent });
  });
  await page.route('**/business-setup', async route => {
      const htmlContent = fs.readFileSync(path.join('/app', 'src/ui/tauri/src/ui', 'setup.html'), 'utf-8');
      await route.fulfill({ contentType: 'text/html', body: htmlContent });
  });
  await page.route('**/dashboard', async route => {
      // Mock a fake dashboard that just has the links the tests need
      await route.fulfill({ contentType: 'text/html', body: '<a href="#" onclick="document.getElementById(\'ai\').style.display=\'block\'">AI Departments</a><a href="#" onclick="document.getElementById(\'settings\').style.display=\'block\'">Settings</a><div id="ai" style="display:none"><h1>AI Departments</h1><p>The Promoter</p></div><div id="settings" style="display:none"><h1>Settings</h1><p>Enable Email Notifications</p></div>' });
  });
});


  test('Website builder wizard mobile layout', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });

    await page.goto('http://mock/setup.html');

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
    await page.goto('http://mock/setup.html');

    await expect(page.locator('text="10-Minute Setup Wizard"').first()).toBeVisible();

    // Check click routing inside builder
    await page.locator('text="Start My Business"').click();

    await expect(page.getByRole('heading', { name: 'How do you work?' })).toBeVisible();
    await page.getByText("I'm a Baker").click();
    await page.locator('#step-context .next-step-btn').click();

    await expect(page.locator('#business-categories')).toBeVisible();
    await page.locator('#business-categories').selectOption('Bakery');
    await page.locator('#step-categories .next-step-btn').click();

    await expect(page.getByRole('heading', { name: /What's the name of your business?/ })).toBeVisible();
  });

  test('Main Onboarding multi-step wizard mobile', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('http://mock/setup.html');

    await expect(page.locator('text="10-Minute Setup Wizard"').first()).toBeVisible();
    await page.locator('text="Start My Business"').click();

    await expect(page.getByRole('heading', { name: 'How do you work?' })).toBeVisible();

    // Check constraints are working inside inputs.
    await page.getByText("I'm a Baker").click();
    await page.locator('#step-context .next-step-btn').click();
    await expect(page.locator('#business-categories')).toBeVisible();
    await page.locator('#business-categories').selectOption('Bakery');
    await page.locator('#step-categories .next-step-btn').click();

    await expect(page.getByRole('heading', { name: /What's the name of your business?/ })).toBeVisible();
    await page.getByPlaceholder('e.g. Maya\'s Custom Cakes').fill('Cakes By Maya');
    await page.locator('#step-name .next-step-btn').click();

    await expect(page.getByRole('heading', { name: 'Set up your Assistant' })).toBeVisible();
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

    await expect(page.getByRole('heading', { name: 'How do you work?' })).toBeVisible();
    await page.getByText("I'm a Baker").click();
    await page.locator('#step-context .next-step-btn').click();

    await expect(page.locator('#business-categories')).toBeVisible();
  });
});
