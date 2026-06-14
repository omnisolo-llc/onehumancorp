import { test, expect } from './fixtures';

test.describe('Tauri Setup UI Cross Device State', () => {

  test('Cross device setup wizard resume from backend state', async ({ page, browser }) => {
    // 1. Initial browser context
    await page.goto('/ui/setup.html');
    await page.waitForLoadState('networkidle');

    // Simulate clicking through step 1
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

    // Setting standard local storage ID mimicking login state, then load setup
    await newPage.goto('/dashboard');
    await newPage.evaluate(() => {
        localStorage.setItem('tenant_id', 'storefront');
        localStorage.setItem('user_id', 'test-user');
    });

    await newPage.goto('/ui/setup.html');

    // Since the state comes from backend now (via backend API loading or Tauri invoke),
    // it should jump to step 3 (index 2) where it was left off, and inputs should be populated.
    await expect(newPage.getByRole('heading', { name: 'What\'s the name of your business?' })).toBeVisible();
    await expect(newPage.locator('#business-name')).toHaveValue('Carlos Plumbing Tools');

    // Check previous step category
    await newPage.getByRole('button', { name: 'Back' }).click();
    await expect(newPage.locator('#business-categories')).toHaveValue('Plumbing');

    await newContext.close();
  });

  test('Setup UI requires valid email format', async ({ page }) => {
    await page.goto('/ui/setup.html');
    // Navigate to step 5
    await page.getByText('Local Service').click();
    await page.getByRole('button', { name: 'Next' }).click();
    await page.locator('#business-categories').selectOption('Other');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.fill('#business-name', 'Test');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.fill('#assistant-name', 'Bot');
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: 'Admin Credentials' })).toBeVisible();
    await page.fill('#admin-email', 'invalid-email');
    await page.fill('#admin-password', 'password123');
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#email-error')).toBeVisible();
  });

  test('Setup UI requires at least 8 chars password', async ({ page }) => {
    await page.goto('/ui/setup.html');
    await page.getByText('Local Service').click();
    await page.getByRole('button', { name: 'Next' }).click();
    await page.locator('#business-categories').selectOption('Other');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.fill('#business-name', 'Test');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.fill('#assistant-name', 'Bot');
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.fill('#admin-email', 'test@example.com');
    await page.fill('#admin-password', 'pass');
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#password-error')).toBeVisible();
  });

  test('Setup UI allows finishing setup', async ({ page }) => {
    await page.goto('/ui/setup.html');
    await page.getByText('Local Service').click();
    await page.getByRole('button', { name: 'Next' }).click();
    await page.locator('#business-categories').selectOption('Other');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.fill('#business-name', 'Final Test Biz');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.fill('#assistant-name', 'Bot');
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.fill('#admin-email', 'test@example.com');
    await page.fill('#admin-password', 'password123');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.fill('#first-offer', 'My Offer');
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: 'Template Selection' })).toBeVisible();
    await page.locator('#template-selection').selectOption('Modern');
    await page.getByRole('button', { name: 'Finish Setup' }).click();
    // Finish setup redirects to success
    await expect(page).toHaveURL(/.*success.html/);
  });

  test('Setup UI Persona chips auto-fill the form correctly', async ({ page }) => {
    await page.goto('/ui/setup.html');
    await page.getByText('I\'m a Baker').click();
    await expect(page.locator('input[name="work_context"]:checked')).toHaveValue('Storefront');

    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#business-categories')).toHaveValue('Bakery');

    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#business-name')).toHaveValue("Maya's Bakery");
  });

});

test.describe('Tauri Setup UI Auto-save', () => {

  test('Auto-save debounces correctly on text input', async ({ page }) => {
    let saveRequests = 0;
    await page.route('**/api/onboarding/state', async route => {
      if (route.request().method() === 'POST') {
        saveRequests++;
        await route.fulfill({ status: 204 });
      } else {
        await route.fallback();
      }
    });

    await page.goto('/ui/setup.html');
    await page.getByText('Local Service').click();
    await page.getByRole('button', { name: 'Next' }).click();
    await page.locator('#business-categories').selectOption('Plumbing');
    await page.getByRole('button', { name: 'Next' }).click();

    // Type quickly to trigger debounce
    const nameInput = page.locator('#business-name');
    await nameInput.fill('A');
    await page.waitForTimeout(100);
    await nameInput.fill('Au');
    await page.waitForTimeout(100);
    await nameInput.fill('Aut');
    await page.waitForTimeout(100);
    await nameInput.fill('Auto');

    // Shouldn't have saved yet
    expect(saveRequests).toBe(0);

    // Wait for debounce timeout
    await page.waitForTimeout(1600);

    // Should have saved exactly once
    expect(saveRequests).toBe(1);
  });

  test('Auto-save triggers on select change', async ({ page }) => {
    let saveRequests = 0;
    await page.route('**/api/onboarding/state', async route => {
      if (route.request().method() === 'POST') {
        saveRequests++;
        await route.fulfill({ status: 204 });
      } else {
        await route.fallback();
      }
    });

    await page.goto('/ui/setup.html');
    await page.getByText('Local Service').click();
    await page.getByRole('button', { name: 'Next' }).click();

    // Change select option
    await page.locator('#business-categories').selectOption('Electrical');

    // Wait for debounce
    await page.waitForTimeout(1600);
    expect(saveRequests).toBe(1);
  });

  test('Auto-save triggers on radio button change', async ({ page }) => {
    let saveRequests = 0;
    await page.route('**/api/onboarding/state', async route => {
      if (route.request().method() === 'POST') {
        saveRequests++;
        await route.fulfill({ status: 204 });
      } else {
        await route.fallback();
      }
    });

    await page.goto('/ui/setup.html');

    // Click radio button
    await page.getByText('Online Creator').click();

    // Wait for debounce
    await page.waitForTimeout(1600);
    expect(saveRequests).toBe(1);
  });

  test('Auto-saved state can be resumed on a new device', async ({ page, browser }) => {
    await page.goto('/ui/setup.html');

    // Advance to step 3 and type something
    await page.getByText('Local Service').click();
    await page.getByRole('button', { name: 'Next' }).click();
    await page.locator('#business-categories').selectOption('Plumbing');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.fill('#business-name', 'Auto Saved Biz');

    // Wait for auto-save debounce
    await page.waitForTimeout(1600);

    // Open new context simulating a new device
    const newContext = await browser.newContext();
    const newPage = await newContext.newPage();

    await newPage.goto('/dashboard');
    await newPage.evaluate(() => {
        localStorage.setItem('tenant_id', 'storefront');
        localStorage.setItem('user_id', 'test-user');
    });

    await newPage.goto('/ui/setup.html');

    // Should resume on step 3 with populated text
    await expect(newPage.getByRole('heading', { name: 'What\'s the name of your business?' })).toBeVisible();
    await expect(newPage.locator('#business-name')).toHaveValue('Auto Saved Biz');

    await newContext.close();
  });

  test('Auto-save does not block the UI or show loading overlay', async ({ page }) => {
    await page.goto('/ui/setup.html');
    await page.getByText('Local Service').click();
    await page.getByRole('button', { name: 'Next' }).click();
    await page.locator('#business-categories').selectOption('Plumbing');
    await page.getByRole('button', { name: 'Next' }).click();

    // Route to delay backend response
    await page.route('**/api/onboarding/state', async route => {
      if (route.request().method() === 'POST') {
        await new Promise(r => setTimeout(r, 1000));
        await route.fulfill({ status: 204 });
      } else {
        await route.fallback();
      }
    });

    await page.fill('#business-name', 'Delay Test');

    // Wait just long enough for auto-save to trigger, but not finish
    await page.waitForTimeout(1600);

    // We should be able to type immediately, UI is not blocked
    await page.fill('#business-name', 'Delay Test Updated');
    await expect(page.locator('#business-name')).toHaveValue('Delay Test Updated');

    // The manual save draft button should not be showing "Saving..."
    await expect(page.locator('.step.active .save-draft-btn')).not.toHaveText('Saving...');
  });

});
