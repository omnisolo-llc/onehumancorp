import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Flow', () => {


  test.beforeEach(async ({ page }) => {
    const fs = require('fs');
    const path = require('path');
    const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });
    await page.goto('http://mock/setup.html');
  });

  test('successfully completes the wizard with drafting and instant image url', async ({ page }) => {
    await expect(page.locator('body')).toContainText('10-Minute Setup Wizard');

    // Sometimes there might be a loading transition, wait for the Instant Build button to be ready
    await page.waitForTimeout(2000);

    // There are multiple ways to click Instant Build, we added ID "instant-build-btn-text"
    await page.getByText('Instant Build').click();

    const bioInput = page.locator('#instant-bio');
    await expect(bioInput).toBeVisible();

    await bioInput.fill('My E2E Bakery');

    const imageUrlInput = page.locator('#instant-image-url');
    await expect(imageUrlInput).toBeVisible();
    await imageUrlInput.fill('https://example.com/bakery.png');

    await page.locator('#generate-storefront-btn').click();

    await expect(page.locator('#loading-title')).toContainText('Building Your Business...', { timeout: 15000 });
  });

  test('successfully navigates through the wizard steps', async ({ page }) => {
    await expect(page.locator('body')).toContainText('10-Minute Setup Wizard');

    await page.getByTestId('next-step-btn').click();
    await expect(page.locator('body')).toContainText('work context');

    // Make a choice for context
    await page.locator('input[value="field"]').click({ force: true });
    await page.getByTestId('next-step-btn').nth(1).click();

    await expect(page.locator('body')).toContainText('category');
    await page.locator('#business-categories').selectOption('Other');
    await page.getByTestId('next-step-btn').nth(2).click();

    await expect(page.locator('body')).toContainText('name of your business');
  });

  test('prevents progression if categories input is empty', async ({ page }) => {
    await expect(page.locator('body')).toContainText('10-Minute Setup Wizard');

    await page.getByTestId('next-step-btn').click();
    await page.locator('input[value="field"]').click({ force: true });
    await page.getByTestId('next-step-btn').nth(1).click();

    await expect(page.locator('body')).toContainText('category');
    await page.getByTestId('next-step-btn').nth(2).click();

    await expect(page.locator('#categories-error')).toBeVisible();
    await expect(page.locator('#business-categories')).toHaveClass(/invalid-input/);
  });

  test('validates business name correctly', async ({ page }) => {
    await expect(page.locator('body')).toContainText('10-Minute Setup Wizard');

    await page.getByTestId('next-step-btn').click();
    await page.locator('input[value="field"]').click({ force: true });
    await page.getByTestId('next-step-btn').nth(1).click();

    await page.locator('#business-categories').selectOption('Other');
    await page.getByTestId('next-step-btn').nth(2).click();

    await expect(page.locator('body')).toContainText('name of your business');

    // empty submission
    await page.getByTestId('next-step-btn').nth(3).click();
    await expect(page.locator('#name-error')).toBeVisible();

    // valid submission
    await page.locator('#business-name').fill('My Awesome Business');
    await page.getByTestId('next-step-btn').nth(3).click();
    await expect(page.locator('body')).toContainText('Hire Your First Agent');
  });

  test('saves state to localStorage when clicking Save Draft', async ({ page }) => {
    await expect(page.locator('body')).toContainText('10-Minute Setup Wizard');

    await page.getByTestId('next-step-btn').click();
    await page.locator('input[value="field"]').click({ force: true });

    await page.getByTestId('save-draft-btn').nth(0).click();

    // wait for localstorage to populate
    await page.waitForTimeout(1000);
    const storedData = await page.evaluate(() => window.localStorage.getItem('onboardingState'));
    expect(storedData).toContain('field');
  });
});
