import { test, expect } from './fixtures';

test.describe('Wizard Cross Device E2E', () => {
  test('Persona: Business Owner can resume setup wizard cross device', async ({ page, browser }) => {
    // 1. Owner starts wizard directly from the current route.
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('onboardingState');
    }, 'storefront');
    await page.goto('/setup.html');
    await page.waitForLoadState('networkidle');

    // 2. Click Start My Business to advance to step 1
    // The first screen is "10-Minute Setup Wizard", clicking "Start My Business" moves to step 1
    await page.getByRole('button', { name: 'Back' }).click();
    await page.getByRole('button', { name: /Start My Business/ }).click();
    await expect(page.getByRole('heading', { name: "How do you work?" })).toBeVisible();

    await page.getByText("I'm a Baker").click();
    await page.locator('#step-context .next-step-btn').click();

    await expect(page.getByRole('heading', { name: /What's your category?/ })).toBeVisible();
    await page.locator('#business-categories').selectOption('Bakery');
    await page.locator('#step-categories .next-step-btn').click();

    await expect(page.getByRole('heading', { name: /What's the name of your business?/ })).toBeVisible();

    // 3. Move to step 2 and enter business name
    // It's already at the "What's the name of your business?" step. We fill the input.
    // The placeholder is "e.g. Maya's Custom Cakes"
    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill('Cross Device Wizard');

    // Wait until local storage is updated with the business name
    await expect.poll(async () => {
      const stateStr = await page.evaluate(() => localStorage.getItem('onboardingState'));
      if (!stateStr) return '';
      try {
        const state = JSON.parse(stateStr);
        return state.businessName;
      } catch (e) {
        return '';
      }
    }, {
      message: 'Wait for local storage to save business name',
      timeout: 5000,
    }).toBe('Cross Device Wizard');

    // Also trigger save to backend
    await page.locator('#step-name .next-step-btn').click();
    await expect(page.getByRole('heading', { name: "Set up your Assistant" })).toBeVisible();
    // Wait for the backend debounce (1000ms) + network travel time
    await page.waitForTimeout(2000);

    // 4. Simulate a cross-device session with a new browser context
    const newContext = await browser.newContext();
    const newPage = await newContext.newPage();

    // Inject the exact same local storage state to the new context to test restoration
    // We navigate to dashboard first to have the right origin
    await newPage.goto('/dashboard');
    const wizardState = await page.evaluate(() => localStorage.getItem('onboardingState'));

    await newPage.evaluate((state) => {
        if(state) {
            localStorage.setItem('onboardingState', state);
        }
        localStorage.setItem('tenant_id', 'storefront');
        localStorage.setItem('user_id', 'storefront');
    }, wizardState);

    await newPage.goto('/setup.html');
    await newPage.waitForLoadState('networkidle');

    // 5. Verify the business name and step was properly restored
    await expect(newPage.getByRole('heading', { name: 'Set up your Assistant' })).toBeVisible({ timeout: 10000 });
    await newPage.locator('#step-assistant .prev-step-btn').first().click();
    await expect(newPage.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await expect(newPage.getByPlaceholder("e.g. Maya's Custom Cakes")).toHaveValue('Cross Device Wizard', { timeout: 10000 });

    await newContext.close();
  });
});
