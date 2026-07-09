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
    await expect(page.locator('body')).toContainText('Tell us about your business');

    // Sometimes there might be a loading transition, wait for it
    await page.waitForTimeout(2000);

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

    await page.getByTestId('next-step-btn').first().click();
    await expect(page.locator('body')).toContainText('How do you work');

    // Make a choice for context
    await page.getByTestId('context-storefront').click();
    await page.getByTestId('next-step-btn').nth(1).click();

    await expect(page.locator('body')).toContainText('category');
    await page.locator('#business-categories').selectOption('Other');
    await page.getByTestId('next-step-btn').nth(2).click();

    await expect(page.locator('body')).toContainText('name of your business');
  });

  test('prevents progression if categories input is empty', async ({ page }) => {
    await expect(page.locator('body')).toContainText('Tell us about your business');

    await page.getByTestId('next-step-btn').first().click();
    await page.getByTestId('context-storefront').click();
    await page.getByTestId('next-step-btn').nth(1).click();

    await expect(page.locator('body')).toContainText('category');
    await page.getByTestId('next-step-btn').nth(2).click();

    await expect(page.locator('#categories-error')).toBeVisible();
    await expect(page.locator('#business-categories')).toHaveClass(/invalid-input/);
  });

  test('validates business name correctly', async ({ page }) => {
    await expect(page.locator('body')).toContainText('Tell us about your business');

    await page.getByTestId('next-step-btn').first().click();
    await page.getByTestId('context-storefront').click();
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
    await expect(page.locator('body')).toContainText('Set up your Assistant');
  });

  test('saves state to localStorage when clicking Save Draft', async ({ page }) => {
    await expect(page.locator('body')).toContainText('Tell us about your business');

    await page.getByTestId('next-step-btn').first().click();
    await page.getByTestId('context-storefront').click();

    await page.getByTestId('save-draft-btn').nth(0).click();

    // wait for localstorage to populate
    await page.waitForTimeout(1000);
    const storedData = await page.evaluate(() => window.localStorage.getItem('onboardingState'));
    expect(storedData).toContain('Storefront');
  });

  test('Assistant step validation error disappears upon selection', async ({ page }) => {
    await expect(page.locator('body')).toContainText('Tell us about your business');

    await page.getByTestId('next-step-btn').first().click();
    await page.getByTestId('context-storefront').click();
    await page.getByTestId('next-step-btn').nth(1).click();
    await page.locator('#business-categories').selectOption('Other');
    await page.getByTestId('next-step-btn').nth(2).click();
    await page.locator('#business-name').fill('My Awesome Business');
    await page.getByTestId('next-step-btn').nth(3).click();

    await expect(page.locator('body')).toContainText('Set up your Assistant');

    // Click Next without selecting an assistant to trigger error
    await page.getByTestId('next-step-btn').nth(4).click();

    // Expect error to be visible
    const errorMsg = page.locator('#assistant-name-error');
    await expect(errorMsg).toBeVisible();

    // Select an assistant
    await page.getByTestId('team-support').click();

    // The error should disappear immediately
    await expect(errorMsg).toBeHidden();
  });

  test('Assistant tone validation error disappears upon selection', async ({ page }) => {
    await expect(page.locator('body')).toContainText('Tell us about your business');

    await page.getByTestId('next-step-btn').first().click();
    await page.getByTestId('context-storefront').click();
    await page.getByTestId('next-step-btn').nth(1).click();
    await page.locator('#business-categories').selectOption('Other');
    await page.getByTestId('next-step-btn').nth(2).click();
    await page.locator('#business-name').fill('My Awesome Business');
    await page.getByTestId('next-step-btn').nth(3).click();

    await expect(page.locator('body')).toContainText('Set up your Assistant');

    // Select an assistant so only tone is missing
    await page.getByTestId('team-support').click();

    // Click Next to trigger tone error
    await page.getByTestId('next-step-btn').nth(4).click();

    // Expect error to be visible
    const errorMsg = page.locator('#tone-error');
    await expect(errorMsg).toBeVisible();

    // Select a tone
    await page.locator('#assistant-tone').selectOption('Professional');

    // The error should disappear immediately
    await expect(errorMsg).toBeHidden();
  });

  test('Back button on domain step navigates to target audience step', async ({ page }) => {
    await expect(page.locator('body')).toContainText('Tell us about your business');

    await page.getByTestId('next-step-btn').first().click();
    await page.getByTestId('context-storefront').click();
    await page.getByTestId('next-step-btn').nth(1).click();
    await page.locator('#business-categories').selectOption('Other');
    await page.getByTestId('next-step-btn').nth(2).click();
    await page.locator('#business-name').fill('My Awesome Business');
    await page.getByTestId('next-step-btn').nth(3).click();

    // Assistant step
    await page.getByTestId('team-support').click();
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.getByTestId('next-step-btn').nth(4).click();

    // Admin Setup step
    await page.locator('#admin-name').fill('John Doe');
    await page.locator('#admin-email').fill('john.doe@example.com');
    await page.locator('#admin-password').fill('password123');
    await page.getByTestId('next-step-btn').nth(5).click();

    // What you sell step
    await expect(page.locator('body')).toContainText('What do you sell?');
    await page.locator('#first-offer').fill('I sell awesome products');
    await page.getByTestId('next-step-btn').nth(6).click();

    // Location step
    await expect(page.locator('body')).toContainText('Where are you located?');
    await page.locator('#location-input').fill('New York, NY');
    await page.getByTestId('next-step-btn').nth(7).click();

    // Target Audience step
    await expect(page.locator('body')).toContainText('Who is your target audience?');
    await page.locator('#target-audience').fill('Everyone');
    await page.getByTestId('next-step-btn').nth(8).click();

    // Domain step
    await expect(page.locator('body')).toContainText('Where will your business live?');
    await page.locator('#domain-name').fill('mybusiness');

    // Wait for the animation to finish
    await page.waitForTimeout(500);

    // Click back button inside step-domain
    const domainStep = page.locator('#step-domain');
    await domainStep.getByTestId('prev-step-btn').click();

    // Assert that we are back at Target Audience step
    await expect(page.locator('body')).toContainText('Who is your target audience?');
    await expect(page.locator('#step-target-audience')).toHaveClass(/active/);
    await expect(page.locator('#step-domain')).toBeHidden();
  });

  test('Back button on location step navigates to offer step', async ({ page }) => {
    await expect(page.locator('body')).toContainText('Tell us about your business');

    await page.getByTestId('next-step-btn').first().click();
    await page.getByTestId('context-storefront').click();
    await page.getByTestId('next-step-btn').nth(1).click();
    await page.locator('#business-categories').selectOption('Other');
    await page.getByTestId('next-step-btn').nth(2).click();
    await page.locator('#business-name').fill('My Awesome Business');
    await page.getByTestId('next-step-btn').nth(3).click();

    // Assistant step
    await page.getByTestId('team-support').click();
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.getByTestId('next-step-btn').nth(4).click();

    // Admin Setup step
    await page.locator('#admin-name').fill('John Doe');
    await page.locator('#admin-email').fill('john.doe@example.com');
    await page.locator('#admin-password').fill('password123');
    await page.getByTestId('next-step-btn').nth(5).click();

    // What you sell step
    await expect(page.locator('body')).toContainText('What do you sell?');
    await page.locator('#first-offer').fill('I sell awesome products');
    await page.getByTestId('next-step-btn').nth(6).click();

    // Location step
    await expect(page.locator('body')).toContainText('Where are you located?');
    await page.locator('#location-input').fill('New York, NY');

    // Click back button inside step-location
    const locationStep = page.locator('#step-location');
    await locationStep.getByTestId('prev-step-btn').click();

    // Assert that we are back at Offer step
    await expect(page.locator('body')).toContainText('What do you sell?');
    await expect(page.locator('#step-offer')).toHaveClass(/active/);
    await expect(page.locator('#step-location')).toBeHidden();
  });
});
