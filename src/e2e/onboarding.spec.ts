import { test, expect } from '@playwright/test';
import path from 'path';
import fs from 'fs';

test.describe('Onboarding Wizard E2E Flow', () => {

  test.beforeEach(async ({ page }) => {
    await page.route('**/api/onboarding/**', async (route) => {
      await route.fulfill({
        status: 200,
        json: { step: 0, status: 'success', business_name: 'My Awesome E2E Business', business_type: 'Online Store' },
      });
    });

    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR, process.env.TEST_WORKSPACE)
        : process.cwd();

    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    await page.route('**/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.route('**/assistant-setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'assistant-setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });
  });

  // Test 2: Validates the 44px minimum touch target size (via 54px min-height)
  test('Validates 54px touch targets on mobile sizes', async ({ page }) => {
    // Set a mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('http://mock/setup.html');
    const setupScreen = page.locator('#form-container');
    await expect(setupScreen).toBeVisible({ timeout: 30000 });

    const nameInput = page.getByPlaceholder("e.g. Maya's Bakery");
    await expect(nameInput).toBeVisible();
    const box = await nameInput.boundingBox();
    expect(box?.height).toBeGreaterThanOrEqual(54);
  });

  // Test 3: Verifies input disabled states
  test('Next button triggers error when input is empty', async ({ page }) => {
    await page.goto('http://mock/setup.html');
    const setupScreen = page.locator('#form-container');
    await expect(setupScreen).toBeVisible({ timeout: 30000 });

    const nextButton = page.getByRole('button', { name: 'Next' });

    const nameInput = page.getByPlaceholder("e.g. Maya's Bakery");
    await nameInput.fill("");
    await nextButton.click();
    await expect(page.locator('#name-error')).toBeVisible();
  });

  // Test 4: Enter key submits the first step
  test('Enter key submits the input', async ({ page }) => {
    await page.goto('http://mock/setup.html');
    const setupScreen = page.locator('#form-container');
    await expect(setupScreen).toBeVisible({ timeout: 30000 });

    const nameInput = page.getByPlaceholder("e.g. Maya's Bakery");
    await nameInput.fill("ABC");
    await nameInput.press('Enter');

    await expect(page.getByRole('heading', { name: "Set up your Assistant" })).toBeVisible();
  });

});
