import { test, expect } from './fixtures';

test.describe('Leo Onboarding Flow (Music Tutor)', () => {
  test('Completes the Idea to Live setup', async ({ page }) => {
    const id = `leo-onboarding-${Date.now()}`;
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('ohc_wizard_state');
    }, id);

    await page.goto('/onboarding');
    await expect(page.getByRole('heading', { name: 'Tell us about your business' })).toBeVisible();

    await page.getByPlaceholder(/e.g. Maya's Custom Cakes/i).fill('Leo Guitar Lessons');
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
    await page.getByPlaceholder(/e.g. I bake custom vegan cakes/i).fill('Online and in-person guitar lessons');
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: 'Where are you located?' })).toBeVisible();
    await page.getByPlaceholder(/e.g. Portland, OR/i).fill('Seattle, WA');
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    // Review step
    await expect(page.getByRole('heading', { name: 'Review Details' })).toBeVisible();
    await page.getByRole('button', { name: 'Continue' }).click();

    // Style step
    await expect(page.getByRole('heading', { name: 'Style & Team' })).toBeVisible();
    await page.getByText('Minimal').click();
    await page.getByRole('button', { name: 'Launch Store' }).click();

    // Success step
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible();
  });
});
