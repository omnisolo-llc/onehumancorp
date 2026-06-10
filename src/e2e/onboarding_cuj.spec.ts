import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard CUJ', () => {

  test.beforeEach(async ({ page }) => {
    // Navigate to the setup screen. We are testing the Tauri UI directly using the Playwright testing setup.
  });

  // Test 1: Completes the onboarding flow
  test('Persona: Business Owner completes initial setup successfully', async ({ page }) => {
    await page.goto('/src/ui/setup.html');

    // Step 1: Work Context
    await expect(page.getByRole('heading', { name: "How do you work?" })).toBeVisible();
    await page.getByText('Local Service').click();
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 2: Categories
    await expect(page.getByRole('heading', { name: "What's your category?" })).toBeVisible();
    const categoryInput = page.getByPlaceholder("e.g. Graphic Design");
    await categoryInput.fill("Home Repairs");
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 3: Business Name and Tagline
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    const nameInput = page.getByPlaceholder("e.g. Maya's Bakery");
    await nameInput.fill("Bob's Fix-it Shop");
    const taglineInput = page.getByPlaceholder("Tagline (optional)");
    await taglineInput.fill("We fix it right");
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 4: Assistant Setup
    await expect(page.getByRole('heading', { name: "Set up your Assistant" })).toBeVisible();
    const assistantNameInput = page.getByPlaceholder("e.g. Jarvis");
    await assistantNameInput.fill("BobBot");
    await page.locator('#assistant-tone').selectOption('Friendly');
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 5: First Offer
    await expect(page.getByRole('heading', { name: "Your First Offer" })).toBeVisible();
    const offerInput = page.getByPlaceholder("e.g. Custom Birthday Cake");
    await offerInput.fill("Leaky Faucet Repair");

    // Click Finish Setup
    const finishBtn = page.getByRole('button', { name: 'Finish Setup' });
    await expect(finishBtn).toBeVisible();
    await finishBtn.click();

    // We should be redirected to success.html
    await expect(page).toHaveURL(/.*success.html/);
  });

  test('Persona: Business Owner fails validation on short business name', async ({ page }) => {
    await page.goto('/src/ui/setup.html');
    await page.getByText('Local Service').click();
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder("e.g. Graphic Design").fill("Home Repair");
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder("e.g. Maya's Bakery").fill("A");
    await page.getByRole('button', { name: 'Next' }).click();

    // Expect error
    await expect(page.locator('#name-error')).toBeVisible();
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
  });

  test('Persona: Business Owner cannot progress without context', async ({ page }) => {
    await page.goto('/src/ui/setup.html');
    await page.getByRole('button', { name: 'Next' }).click();

    // Expect error
    await expect(page.locator('#context-error')).toBeVisible();
    await expect(page.getByRole('heading', { name: "How do you work?" })).toBeVisible();
  });
});
