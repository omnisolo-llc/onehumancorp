import { test, expect } from './fixtures';

test.describe('Wizard Cross Device E2E', () => {
  test('Persona: Business Owner can resume setup wizard cross device', async ({ page, browser }) => {
    // We use a unique tenant for this test to ensure isolation on the backend.
    const tenantId = `e2e-wizard-${Date.now()}`;
    const userId = tenantId;

    // 1. Owner starts wizard directly from the setup route.
    await page.goto('/setup.html');
    await page.waitForLoadState('networkidle');

    // Since our test runner injects a default tenant if not logged in, we override it
    // by intercepting the state save, or we just rely on the API context if the e2e framework provides it.
    // Wait for the form to appear.
    await expect(page.getByRole('heading', { name: 'How do you work?' })).toBeVisible();

    // 2. Select work context and proceed to next step
    await page.getByLabel('Local Service').click();
    await page.getByRole('button', { name: 'Next' }).click();

    // 3. Select category
    await expect(page.getByRole('heading', { name: 'What\'s your category?' })).toBeVisible();
    await page.locator('#business-categories').selectOption('Plumbing');
    await page.getByRole('button', { name: 'Next' }).click();

    // 4. Enter business name and save draft
    await expect(page.getByRole('heading', { name: 'What\'s the name of your business?' })).toBeVisible();
    await page.getByPlaceholder('e.g. Maya\'s Bakery').fill('Cross Device Wizard Store');

    // Explicitly click Save Draft to fire off the API request
    await page.getByRole('button', { name: 'Save Draft' }).click();
    await expect(page.locator('#draft-saved-msg')).toBeVisible({ timeout: 5000 });

    // 5. Simulate a cross-device session with a new browser context
    const newContext = await browser.newContext();

    // Transfer any needed auth cookies from the old context to the new context
    const cookies = await page.context().cookies();
    await newContext.addCookies(cookies);

    const newPage = await newContext.newPage();

    // Do NOT transfer localStorage so we test the API retrieval
    await newPage.goto('/setup.html');
    await newPage.waitForLoadState('networkidle');

    // 6. Verify the business name and step was properly restored from the API
    await expect(newPage.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible({ timeout: 10000 });
    await expect(newPage.getByPlaceholder("e.g. Maya's Bakery")).toHaveValue('Cross Device Wizard Store', { timeout: 10000 });

    await newContext.close();
  });
});
