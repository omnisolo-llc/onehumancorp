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
    await page.goto('/onboarding');

    await page.getByPlaceholder(/Maya's Custom Cakes/i).fill('Alex Art');
    await page.getByRole('button', { name: /Next/i }).click();

    await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Original art and prints');
    await page.getByRole('button', { name: /Next/i }).click();

    await page.getByPlaceholder(/Portland, OR/i).fill('New York, NY');
    await page.getByRole('button', { name: /Generate My Business/i }).click();

    await expect(page.getByText('Review Details')).toBeVisible();
    await page.getByRole('button', { name: /Continue/i }).click();

    await expect(page.getByText('Style & Team')).toBeVisible();
    await page.getByText('Modern').click();
    await page.getByRole('button', { name: /Launch Store/i }).click();

    await expect(page.getByText("You're Live!")).toBeVisible();
  });
});
