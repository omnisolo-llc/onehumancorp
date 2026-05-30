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

    // Step 1: chatStep 1
    const businessNameInput = page.locator('input[placeholder="e.g. Maya\'s Custom Cakes"]');
    await expect(businessNameInput).toBeVisible();
    await businessNameInput.fill('Alex Art');
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 1: chatStep 2
    const sellInput = page.locator('textarea[placeholder="e.g. I bake custom vegan cakes for weddings and parties..."]');
    await expect(sellInput).toBeVisible();
    await sellInput.fill('Original art and prints');
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 1: chatStep 3
    const locationInput = page.locator('input[placeholder="e.g. Portland, OR"]');
    await expect(locationInput).toBeVisible();
    await locationInput.fill('Portland, OR');

    // Submit details and generate business
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    // Step 2: Review Details
    const continueBtn = page.getByRole('button', { name: 'Continue' });
    await expect(continueBtn).toBeVisible({ timeout: 10000 });
    await continueBtn.click();

    // Step 3: Style & Team
    const modernTemplate = page.locator('div').filter({ hasText: /^Modern$/ });
    await expect(modernTemplate).toBeVisible();
    await modernTemplate.click();

    const launchBtn = page.getByRole('button', { name: 'Launch Store' });
    await expect(launchBtn).toBeVisible();
    await launchBtn.click();

    // Step 5: You're Live!
    await expect(page.getByRole('heading', { name: /You're Live!/ })).toBeVisible();
    await expect(page.locator('a:has-text("Go to Dashboard")')).toBeVisible();
  });
});
