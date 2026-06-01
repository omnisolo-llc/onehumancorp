import { test, expect } from '@playwright/test';
// NOTE: We rely on the seeded test environment, doing our best to perform an unmocked E2E operation

test.describe('Onboarding Wizard CUJ', () => {

  // Test 1: Persona navigates from home, starts onboarding
  test('Persona: Business Owner completes initial setup successfully', async ({ page }) => {
    // 1. Owner starts from the home page after user login via the UI
    await page.goto('/login');
    // We assume the test framework has setup or we just login
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();

    // Now on home page, click to start onboarding
    await expect(page.getByRole('heading', { name: /Welcome/i })).toBeVisible({ timeout: 15000 });
    await page.getByRole('link', { name: /Start Onboarding/i }).click();

    // Verify it landed on the Onboarding page
    await expect(page.getByText('Tell us about your business')).toBeVisible();

    // 2. Owner enters business name
    const nameInput = page.getByPlaceholder(/e.g. Maya's Custom Cakes/i);
    await nameInput.fill('Maya Bakery');
    await page.getByRole('button', { name: /Next/i }).click();

    // 3. Owner enters what they sell
    const sellInput = page.getByPlaceholder(/I bake custom vegan cakes/i);
    await sellInput.fill('Cakes');
    await page.getByRole('button', { name: /Next/i }).click();

    // 4. Owner enters location
    const locInput = page.getByPlaceholder(/Portland, OR/i);
    await locInput.fill('NY');

    const generateBtn = page.getByRole('button', { name: /Generate My Business/i });
    await generateBtn.click();

    // 5. Verify it transitions to Live Screen (bypasses Step 2 and 3)
    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 30000 });
    await expect(page.getByRole('link', { name: /Go to Dashboard/i })).toBeVisible();
  });

  // Test 3: Validate missing location blocks progression
  test('Persona: Business Owner cannot progress without location', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();
    await page.getByRole('link', { name: /Start Onboarding/i }).click();

    const nameInput = page.getByPlaceholder(/e.g. Maya's Custom Cakes/i);
    await nameInput.fill('Maya Bakery');
    await page.getByRole('button', { name: /Next/i }).click();

    const sellInput = page.getByPlaceholder(/I bake custom vegan cakes/i);
    await sellInput.fill('Cakes');
    await page.getByRole('button', { name: /Next/i }).click();

    // Keep location empty
    const generateBtn = page.getByRole('button', { name: /Generate My Business/i });
    await expect(generateBtn).toBeDisabled();
  });

  // Test 4: Navigating Back works
  test('Persona: Business Owner can navigate back from sell step', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();
    await page.getByRole('link', { name: /Start Onboarding/i }).click();

    const nameInput = page.getByPlaceholder(/e.g. Maya's Custom Cakes/i);
    await nameInput.fill('Maya Bakery');
    await page.getByRole('button', { name: /Next/i }).click();
    await expect(page.getByText('What do you sell?')).toBeVisible();

    const backBtn = page.getByRole('button', { name: /Back/i });
    await backBtn.click();
    await expect(page.getByText("What's the name of your business?")).toBeVisible();
  });
});
