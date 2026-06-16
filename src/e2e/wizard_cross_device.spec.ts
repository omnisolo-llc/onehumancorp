import { test, expect } from './fixtures';

test.describe('Wizard Cross Device E2E', () => {
  test('Persona: Business Owner can resume setup wizard cross device', async ({ page, browser }) => {
    // 1. Owner starts wizard directly from the current route.
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('onboarding-storage-v3');
    }, 'storefront');
    await page.goto('/onboarding');
    await page.waitForLoadState('networkidle');

    // 2. The first screen is "Tell us about your business" for Instant Build. We fill the bio.
    await expect(page.getByRole('heading', { name: "Tell us about your business" })).toBeVisible();
    await page.getByPlaceholder("e.g. I run a local bakery").fill('Cross Device Wizard');
    await page.getByTestId('admin-email').fill('maya@example.com');
    await page.getByTestId('admin-password').fill('mypassword123');

    // Wait until local storage is updated with the bio
    await expect.poll(async () => {
      const stateStr = await page.evaluate(() => localStorage.getItem('onboarding-storage-v3'));
      if (!stateStr) return '';
      try {
        const state = JSON.parse(stateStr);
        return state.state.bio;
      } catch (e) {
        return '';
      }
    }, {
      message: 'Wait for local storage to save bio',
      timeout: 5000,
    }).toBe('Cross Device Wizard');

    // 3. Simulate a cross-device session with a new browser context
    const newContext = await browser.newContext();
    const newPage = await newContext.newPage();

    // Inject the exact same local storage state to the new context to test restoration
    // We navigate to dashboard first to have the right origin
    await newPage.goto('/dashboard');
    const wizardState = await page.evaluate(() => localStorage.getItem('onboarding-storage-v3'));

    await newPage.evaluate((state) => {
        if(state) {
            localStorage.setItem('onboarding-storage-v3', state);
        }
        localStorage.setItem('tenant_id', 'storefront');
        localStorage.setItem('user_id', 'storefront');
    }, wizardState);

    await newPage.goto('/onboarding');
    await newPage.waitForLoadState('networkidle');

    // 4. Verify the bio was properly restored
    await expect(newPage.getByRole('heading', { name: "Tell us about your business" })).toBeVisible({ timeout: 10000 });
    await expect(newPage.getByPlaceholder("e.g. I run a local bakery")).toHaveValue('Cross Device Wizard', { timeout: 10000 });

    await newContext.close();
  });
});
