import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Flow', () => {


  test.beforeEach(async ({ page }) => {
    const fs = require('fs');
    const path = require('path');
    const tauriUiDir = path.join(process.cwd(), '../ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });
    await page.goto('http://mock/setup.html');
  });

  test('successfully completes the wizard with drafting and instant image url', async ({ page }) => {
    await expect(page.locator('body')).toContainText('Tell us about your business');

    // Sometimes there might be a loading transition, wait for the Instant Build button to be ready
    await page.waitForTimeout(2000);

    // There are multiple ways to click Instant Build, we added ID "instant-build-btn-text"
    // await page.getByText('Instant Build').click(); // Instant build is replaced by conversational setup/manual

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
    await expect(page.locator('body')).toContainText('Tell us about your business');

    await page.getByRole('button', { name: 'Step-by-Step Setup' }).click();
    await expect(page.locator('body')).toContainText('How do you work?');

    // Make a choice for context
    await page.getByText('Local Service').click();
    await page.locator('#step-context').getByRole('button', { name: 'Next' }).click();

    await expect(page.locator('body')).toContainText('category');
    await page.locator('#business-categories').selectOption('Other');
    await page.locator('#step-categories').getByRole('button', { name: 'Next' }).click();

    await expect(page.locator('body')).toContainText('name of your business');
  });

  test('prevents progression if categories input is empty', async ({ page }) => {
    await expect(page.locator('body')).toContainText('Tell us about your business');

    await page.getByRole('button', { name: 'Step-by-Step Setup' }).click();
    await page.getByText('Local Service').click();
    await page.locator('#step-context').getByRole('button', { name: 'Next' }).click();

    await expect(page.locator('body')).toContainText('category');
    await page.locator('#step-categories').getByRole('button', { name: 'Next' }).click();

    await expect(page.locator('#categories-error')).toBeVisible();
    await expect(page.locator('#business-categories')).toHaveClass(/invalid-input/);
  });

  test('validates business name correctly', async ({ page }) => {
    await expect(page.locator('body')).toContainText('Tell us about your business');

    await page.getByRole('button', { name: 'Step-by-Step Setup' }).click();
    await page.getByText('Local Service').click();
    await page.locator('#step-context').getByRole('button', { name: 'Next' }).click();

    await page.locator('#business-categories').selectOption('Other');
    await page.locator('#step-categories').getByRole('button', { name: 'Next' }).click();

    await expect(page.locator('body')).toContainText('name of your business');

    // empty submission
    await page.locator('#step-name').getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#name-error')).toBeVisible();

    // valid submission
    await page.locator('#business-name').fill('My Awesome Business');
    await page.locator('#step-name').getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('body')).toContainText('Set up your Assistant');
  });

  test('saves state to localStorage when clicking Save Draft', async ({ page }) => {
    await expect(page.locator('body')).toContainText('Tell us about your business');

    await page.getByRole('button', { name: 'Step-by-Step Setup' }).click();
    await page.getByText('Local Service').click();

    await page.getByTestId('save-draft-btn').nth(0).click();

    // wait for localstorage to populate
    await page.waitForTimeout(1000);
    const storedData = await page.evaluate(() => window.localStorage.getItem('onboardingState'));
    expect(storedData).toContain('Local Service');
  });
});
