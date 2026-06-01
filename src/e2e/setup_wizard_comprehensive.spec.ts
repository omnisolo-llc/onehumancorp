import { test, expect } from './fixtures';

/**
 * Persona: Maya - The Home Baker (28, non-technical)
 * Concept: Maya bakes custom vegan cakes and needs a simple storefront.
 * Operating Plan: Use the progressive AI interview flow to answer 3 quick questions about her bakery.
 * CUJ:
 *   1. Enter business name ("Maya's Bakery")
 *   2. Enter what she sells ("Vegan custom cakes for weddings")
 *   3. Enter location ("Seattle, WA") and generate business.
 *   4. Review AI-generated details (Business name, categories, product name, price) and continue.
 *   5. Select a "Minimal" website template and assign "Sales Agent".
 *   6. Launch store and confirm the "You're Live!" success page displays the new domain.
 */
test.describe('Business Setup Wizard Comprehensive Flow (Progressive AI Interview)', () => {
  test('traverses the conversational onboarding from welcome to live launch', async ({ page }) => {
    const id = `setup-comprehensive-${Date.now()}-${Math.random()}`;
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('onboarding-storage-v3');
      localStorage.removeItem('ohc_wizard_state');
    }, id);

    await page.goto('/onboarding');

    // Step 1: Chat Step 1 - Business Name
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill("Maya's Bakery");
    await page.getByRole('button', { name: /Next/ }).click();

    // Step 1: Chat Step 2 - What you sell
    await expect(page.getByRole('heading', { name: "What do you sell?" })).toBeVisible();
    await page.getByPlaceholder("e.g. I bake custom vegan cakes for weddings and parties...").fill("Vegan custom cakes for weddings");
    await page.getByRole('button', { name: /Next/ }).click();

    // Step 1: Chat Step 3 - Location
    await expect(page.getByRole('heading', { name: "Where are you located?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Portland, OR").fill("Seattle, WA");
    await page.getByRole('button', { name: /Generate My Business/ }).click();

    // Step 2: Review Details
    // API mock or backend call will transition state, wait for Review Details to load
    await expect(page.getByRole('heading', { name: "Review Details" })).toBeVisible();
    await expect(page.getByDisplayValue("Maya's Bakery")).toBeVisible();
    await page.getByRole('button', { name: /Continue/ }).click();

    // Step 3: Style & Team
    await expect(page.getByRole('heading', { name: "Style & Team" })).toBeVisible();
    await page.getByText('Minimal').click();
    await page.getByText('Sales Agent').click();

    // Auto-respond toggle is checked by default, so we can just leave it
    await page.getByRole('button', { name: /Launch Store/ }).click();

    // Step 4 & 5: Launching and Live Page
    await expect(page.getByRole('heading', { name: "Building Your Business..." })).toBeVisible();
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible();
    await expect(page.getByText('my-business.ohc.store')).toBeVisible();
    await expect(page.getByRole('link', { name: /Go to Dashboard/ })).toBeVisible();
  });
});
