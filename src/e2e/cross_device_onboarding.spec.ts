// NOTE: E2E test runs for this flow are skipped locally/in sandbox due to a Docker/PGVector permission issue.
// They will be run manually in CI or when the sandbox issue is resolved.

import { test, expect } from './fixtures';

test.describe('Cross Device Onboarding CUJ', () => {
  test.beforeEach(async ({ page }) => {
    const id = `cross-device-${Date.now()}-${Math.random()}`;
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('ohc_wizard_state');
    }, id);
  });

  test('Persona: Business Owner can save draft and resume cross device', async ({ page }) => {
    // 1. Owner starts from the website builder page which shows the setup screen
    await page.goto('/website-builder');
    await expect(page.locator('#setup-screen')).toBeVisible();

    // 2. Open Setup Wizard and start
    await expect(page.getByRole('heading', { name: '10-Minute Setup Wizard' })).toBeVisible();
    await page.getByRole('button', { name: /Start My Business Next/i }).click();

    // Step 2: What kind of business are you building?
    await expect(page.getByRole('heading', { name: 'What kind of business are you building?' })).toBeVisible();
    await page.getByRole('button', { name: /Online Store/i }).click();

    // Step 3: Give your business a name
    await expect(page.getByRole('heading', { name: 'Give your business a name' })).toBeVisible();

    // 3. Owner enters business name and waits for auto-save debounce (500ms)
    const nameInput = page.getByPlaceholder(/e.g. Maya's Cakes/i);
    const responsePromise = page.waitForResponse(r => r.url().includes('/api/onboarding/state') && r.request().method() === 'POST' && r.request().postData()?.includes('Cross Device Bakery'));
    await nameInput.fill('Cross Device Bakery');
    await responsePromise;

    // 4. Simulate a cross-device session or reload
    await page.reload();

    // 5. Verify the business name was properly restored
    await expect(page.getByRole('heading', { name: 'Give your business a name' })).toBeVisible({ timeout: 10000 });
    await expect(page.getByPlaceholder(/e.g. Maya's Cakes/i)).toHaveValue('Cross Device Bakery', { timeout: 10000 });
  });
});
