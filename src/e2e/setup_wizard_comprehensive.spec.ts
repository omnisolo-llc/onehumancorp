import { test, expect } from './fixtures';

test.describe('Business Setup Wizard Comprehensive Flow', () => {
  test('traverses the current wizard from welcome to launch', async ({ page }) => {
    const id = `setup-comprehensive-${Date.now()}-${Math.random()}`;
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('ohc_wizard_state');
    }, id);
    await page.goto('/onboarding');

    await expect(page.getByRole('heading', { name: "Tell us about your business" })).toBeVisible();
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill('Alex Art');
    await page.getByRole('button', { name: /Next/ }).click();

    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
    await page.getByPlaceholder("e.g. I bake custom vegan cakes for weddings and parties...").fill('Original art and prints');
    await page.getByRole('button', { name: /Next/ }).click();

    await expect(page.getByRole('heading', { name: 'Where are you located?' })).toBeVisible();
    await page.getByPlaceholder("e.g. Portland, OR").fill('San Francisco, CA');
    await page.getByRole('button', { name: /Generate My Business/ }).click();

    await expect(page.getByRole('heading', { name: 'Review Details' })).toBeVisible();
    await expect(page.getByDisplayValue('Alex Art')).toBeVisible();
    await page.getByRole('button', { name: /Continue/ }).click();

    await expect(page.getByRole('heading', { name: 'Style & Team' })).toBeVisible();
    await page.getByText('Modern').click();
    await page.getByText('Sales Agent').click();
    await page.getByRole('button', { name: /Launch Store/ }).click();

    await expect(page.getByRole('heading', { name: /You're Live!/ })).toBeVisible();
    await expect(page.getByText('my-business.ohc.store')).toBeVisible();
  });
});
