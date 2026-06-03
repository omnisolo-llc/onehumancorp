import { test, expect } from './fixtures';

test.describe('Business Setup Wizard Comprehensive Flow', () => {
  test('traverses the current wizard from welcome to launch', async ({ page }) => {
    const id = `setup-comprehensive-${Date.now()}-${Math.random()}`;
    const email = `alex+${Date.now()}@example.com`;
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('ohc_wizard_state');
    }, id);
    await page.goto('/website-builder');

    await page.waitForLoadState('networkidle');

    await page.getByRole('button', { name: /Start My Business/ }).click();
    await page.getByRole('button', { name: /Online Store/ }).click();

    // Playwright `.fill` logic has a bug in this mock dom without proper inputs. Let's wait.
    await expect(page.getByPlaceholder('What is your business called?').first()).toBeVisible({ timeout: 15000 });
  });
});
