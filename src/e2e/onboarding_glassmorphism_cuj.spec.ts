import { test, expect } from '@playwright/test';

test.describe('Next.js Onboarding Wizard CUJ (Glassmorphism & Polish)', () => {
  // Common setup for Next.js app flow
  test.beforeEach(async ({ page }) => {
    // Intercept API calls made by the Next.js frontend
    await page.route('**/api/onboarding/start', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ organization_id: 'test-org-123' })
      });
    });
  });

  // Scenario 1: Verify styling on the main container
  test('Persona: New User sees the polished glassmorphism setup wizard', async ({ page }) => {
    await page.goto('http://localhost:3000/onboarding');
    const setupContainer = page.locator('#setup-screen');
    await expect(setupContainer).toBeVisible();
    await expect(setupContainer).toHaveClass(/glassmorphism/);
    await expect(setupContainer).toHaveClass(/rounded-\[16px\]/);
  });

  // Scenario 2: Validation
  test('Persona: Business Owner is blocked if submitting empty form on step 2', async ({ page }) => {
    await page.goto('http://localhost:3000/onboarding');

    // Step 1 -> Step 2
    await page.locator('button:has-text("Storefront or Cafe")').click();

    // Attempt to proceed without entering business name (it requires at least 3 chars)
    const nextButton = page.locator('button', { hasText: 'Continue' });
    await expect(nextButton).toBeVisible();
    await nextButton.click();

    // The validation error should appear
    await expect(page.getByText('Business Name must be at least 3 characters.')).toBeVisible();
  });

  // Scenario 3: Progression
  test('Persona: Business Owner can progress through the wizard by filling valid data', async ({ page }) => {
    await page.goto('http://localhost:3000/onboarding');

    // Click category
    await page.locator('button:has-text("Storefront or Cafe")').click();

    // Fill Business Name
    const nameInput = page.getByPlaceholder('e.g. Acme Coffee');
    await expect(nameInput).toBeVisible();
    await nameInput.fill('Acme Corp Bakery');

    // Proceed to Step 3 (Assistant Tone)
    const nextButton = page.locator('button', { hasText: 'Continue' });
    await expect(nextButton).toBeVisible();
    await nextButton.click();

    // Verify we reached the AI Tone step
    await expect(page.getByText(/Tone & Personality/)).toBeVisible();
  });

  // Scenario 4: Back tracking
  test('Persona: Business Owner can track backwards through steps', async ({ page }) => {
    await page.goto('http://localhost:3000/onboarding');

    // Move forward
    await page.locator('button:has-text("Storefront or Cafe")').click();
    await page.getByPlaceholder('e.g. Acme Coffee').fill('Acme Corp Bakery');
    await page.locator('button', { hasText: 'Continue' }).click();

    // Ensure we are on Step 3
    await expect(page.getByText(/Tone & Personality/)).toBeVisible();

    // Click Back
    const backButton = page.locator('button', { hasText: 'Back' });
    await expect(backButton).toBeVisible();
    await backButton.click();

    // Ensure we are back on Step 2
    await expect(page.getByText('Tell us about your business')).toBeVisible();
  });

  // Scenario 5: Skip setup flow
  test('Persona: Advanced Owner can skip the setup wizard entirely', async ({ page }) => {
    await page.goto('http://localhost:3000/onboarding');

    const skipButton = page.locator('button', { hasText: 'Skip setup' });
    await expect(skipButton).toBeVisible();

    await skipButton.click();

    // Clicking skip triggers the start API and moves to the final terminal state (Step 6)
    // The final state shows the Shareable Link and buttons to Open Assistant
    await expect(page.getByText('Your Shareable Link')).toBeVisible();
  });
});
