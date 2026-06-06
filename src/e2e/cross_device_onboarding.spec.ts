import { test, expect } from './fixtures';

test.describe('Cross Device Onboarding CUJ', () => {
  test('Persona: Business Owner can save draft and resume cross device', async ({ page }) => {
    const id = `cross-device-${Date.now()}-${Math.random()}`;
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('ohc_wizard_state');
      localStorage.removeItem('onboarding-storage-v3');
      localStorage.removeItem('website-builder-storage');
    }, id);

    // 1. Owner starts onboarding directly from the current route.
    await page.goto('/website-builder');
    await page.waitForLoadState('networkidle');

    // Check we landed on the Welcome screen.
    await expect(page.getByText(/Your business, live in minutes./)).toBeVisible();

    // Choose start business manually
    await page.getByRole('button', { name: /Start My Business/i }).click();

    // Verify it landed on the Business Type screen
    await expect(page.getByText('What kind of business are you building?')).toBeVisible();

    // Select type and progress
    await page.getByRole('button', { name: /Online Store/i }).click();

    // 2. Owner enters business name
    await expect(page.getByText('Give your business a name')).toBeVisible();
    const nameInput = page.getByPlaceholder(/What is your business called\?/i);
    await nameInput.fill('Cross Device Bakery');

    // 3. Click Save Draft
    const saveDraftBtn = page.getByRole('button', { name: /Save Draft/i }).first();
    await saveDraftBtn.click();
    await expect(page.getByText('Draft Saved!')).toBeVisible();

    // Clear local storage to simulate device switch but keep tenant info to restore the backend state
    await page.evaluate((tenantId) => {
      window.localStorage.clear();
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
    }, id);

    // 4. Simulate a cross-device session or reload
    await page.reload();

    // 5. Verify the business name was properly restored to 'Cross Device Bakery'
    // This expects to find the input restored with the saved state.
    await expect(page.getByText('Give your business a name')).toBeVisible();
    await expect(page.getByPlaceholder(/What is your business called\?/i)).toHaveValue('Cross Device Bakery', { timeout: 10000 });
  });
});
