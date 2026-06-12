import { test, expect } from '@playwright/test';

test.describe('Zero-Click Onboarding E2E Flow', () => {
  test('A new user initiates and completes zero-click onboarding via chat', async ({ page }) => {
    await page.goto('/onboarding');

    const setupScreen = page.locator('#setup-screen');
    await expect(setupScreen).toBeVisible({ timeout: 30000 });

    const chatButton = page.locator('button', { hasText: 'Chat Setup' });
    await expect(chatButton).toBeVisible();
    await chatButton.click();

    await expect(page.getByRole('heading', { name: "Hi! I'm your OHC assistant." })).toBeVisible();

    const chatInput = page.getByPlaceholder("What's the name of your business...");
    await expect(chatInput).toBeVisible();
    await chatInput.fill("I'm Maya and I run a custom cake shop.");
    await chatInput.press('Enter');

    // Simulate magic moment
    const loadingState = page.getByText(/Analyzing photos|Drafting product descriptions/i).first();
    await expect(loadingState).toBeVisible({ timeout: 10000 });

    // Success Screen
    const successHeading = page.getByRole('heading', { name: "You're Live!" });
    await expect(successHeading).toBeVisible({ timeout: 30000 });
  });
});
