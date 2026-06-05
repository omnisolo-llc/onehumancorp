import { test, expect } from './fixtures';

test.describe('Business Setup Wizard', () => {
  test.beforeEach(async ({ page }) => {
    const id = `business-setup-${Date.now()}-${Math.random()}`;
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('ohc_wizard_state');
      localStorage.removeItem('onboarding-storage-v3');
    }, id);
    await page.goto('/onboarding');
    await page.waitForLoadState('networkidle');
  });

  test('shows the current setup welcome step', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await expect(page.getByRole('heading', { name: 'Tell us about your business' })).toBeVisible();
    await expect(page.getByRole('button', { name: /Start Onboarding/ })).toBeVisible();
  });

  test('moves through business name and type steps', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.getByRole('button', { name: /Start Onboarding/ }).click();
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();

    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill('Maya Bakery');
    await page.getByRole('button', { name: /Next/ }).click();
    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
    await page.getByPlaceholder("e.g. I bake custom vegan cakes for birthdays").fill('Cakes');
    await page.getByRole('button', { name: /Next/ }).click();
    await expect(page.getByRole('heading', { name: 'Where are you located?' })).toBeVisible();
  });

  test('completes the publish path to the checklist', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    const email = `maya+${Date.now()}@example.com`;

    await page.getByRole('button', { name: /Start Onboarding/ }).click();
    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill('Maya Bakery');
    await page.getByRole('button', { name: /Next/ }).click();

    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
    await page.getByPlaceholder("e.g. I bake custom vegan cakes for birthdays").fill('Cakes');
    await page.getByRole('button', { name: /Next/ }).click();

    await expect(page.getByRole('heading', { name: 'Where are you located?' })).toBeVisible();
    await page.getByPlaceholder("e.g. Portland, OR").fill('NY');
    await page.getByRole('button', { name: /Generate My Business/ }).click();

    // Verify Review Step
    await expect(page.getByRole('heading', { name: 'Review Your Setup' })).toBeVisible({ timeout: 15000 });
    await expect(page.getByDisplayValue('Maya Bakery')).toBeVisible();

    // We expect MiniMax or fallback to generate the values correctly
    await page.getByRole('button', { name: /Next/ }).click();

    // Style & Team Step
    await expect(page.getByRole('heading', { name: 'Style & Team' })).toBeVisible();
    await page.getByText('Modern').click();

    // Fill in Account Setup fields
    const nameInput2 = page.getByPlaceholder(/e.g. Maya Smith/i);
    await nameInput2.fill('Maya Smith');

    const emailInput = page.getByPlaceholder(/you@example.com/i);
    await emailInput.fill('maya@example.com');

    const passwordInput = page.getByPlaceholder(/••••••••/i);
    await passwordInput.fill('mypassword123');

    // Submit
    const launchButton = page.getByRole('button', { name: /Launch Store/i });
    await launchButton.click();

    // Wait for "You're Live"
    await expect(page.getByRole('heading', { name: /You're Live!/ })).toBeVisible({ timeout: 15000 });
  });
});
