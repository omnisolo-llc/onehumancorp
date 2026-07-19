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
    await page.route('**/api/v1/onboarding/start_zero_click', async route => {
        await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ success: true }) });
    });
    await page.route('**/api/v1/onboarding/draft', async route => {
        await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({}) });
    });
    await page.goto('http://mock/setup.html');
  });

  test('successfully completes the wizard with drafting and instant image url', async ({ page }) => {
    await expect(page.locator('body')).toContainText('Generate My Workspace');

    // Sometimes there might be a loading transition, wait for it
    await page.waitForTimeout(2000);

    const bioInput = page.locator('#instant-bio');
    await expect(bioInput).toBeVisible();

    await bioInput.fill('My E2E Bakery');

    const imageUrlInput = page.locator('#instant-image-url');
    await expect(imageUrlInput).toBeVisible();
    await imageUrlInput.fill('https://example.com/bakery.png');

    await page.locator('#generate-storefront-btn').click();

    const successHeading = page.getByRole('heading', { name: /You're Live!/ });
    await expect(successHeading).toBeVisible({ timeout: 15000 });
  });

  test('successfully navigates through the wizard steps', async ({ page }) => {
    await expect(page.locator('body')).toContainText('Generate My Workspace');

    await page.getByTestId('next-step-btn').first().click();
    await expect(page.locator('body')).toContainText('Context');

    // Make a choice for context
    await page.getByTestId('context-storefront').click();
    await page.getByTestId('next-step-btn').nth(1).click();

    await expect(page.locator('body')).toContainText('Categories');
    await page.getByTestId('business-categories').selectOption('Home Baker');
    await page.getByTestId('next-step-btn').nth(2).click();

    await expect(page.locator('body')).toContainText('Name');
  });

  test('prevents progression if categories input is empty', async ({ page }) => {
    await expect(page.locator('body')).toContainText('Generate My Workspace');

    await page.getByTestId('next-step-btn').first().click();
    await page.getByTestId('context-storefront').click();
    await page.getByTestId('next-step-btn').nth(1).click();

    await expect(page.locator('body')).toContainText('Categories');
    await page.getByTestId('next-step-btn').nth(2).click();

    await expect(page.locator('#categories-error')).toBeVisible();
    await expect(page.locator('#business-categories')).toHaveClass(/invalid-input/);
  });

  test('validates business name correctly', async ({ page }) => {
    await expect(page.locator('body')).toContainText('Generate My Workspace');

    await page.getByTestId('next-step-btn').first().click();
    await page.getByTestId('context-storefront').click();
    await page.getByTestId('next-step-btn').nth(1).click();

    await page.getByTestId('business-categories').selectOption('Home Baker');
    await page.getByTestId('next-step-btn').nth(2).click();

    await expect(page.locator('body')).toContainText('Name');

    // empty submission
    await page.getByTestId('next-step-btn').nth(3).click();
    await expect(page.locator('#name-error')).toBeVisible();

    // valid submission
    await page.locator('#business-name').fill('My Awesome Business');
    await page.getByTestId('next-step-btn').nth(3).click();
    await expect(page.locator('body')).toContainText('Assistant');
  });

  test('saves state to localStorage when clicking Save Draft', async ({ page }) => {
    await expect(page.locator('body')).toContainText('Generate My Workspace');

    await page.getByTestId('next-step-btn').first().click();
    await page.getByTestId('context-storefront').click();

    await page.getByTestId('save-draft-btn').nth(0).click();

    // wait for localstorage to populate
    await page.waitForTimeout(1000);
    const storedData = await page.evaluate(() => window.localStorage.getItem('onboardingState'));
    expect(storedData).toContain('Storefront');
  });

  test('Assistant step validation error disappears upon selection', async ({ page }) => {
    await expect(page.locator('body')).toContainText('Generate My Workspace');

    await page.getByTestId('next-step-btn').first().click();
    await page.getByTestId('context-storefront').click();
    await page.getByTestId('next-step-btn').nth(1).click();
    await page.getByTestId('business-categories').selectOption('Home Baker');
    await page.getByTestId('next-step-btn').nth(2).click();
    await page.locator('#business-name').fill('My Awesome Business');
    await page.getByTestId('next-step-btn').nth(3).click();

    await expect(page.locator('body')).toContainText('Assistant');

    // Click Next without selecting an assistant to trigger error
    await page.getByTestId('next-step-btn').nth(4).click();

    // Expect error to be visible
    const errorMsg = page.locator('#assistant-name-error');
    await expect(errorMsg).toBeVisible();

    // Select an assistant
    await page.getByTestId('team-support').click();

    // The error should disappear immediately
    await expect(errorMsg).not.toHaveClass(/active/);
  });

  test('Assistant tone validation error disappears upon selection', async ({ page }) => {
    await expect(page.locator('body')).toContainText('Generate My Workspace');

    await page.getByTestId('next-step-btn').first().click();
    await page.getByTestId('context-storefront').click();
    await page.getByTestId('next-step-btn').nth(1).click();
    await page.getByTestId('business-categories').selectOption('Home Baker');
    await page.getByTestId('next-step-btn').nth(2).click();
    await page.locator('#business-name').fill('My Awesome Business');
    await page.getByTestId('next-step-btn').nth(3).click();

    await expect(page.locator('body')).toContainText('Assistant');

    // Select an assistant so only tone is missing
    await page.getByTestId('team-support').click();

    // Click Next to trigger tone error
    await page.getByTestId('next-step-btn').nth(4).click();

    // Expect error to be visible
    const errorMsg = page.locator('#tone-error');
    await expect(errorMsg).toBeVisible();

    // Select a tone
    await page.getByTestId('assistant-tone').selectOption('Friendly');

    // The error should disappear immediately
    await expect(errorMsg).not.toHaveClass(/active/);
  });

  test('Back button on domain step navigates to target audience step', async ({ page }) => {
    await expect(page.locator('body')).toContainText('Generate My Workspace');

    await page.getByTestId('next-step-btn').first().click();
    await page.getByTestId('context-storefront').click();
    await page.getByTestId('next-step-btn').nth(1).click();
    await page.getByTestId('business-categories').selectOption('Home Baker');
    await page.getByTestId('next-step-btn').nth(2).click();
    await page.locator('#business-name').fill('My Awesome Business');
    await page.getByTestId('next-step-btn').nth(3).click();

    // Assistant step
    await page.getByTestId('team-support').click();
    await page.getByTestId('assistant-tone').selectOption('Friendly');
    await page.getByTestId('next-step-btn').nth(4).click();

    // Admin Setup step
    await page.getByTestId('admin-name').fill('John Doe');
    await page.getByTestId('admin-email').fill('john.doe@example.com');
    await page.getByTestId('admin-password').fill('password123');
    await page.getByTestId('next-step-btn').nth(5).click();

    // What you sell step
    await expect(page.locator('body')).toContainText('Offer');
    await page.getByTestId('first-offer').fill('I sell awesome products');
    await page.getByTestId('next-step-btn').nth(6).click();

    // Location step
    await expect(page.locator('body')).toContainText('Location');
    await page.getByTestId('location-input').fill('New York, NY');
    await page.getByTestId('next-step-btn').nth(7).click();

    // Target Audience step
    await expect(page.locator('body')).toContainText('Target Audience');
    await page.getByTestId('target-audience').fill('Everyone');
    await page.getByTestId('next-step-btn').nth(8).click();

    // Domain step
    await expect(page.locator('body')).toContainText('Domain');
    await page.getByTestId('domain-name').fill('mybusiness');

    // Wait for the animation to finish
    await page.waitForTimeout(500);

    // Click back button inside step-domain
    const domainStep = page.locator('#step-domain');
    await domainStep.getByTestId('prev-step-btn').click();

    // Assert that we are back at Target Audience step
    await expect(page.locator('body')).toContainText('Target Audience');
    await expect(page.locator('#step-target-audience')).toBeVisible();
    await expect(page.locator('#step-domain')).not.toHaveClass(/active/);
  });

  test('Back button on location step navigates to offer step', async ({ page }) => {
    await expect(page.locator('body')).toContainText('Generate My Workspace');

    await page.getByTestId('next-step-btn').first().click();
    await page.getByTestId('context-storefront').click();
    await page.getByTestId('next-step-btn').nth(1).click();
    await page.getByTestId('business-categories').selectOption('Home Baker');
    await page.getByTestId('next-step-btn').nth(2).click();
    await page.locator('#business-name').fill('My Awesome Business');
    await page.getByTestId('next-step-btn').nth(3).click();

    // Assistant step
    await page.getByTestId('team-support').click();
    await page.getByTestId('assistant-tone').selectOption('Friendly');
    await page.getByTestId('next-step-btn').nth(4).click();

    // Admin Setup step
    await page.getByTestId('admin-name').fill('John Doe');
    await page.getByTestId('admin-email').fill('john.doe@example.com');
    await page.getByTestId('admin-password').fill('password123');
    await page.getByTestId('next-step-btn').nth(5).click();

    // What you sell step
    await expect(page.locator('body')).toContainText('Offer');
    await page.getByTestId('first-offer').fill('I sell awesome products');
    await page.getByTestId('next-step-btn').nth(6).click();

    // Location step
    await expect(page.locator('body')).toContainText('Location');
    await page.getByTestId('location-input').fill('New York, NY');

    // Click back button inside step-location
    const locationStep = page.locator('#step-location');
    await locationStep.getByTestId('prev-step-btn').click();

    // Assert that we are back at Offer step
    await expect(page.locator('body')).toContainText('Offer');
    await expect(page.locator('#step-offer')).toBeVisible();
    await expect(page.locator('#step-location')).not.toHaveClass(/active/);
  });
});
