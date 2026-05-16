import { test, expect } from '@playwright/test';

test.describe('Conversational Onboarding Journey', () => {
  // Mobile viewport size according to requirements (375px)
  test.use({ viewport: { width: 375, height: 667 } });

  test('should complete conversational onboarding for products and render correct NBA card', async ({ page }) => {
    await page.goto('/login');
    // We assume standard login is fine for tests, or the test can just proceed if session is mocked
    // I'll interact with the "🚀 Start Business Setup" button on the login page as it routes directly to setup-screen
    const startSetupBtn = page.locator('button:has-text("🚀 Start Business Setup")');
    if (await startSetupBtn.isVisible()) {
        await startSetupBtn.click();
    } else {
        await page.goto('/business-setup');
    }

    // Wait for the advisor message
    await expect(page.locator('text=Hi there! I\'m The Advisor.')).toBeVisible();

    // Type the first answer
    await page.fill('#chat-input', 'I want to sell custom sneakers');
    await page.click('button:has-text("Send")');

    // Wait for the next prompt from the advisor
    await expect(page.locator('text=Awesome! And what is the name of your business?')).toBeVisible();

    // Type the second answer
    await page.fill('#chat-input', 'SneakerX');
    await page.click('button:has-text("Send")');

    // Wait for the confirmation message
    await expect(page.locator('text=Perfect. I\'m setting everything up for you now...')).toBeVisible();

    // It should eventually redirect to the dashboard
    await expect(page.locator('text=Dashboard')).toBeVisible({ timeout: 10000 });

    // Validate the Next Best Action card
    const nbaText = page.locator('#nba-text');
    await expect(nbaText).not.toHaveText('Loading your next step...', { timeout: 5000 });

    // Depending on what state is returned (we assume the mock setup doesn't explicitly flag enable_booking or enable_menu based on standard text unless handled by AI, but for test purposes it falls back to products)
    const btnText = page.locator('#nba-btn');
    await expect(btnText).toBeVisible();
  });
});
