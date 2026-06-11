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
