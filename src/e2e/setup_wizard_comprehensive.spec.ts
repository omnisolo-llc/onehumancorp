import { test, expect } from './fixtures';

test.describe('Business Setup Wizard Comprehensive Flow', () => {
  test('traverses the current wizard from welcome to launch', async ({ page }) => {
    const id = `setup-comprehensive-${Date.now()}-${Math.random()}`;
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('onboarding-storage-v3');
    }, id);
    await page.goto('/onboarding');

    // Step 1: Chat flow
    await expect(page.getByRole('heading', { name: /Tell us about your business/ })).toBeVisible();

    // Chat step 1: Business name
    await expect(page.getByRole('heading', { name: /What's the name of your business\?/ })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill('Alex Art');
    await page.getByRole('button', { name: 'Next' }).click();

    // Chat step 2: What do you sell
    await expect(page.getByRole('heading', { name: /What do you sell\?/ })).toBeVisible();
    await page.getByPlaceholder("e.g. I bake custom vegan cakes for weddings and parties...").fill('Original art and prints');
    await page.getByRole('button', { name: 'Next' }).click();

    // Chat step 3: Location
    await expect(page.getByRole('heading', { name: /Where are you located\?/ })).toBeVisible();
    await page.getByPlaceholder("e.g. Portland, OR").fill('San Francisco, CA');

    // Trigger intake API
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    // Step 2: Review Details
    await expect(page.getByRole('heading', { name: 'Review Details' })).toBeVisible({ timeout: 15000 });

    // Check elements dynamically without nth by finding label siblings
    await page.locator('input[value="Alex Art"]').waitFor({ state: 'attached' }); // from previous step

    await page.getByRole('combobox').selectOption('Physical Products');

    const categoriesDiv = page.locator('div').filter({ hasText: /^Categories \(Comma separated\)$/ });
    await categoriesDiv.locator('input').fill('Art');

    const productDiv = page.locator('div').filter({ hasText: /^First Product$/ });
    await productDiv.locator('input').fill('Custom Print');

    const priceDiv = page.locator('div').filter({ hasText: /^Price$/ });
    await priceDiv.locator('input').fill('49.00');

    await page.getByRole('button', { name: 'Continue' }).click();

    // Step 3: Style & Team
    await expect(page.getByRole('heading', { name: 'Style & Team' })).toBeVisible();

    // Select agents
    await page.getByText('Sales Agent').click();
    await page.getByText('Marketing Agent').click();

    // Launch!
    await page.getByRole('button', { name: 'Launch Store' }).click();

    // Step 4: Loading
    await expect(page.getByRole('heading', { name: 'Building Your Business...' })).toBeVisible();

    // Step 5: Live
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });
    await expect(page.getByRole('link', { name: 'Go to Dashboard' })).toBeVisible();
  });
});
