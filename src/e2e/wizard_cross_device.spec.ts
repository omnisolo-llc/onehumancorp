import { test, expect } from './fixtures';

test.describe('Wizard Cross Device E2E', () => {
  test('Persona: Business Owner can resume setup wizard cross device', async ({ page, browser }) => {
    // 1. Owner starts wizard directly from the current route.
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('website-builder-storage');
    }, 'storefront');
    await page.goto('/website-builder');
    await page.waitForLoadState('networkidle');

    // 2. Click Start My Business to advance to step 1
    await page.getByRole('button', { name: /Start My Business/ }).click();
    await expect(page.getByRole('heading', { name: 'What kind of business are you building?' })).toBeVisible();

    // 3. Move to step 2 and enter business name
    await page.getByRole('button', { name: /Online Store/ }).click();
    await expect(page.getByRole('heading', { name: 'Give your business a name' })).toBeVisible();
    await page.getByPlaceholder('What is your business called?').fill('Cross Device Wizard');

    // Wait until local storage is updated with the business name
    await expect.poll(async () => {
      const stateStr = await page.evaluate(() => localStorage.getItem('website-builder-storage'));
      if (!stateStr) return '';
      try {
        const state = JSON.parse(stateStr);
        return state.state.businessName;
      } catch (e) {
        return '';
      }
    }, {
      message: 'Wait for local storage to save business name',
      timeout: 5000,
    }).toBe('Cross Device Wizard');

    // 4. Simulate a cross-device session with a new browser context
    const newContext = await browser.newContext();
    const newPage = await newContext.newPage();

    // Inject the exact same local storage state to the new context to test restoration
    // We navigate to dashboard first to have the right origin
    await newPage.goto('/dashboard');
    const wizardState = await page.evaluate(() => localStorage.getItem('website-builder-storage'));

    await newPage.evaluate((state) => {
        if(state) {
            localStorage.setItem('website-builder-storage', state);
        }
        localStorage.setItem('tenant_id', 'storefront');
        localStorage.setItem('user_id', 'storefront');
    }, wizardState);

    await newPage.goto('/website-builder');

    // 5. Verify the business name and step was properly restored
    await expect(newPage.getByRole('heading', { name: 'Give your business a name' })).toBeVisible();
    await expect(newPage.getByPlaceholder('What is your business called?')).toHaveValue('Cross Device Wizard', { timeout: 10000 });

    await newContext.close();
  });
});
