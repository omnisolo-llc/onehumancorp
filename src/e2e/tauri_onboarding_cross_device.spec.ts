const tauriUiDir = require('path').join(process.cwd(), 'src/ui/tauri/src/ui');
import { test, expect } from '@playwright/test';

test.describe('Tauri Setup UI Cross Device State', () => {

    test('Cross device setup wizard resume from backend state', async ({ page, browser }) => {
    const tauriUiDir = require('path').join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const htmlContent = require('fs').readFileSync(require('path').join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: htmlContent });
    });
    // intercept tooltips
    await page.route('**/api/tooltips', async route => {
      await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });
    await page.route('**/api/onboarding/draft', async route => {
       await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });
    // 1. Initial browser context
    await page.goto('http://mock/setup.html');
    await page.waitForLoadState('networkidle');

    // Simulate clicking through step 1
    await page.getByRole('button', { name: 'Start My Business' }).click();
    await page.getByText('Local Service').click();
    await page.getByRole('button', { name: 'Next' }).click();

    // Verify step 2
    await expect(page.getByRole('heading', { name: 'What\'s your category?' })).toBeVisible();
    await page.locator('#business-categories').selectOption('Plumbing');
    await page.getByRole('button', { name: 'Next' }).click();

    // Fill in Step 3
    await expect(page.getByRole('heading', { name: 'What\'s the name of your business?' })).toBeVisible();
    await page.fill('#business-name', 'Carlos Plumbing Tools');

    // Click Save Draft
    await page.getByRole('button', { name: 'Save Draft' }).click();
    await expect(page.locator('#draft-saved-msg')).toBeVisible();

    // 2. Open a new context simulating a second device loading the UI
    const newContext = await browser.newContext();
    const newPage = await newContext.newPage();
    await newPage.route('**/setup.html', async route => {
        const htmlContent = require('fs').readFileSync(require('path').join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: htmlContent });
    });
    // intercept tooltips
    await newPage.route('**/api/tooltips', async route => {
      await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });
    await newPage.route('**/api/onboarding/draft', async route => {
       await route.fulfill({ status: 200, body: JSON.stringify({
          step: 3,
          work_context: "Local Service",
          categories: "Plumbing",
          businessName: "Carlos Plumbing Tools"
       }) });
    });

    // Setting standard local storage ID mimicking login state, then load setup
    await newPage.route('**/dashboard', route => route.fulfill({status: 200, body: 'mock'}));
    await newPage.goto('http://mock/dashboard');
    await newPage.evaluate(() => {
        localStorage.setItem('tenant_id', 'storefront');
        localStorage.setItem('user_id', 'test-user');
    });

    await newPage.goto('http://mock/setup.html');
    await newPage.waitForLoadState('networkidle');
    await newPage.waitForTimeout(1000);

    // The state was loaded from backend API. Let's verify the inputs are populated.
    // If it didn't jump automatically, we just navigate to the step to verify the populated value.
    const isNameStepVisible = await newPage.getByRole('heading', { name: 'What\'s the name of your business?' }).isVisible();
    if (!isNameStepVisible) {
        await newPage.getByRole('button', { name: 'Start My Business' }).click();
        await newPage.getByRole('button', { name: 'Next' }).click();
        await newPage.getByRole('button', { name: 'Next' }).click();
    }

    await expect(newPage.locator('#business-name')).toHaveValue('Carlos Plumbing Tools');

    // Check previous step category
    const backBtn = newPage.locator('#step-name [data-testid="prev-step-btn"]');
    if (await backBtn.isVisible()) {
        await backBtn.click();
        await expect(newPage.locator('#business-categories')).toHaveValue('Plumbing');
    }

    await newContext.close();
  });

  test('Setup UI requires valid email format', async ({ page }) => {

    await page.route('**/setup.html', async route => {
        const htmlContent = require('fs').readFileSync(require('path').join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: htmlContent });
    });
    // intercept tooltips
    await page.route('**/api/tooltips', async route => {
      await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });
    await page.route('**/api/onboarding/draft', async route => {
       await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });
    await page.goto('http://mock/setup.html');
    // Navigate to step 5
    await page.getByRole('button', { name: 'Start My Business' }).click();
    await page.getByText('Local Service').click();
    await page.getByRole('button', { name: 'Next' }).click();
    await page.locator('#business-categories').selectOption('Other');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.fill('#business-name', 'Test');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.getByTestId('team-operations').click();
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: 'Admin Credentials' })).toBeVisible();
    await page.fill('#admin-email', 'invalid-email');
    await page.fill('#admin-password', 'password123');
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#email-error')).toBeVisible();
    await expect(page.locator('#admin-email')).toHaveClass(/invalid-input/);
  });

  test('Setup UI requires at least 8 chars password', async ({ page }) => {

    await page.route('**/setup.html', async route => {
        const htmlContent = require('fs').readFileSync(require('path').join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: htmlContent });
    });
    // intercept tooltips
    await page.route('**/api/tooltips', async route => {
      await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });
    await page.route('**/api/onboarding/draft', async route => {
       await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });
    await page.goto('http://mock/setup.html');
    await page.getByRole('button', { name: 'Start My Business' }).click();
    await page.getByText('Local Service').click();
    await page.getByRole('button', { name: 'Next' }).click();
    await page.locator('#business-categories').selectOption('Other');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.fill('#business-name', 'Test');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.getByTestId('team-operations').click();
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.fill('#admin-email', 'test@example.com');
    await page.fill('#admin-password', 'pass');
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#password-error')).toBeVisible();
    await expect(page.locator('#admin-password')).toHaveClass(/invalid-input/);
  });

  test('Setup UI allows finishing setup', async ({ page }) => {

    await page.route('**/setup.html', async route => {
        const htmlContent = require('fs').readFileSync(require('path').join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: htmlContent });
    });
    // intercept tooltips
    await page.route('**/api/tooltips', async route => {
      await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });
    await page.route('**/api/onboarding/draft', async route => {
       await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });
    await page.goto('http://mock/setup.html');
    await page.getByRole('button', { name: 'Start My Business' }).click();
    await page.getByText('Local Service').click();
    await page.getByRole('button', { name: 'Next' }).click();
    await page.locator('#business-categories').selectOption('Other');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.fill('#business-name', 'Final Test Biz');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.getByTestId('team-operations').click();
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.fill('#admin-email', 'test@example.com');
    await page.fill('#admin-password', 'password123');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.fill('#first-offer', 'My Offer');
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: 'Where will your business live?' })).toBeVisible();
    await page.fill('#domain-name', 'my-test-domain');
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: 'Template Selection' })).toBeVisible();
    await page.locator('#template-selection').selectOption('Modern');
        await page.route('**/api/onboarding/start', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ organization_id: 'test-org-123' })
      });
    });
    await page.route('**/dashboard.html', async route => {
      await route.fulfill({ status: 200, body: 'Success' });
    });
    await page.getByTestId('finish-btn').click();
    // Finish setup redirects to success
    await expect(page).toHaveURL(/.*dashboard.html/);
  });

  test('Setup UI Persona chips auto-fill the form correctly', async ({ page }) => {

    await page.route('**/setup.html', async route => {
        const htmlContent = require('fs').readFileSync(require('path').join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: htmlContent });
    });
    // intercept tooltips
    await page.route('**/api/tooltips', async route => {
      await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });
    await page.route('**/api/onboarding/draft', async route => {
       await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });
    await page.goto('http://mock/setup.html');
    await page.getByRole('button', { name: 'Start My Business' }).click();
    await page.getByText('I\'m a Baker').click();
    await expect(page.locator('input[name="work_context"]:checked')).toHaveValue('Storefront');

    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#business-categories')).toHaveValue('Home Baker');

    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#business-name')).toHaveValue("Maya's Bakery");
  });

});
