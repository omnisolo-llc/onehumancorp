import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Onboarding Wizard CUJ', () => {
  test.beforeEach(async ({ page }) => {
    await page.addInitScript(() => window.localStorage.clear());
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? require('path').join(process.env.TEST_SRCDIR || require('path').resolve(__dirname, '..', '..'), process.env.TEST_WORKSPACE)
        : require('path').resolve(__dirname, '..', '..');
    const tauriUiDir = require('path').join(workspaceRoot, 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const htmlContent = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: htmlContent });
    });
    // intercept tooltips
    await page.route('**/api/tooltips', async route => {
      await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });

    await page.route('**/api/onboarding/draft', async route => {
       await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });

    await page.route('**/api/onboarding/start', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ organization_id: 'test-org-123' })
      });
    });

    await page.route('**/success.html', async route => {
      await route.fulfill({ status: 200, body: 'Success' });
    });
  });

  async function startOnboarding(page: import('@playwright/test').Page) {
    await page.goto('http://mock/setup.html');
    await expect(page.locator('.container')).toBeVisible();
    await page.locator('[data-testid="next-step-btn"][data-next="step-context"]').click();
    await expect(page.getByRole('heading', { name: 'How do you work?' })).toBeVisible();
  }

  // Test 1: Persona navigates from home, starts onboarding
  test('Persona: Business Owner completes initial setup successfully', async ({ page }) => {
    await startOnboarding(page);

    await page.locator('[data-testid="context-storefront"]').click();
    await page.locator('[data-testid="next-step-btn"][data-next="step-categories"]').click();

    const categorySelect = page.getByTestId('business-categories');
    await expect(categorySelect).toBeVisible();
    await categorySelect.selectOption('Home Baker');
    await page.locator('[data-testid="next-step-btn"][data-next="step-name"]').click();

    await page.getByTestId('business-name').fill('Maya');
    await page.locator('[data-testid="next-step-btn"][data-next="step-assistant"]').click();

    await page.getByTestId('team-operations').click();
    await page.getByTestId('assistant-tone').selectOption('Friendly');
    await page.locator('[data-testid="next-step-btn"][data-next="step-admin"]').click();

    const adminName = page.locator('#admin-name');
    await adminName.fill('Test Admin');
    await page.getByTestId('admin-email').fill('admin@testbakery.local');
    await page.getByTestId('admin-password').fill('SuperSecretPassword123');
    await page.locator('[data-testid="next-step-btn"][data-next="step-offer"]').click();

    await page.getByTestId('first-offer').fill('Chocolate Cake');

    // In `setup.html`, step-offer transitions to step-location
    // `data-next="step-location"`
    await page.locator('#step-offer [data-testid="next-step-btn"][data-next="step-location"]').click();

    await page.getByTestId('location-input').fill('123 Bakery Lane');
    await page.locator('#step-location [data-testid="next-step-btn"][data-next="step-target-audience"]').click();

    await page.getByTestId('target-audience').fill('Local families');
    await page.locator('#step-target-audience [data-testid="next-step-btn"][data-next="step-domain"]').click();

    await page.getByTestId('domain-name').fill('maya-bakery');
    await page.locator('#step-domain [data-testid="next-step-btn"][data-next="step-template"]').click();

    await page.getByTestId('template-selection').selectOption('Modern');

    await expect(page.getByTestId('finish-btn')).toBeVisible();
    await page.getByTestId('finish-btn').click();

  });

  // Test 2: Ensure validation fails on small name
  test('Persona: Business Owner fails validation on short business name', async ({ page }) => {
    await startOnboarding(page);

    await page.locator('label', { hasText: 'Storefront or Cafe' }).click();
    await page.locator('[data-testid="next-step-btn"][data-next="step-categories"]').click();

    const categorySelect = page.getByTestId('business-categories');
    await expect(categorySelect).toBeVisible();
    await categorySelect.selectOption('Home Baker');
    await page.locator('[data-testid="next-step-btn"][data-next="step-name"]').click();

    await page.getByTestId('business-name').fill('M');

    // Attempt to go to next step
    await page.locator('[data-testid="next-step-btn"][data-next="step-assistant"]').click();

    // Expect validation failure message immediately
    // Wait for the name-error div to become visible and check its content.
    await expect(page.locator('#name-error')).toBeVisible();
  });

  // Test 3: Validate missing location blocks progression
  test('Persona: Business Owner cannot progress without offer', async ({ page }) => {
    await startOnboarding(page);
    // Test the actual failure for some other step if location is removed, maybe missing offer blocks progression?
    await page.locator('label', { hasText: 'Storefront or Cafe' }).click();
    await page.locator('[data-testid="next-step-btn"][data-next="step-categories"]').click();
    const categorySelect = page.getByTestId('business-categories');
    await categorySelect.selectOption('Home Baker');
    await page.locator('[data-testid="next-step-btn"][data-next="step-name"]').click();
    await page.getByTestId('business-name').fill('Maya');
    await page.locator('[data-testid="next-step-btn"][data-next="step-assistant"]').click();
    await page.getByTestId('team-operations').click();
    await page.getByTestId('assistant-tone').selectOption('Friendly');
    await page.locator('[data-testid="next-step-btn"][data-next="step-admin"]').click();

    // Fill in admin credentials
    const adminName = page.locator('#admin-name');
    await adminName.fill('Test Admin');
    await page.getByTestId('admin-email').fill('admin@testbakery.local');
    await page.getByTestId('admin-password').fill('SuperSecretPassword123');
    await page.locator('[data-testid="next-step-btn"][data-next="step-offer"]').click();

    // Do not fill offer, try to proceed
    await page.locator('#step-offer [data-testid="next-step-btn"][data-next="step-location"]').click();

    // Expect validation failure message
    await expect(page.getByText('Please tell us what you sell.')).toBeVisible();
  });

  // Test 4: Navigating Back works
  test('Persona: Business Owner can navigate back from sell step', async ({ page }) => {
    await startOnboarding(page);

    await page.locator('label', { hasText: 'Storefront or Cafe' }).click();
    await page.locator('[data-testid="next-step-btn"][data-next="step-categories"]').click();

    const categorySelect = page.getByTestId('business-categories');
    await expect(categorySelect).toBeVisible();
    await categorySelect.selectOption('Home Baker');
    await page.locator('[data-testid="next-step-btn"][data-next="step-name"]').click();

    await page.getByTestId('business-name').fill('Maya');
    await page.locator('[data-testid="next-step-btn"][data-next="step-assistant"]').click();

    await expect(page.getByRole('heading', { name: 'Set up your Assistant' })).toBeVisible();

    // Go back
    await page.locator('#step-assistant [data-testid="prev-step-btn"]').click();
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
  });

  // Test 5: Can cancel from Style & Team
  test('Persona: Business Owner can change Assistant Tone', async ({ page }) => {
    await startOnboarding(page);

    await page.locator('label', { hasText: 'Storefront or Cafe' }).click();
    await page.locator('[data-testid="next-step-btn"][data-next="step-categories"]').click();

    const categorySelect = page.getByTestId('business-categories');
    await expect(categorySelect).toBeVisible();
    await categorySelect.selectOption('Home Baker');
    await page.locator('[data-testid="next-step-btn"][data-next="step-name"]').click();

    await page.getByTestId('business-name').fill('Maya');
    await page.locator('[data-testid="next-step-btn"][data-next="step-assistant"]').click();

    // Test the assistant auto-respond or tone settings which is the new equivalent
    const assistantToneSelect = page.getByTestId('assistant-tone');
    await expect(assistantToneSelect).toBeVisible();
    await assistantToneSelect.selectOption('Professional');
    await expect(assistantToneSelect).toHaveValue('Professional');
  });
});
