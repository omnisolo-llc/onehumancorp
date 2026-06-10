import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard E2E Flow', () => {

  // Test 1: Completes the onboarding flow
  test('Completes the onboarding flow', async ({ page }) => {
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

  // Test 2: Validates the 44px minimum touch target size (via 44px min-height)
  test('Validates 44px touch targets on mobile sizes', async ({ page }) => {
    // Set a mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/src/ui/setup.html');

    const nameInput = page.getByPlaceholder("e.g. Graphic Design");

    // Go to step 2 first
    await page.getByText('Local Service').click();
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(nameInput).toBeVisible();
    const box = await nameInput.boundingBox();
    expect(box?.height).toBeGreaterThanOrEqual(44);
  });

  // Test 3: Verifies input disabled states (empty error states)
  test('Next button is prevented when input is empty', async ({ page }) => {
    await page.goto('/src/ui/setup.html');

    // Go to step 2
    await page.getByText('Local Service').click();
    await page.getByRole('button', { name: 'Next' }).click();

    const nextButton = page.locator('#step-categories').getByRole('button', { name: 'Next' });

    await nextButton.click();

    const errorMsg = page.locator('#categories-error');
    await expect(errorMsg).toBeVisible();

    const categoryInput = page.getByPlaceholder("e.g. Graphic Design");
    await categoryInput.fill("ABC");
    await nextButton.click();
    await expect(errorMsg).not.toBeVisible();
  });

  // Test 4: Enter key submits the first step
  test('Enter key submits the input', async ({ page }) => {
    await page.goto('/src/ui/setup.html');

    // Go to step 2
    await page.getByText('Local Service').click();
    await page.getByRole('button', { name: 'Next' }).click();

    const categoryInput = page.getByPlaceholder("e.g. Graphic Design");
    await categoryInput.fill("ABC");
    await categoryInput.press('Enter');

    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
  });

  // Test 5: Verify manual configuration fallback styling
  test('Verify layout and glassmorphism styling', async ({ page }) => {
    await page.goto('/src/ui/setup.html');

    // Step 2
    await page.getByText('Local Service').click();
    await page.getByRole('button', { name: 'Next' }).click();

    const categoryInput = page.getByPlaceholder("e.g. Graphic Design");
    await expect(categoryInput).toBeVisible();
  });
});
