import { test, expect } from './fixtures';

test.describe('Business Setup Wizard', () => {
  test.beforeEach(async ({ page }) => {
    const id = `business-setup-${Date.now()}-${Math.random()}`;
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('ohc_wizard_state');
    }, id);
    await page.goto('/onboarding');
    await expect(page.locator('#setup-screen')).toBeVisible();
  });

  test('shows the current setup welcome step', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Tell us about your business' })).toBeVisible();
    await expect(page.getByRole('button', { name: /Next/ })).toBeVisible();
  });

  test('moves through business type and name steps', async ({ page }) => {
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill('Test Company');
    await page.getByRole('button', { name: /Next/ }).click();

    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
  });

  test('completes the publish path to the checklist', async ({ page }) => {
    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill('Test Company');
    await page.getByRole('button', { name: /Next/ }).click();

    await page.getByPlaceholder("e.g. I bake custom vegan cakes for weddings and parties...").fill('Custom cookies and cakes');
    await page.getByRole('button', { name: /Next/ }).click();

    await page.getByPlaceholder("e.g. Portland, OR").fill('San Francisco, CA');
    await page.getByRole('button', { name: /Generate My Business/ }).click();

    await expect(page.getByRole('heading', { name: 'Review Details' })).toBeVisible();
    await expect(page.getByDisplayValue('Test Company')).toBeVisible();
    await page.getByRole('button', { name: /Continue/ }).click();

    await expect(page.getByRole('heading', { name: 'Style & Team' })).toBeVisible();
    await page.getByText('Modern').click();
    await page.getByRole('button', { name: /Launch Store/ }).click();

    await expect(page.getByRole('heading', { name: /You're Live!/ })).toBeVisible();
    await page.getByRole('link', { name: /Go to Dashboard/ }).click();
  });
});
