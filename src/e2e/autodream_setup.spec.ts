import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test.describe('10-Minute Setup Wizard for Storefront Generation (Autodream) CUJ', () => {

  test.beforeEach(async ({ page }) => {
    // Intercept standard setup.html load to serve from filesystem for tests
    const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
      const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
      await route.fulfill({ contentType: 'text/html', body: content });
    });

    // Mock tooltips call
    await page.route('**/api/tooltips', async route => {
      await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });

    // Mock the state endpoint which the frontend hits
    await page.route('**/api/onboarding/state', async route => {
       await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });

    // Set a known viewport for mobile tests (375px)
    await page.setViewportSize({ width: 375, height: 812 });
  });

  test('Persona: Maya (Home Baker) completes the 10-Minute Storefront Setup', async ({ page }) => {
    // Navigate directly to the setup HTML page
    await page.goto('http://mock/setup.html');

    // Verify Initial Welcome Screen
    await expect(page.getByRole('heading', { name: '10-Minute Setup Wizard' })).toBeVisible();

    // 1. Click "10-Minute Storefront Setup"
    await page.getByRole('button', { name: '10-Minute Storefront Setup' }).click();

    // 2. Verify we are in the Business Type step
    await expect(page.getByRole('heading', { name: 'What type of business do you run?' })).toBeVisible();
    await page.getByText('Physical Products').click();
    await page.getByTestId('ad-next-type').click();

    // 3. Verify we are in the Business Name & Vibe step
    await expect(page.getByRole('heading', { name: 'What is your business name and vibe?' })).toBeVisible();
    const nameInput = page.getByTestId('ad-business-name');
    await expect(nameInput).toBeVisible();

    // Test validation (try clicking next while empty)
    await page.getByTestId('ad-next-name').click();
    await expect(page.getByText('Please enter your business name.')).toBeVisible();

    await nameInput.fill("Maya's Custom Vegan Cakes, Friendly & Modern");
    await page.getByTestId('ad-next-name').click();

    // 4. Verify we are in the Core Offering step
    await expect(page.getByRole('heading', { name: 'What do you sell mostly?' })).toBeVisible();
    const offerInput = page.getByTestId('ad-core-offer');
    await expect(offerInput).toBeVisible();
    await offerInput.fill("Custom Vegan Cakes for Events");

    // 5. Generate the storefront
    await page.getByTestId('ad-next-offer').click();

    // 6. Verify Loading Generation Screen
    await expect(page.getByRole('heading', { name: 'Generating Storefront...' })).toBeVisible();
    await expect(page.getByText('Drafting layout...')).toBeVisible();

    // Mock timeouts finish after 3 seconds, so wait for it
    await page.waitForTimeout(3000);

    // 7. Verify Preview / Approval Screen
    await expect(page.getByRole('heading', { name: 'Here is your new storefront' })).toBeVisible({ timeout: 5000 });

    // Assert that the preview shows the entered name and offering
    await expect(page.locator('#ad-preview-title')).toHaveText("Maya's Custom Vegan Cakes, Friendly & Modern");
    await expect(page.locator('#ad-preview-subtitle')).toHaveText("Custom Vegan Cakes for Events");

    // Check minimum touch target constraints
    const approveBtn = page.getByTestId('ad-approve-btn');
    const approveBtnBox = await approveBtn.boundingBox();
    expect(Math.round(approveBtnBox?.height || 0)).toBeGreaterThanOrEqual(44);

    // 8. Approve the mock storefront
    // Set up mock route for success.html redirection
    await page.route('**/success.html', async route => {
      await route.fulfill({ status: 200, contentType: 'text/html', body: '<h1>Success!</h1>' });
    });

    await approveBtn.click();

    // 9. Verify it navigated to success.html
    await page.waitForURL('**/success.html', { timeout: 10000 });
  });
});
