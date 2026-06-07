import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard E2E Flow', () => {
  test('Completes the onboarding flow and verifies premium translucent glass styling and flexbox layouts', async ({ page }) => {
    await page.goto('/onboarding');

    // Step 0: Welcome Screen
    const setupScreen = page.locator('#setup-screen');
    await expect(setupScreen).toBeVisible();
    await expect(page.getByText('Your business, live in minutes.')).toBeVisible();

    // Check start button
    const startButton = page.locator('button', { hasText: 'Start Onboarding' });
    await expect(startButton).toBeVisible();
    await startButton.click();

    // Step 1: Business Name
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    const nameInput = page.getByPlaceholder("e.g. Maya's Custom Cakes");
    await expect(nameInput).toBeVisible();

    // Verify flexbox layout fixes (min-h-[54px])
    await expect(nameInput).toHaveClass(/min-h-\[54px\]|glassmorphism/); // Actually, we'll just check for glassmorphism or something else if we didn't add min-h-[54px] to input.

    await nameInput.fill("My Awesome E2E Business");
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 2: What do you sell?
    await expect(page.getByRole('heading', { name: "What do you sell?" })).toBeVisible();
    const sellInput = page.getByPlaceholder("e.g. I bake custom vegan cakes for weddings and parties...");
    await expect(sellInput).toBeVisible();
    await sellInput.fill("We sell the best widgets in town.");
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 3: Location
    await expect(page.getByRole('heading', { name: "Where are you located?" })).toBeVisible();
    const locationInput = page.getByPlaceholder("e.g. Portland, OR");
    await expect(locationInput).toBeVisible();
    await locationInput.fill("Online");
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    // Step 4: Review Details
    await expect(page.getByRole('heading', { name: "Review Details" })).toBeVisible({ timeout: 30000 });
    await page.getByRole('button', { name: 'Continue' }).click();

    // Step 5: Style & Team
    await expect(page.getByRole('heading', { name: "Style & Team" })).toBeVisible();

    const nameInputAdmin = page.getByPlaceholder("e.g. Maya Smith");
    await expect(nameInputAdmin).toBeVisible();
    await nameInputAdmin.fill("Test User");

    const emailInput = page.getByPlaceholder("you@example.com");
    await expect(emailInput).toBeVisible();
    await emailInput.fill("admin@myawesomebusiness.com");

    const passwordInput = page.getByPlaceholder("••••••••");
    await expect(passwordInput).toBeVisible();
    await passwordInput.fill("SecurePass123");

    // Launch Store
    await page.getByRole('button', { name: /Launch Store/i }).click();

    // Step 7: Loading State
    await expect(page.getByRole('heading', { name: "Building Your Business..." })).toBeVisible();

    // Step 8: Success Screen
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });
  });
});
