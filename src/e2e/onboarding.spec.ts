import { test, expect } from '@playwright/test';

test.describe('Autonomous Agentic Onboarding Flow', () => {
  test('User completes conversational onboarding successfully', async ({ page }) => {
    // We mock the backend calls for E2E speed if needed, or run against the real backend
    // but the backend uses a local mock when no LLM is configured.
    await page.goto('/onboarding');

    await expect(page.locator('h1').filter({ hasText: 'OHC Setup Agent' })).toBeVisible();
    await expect(page.locator('text=Hi! Let\'s get your business online. What do you sell?')).toBeVisible();

    const inputLocator = page.locator('input[type="text"]');
    await inputLocator.fill('I make custom vegan cakes.');
    await page.keyboard.press('Enter');
    await expect(page.locator('text=I make custom vegan cakes.')).toBeVisible();

    // The backend's fallback mock responds with "Great! Could you provide..." if messages.len() <= 1
    // The previous state was length 0 user messages, now 1.
    // Wait for the first response
    await expect(page.locator('text=Great! Could you provide an example photo or a little more detail about what you sell?')).toBeVisible({ timeout: 15000 });

    // Send second message to trigger the 'complete' state in the mock
    await inputLocator.fill('They are mostly chocolate cakes.');
    await page.keyboard.press('Enter');

    // Wait for the credentials form to appear
    await expect(page.locator('h3').filter({ hasText: 'Create Owner Account' })).toBeVisible({ timeout: 15000 });

    // Fill in credentials
    await page.locator('input[placeholder="Your Name"]').fill('Test User');
    await page.locator('input[placeholder="Email Address"]').fill('testuser@example.com');
    await page.locator('input[placeholder="Password (8+ chars, 1 number)"]').fill('password123!');

    // Submit
    await page.click('button:has-text("Create Business")');

    // The frontend then kicks off the /start call and shows the 'Go to Owner Dashboard' button
    await expect(page.locator('text=Go to Owner Dashboard')).toBeVisible({ timeout: 15000 });

    // Click dashboard button
    await page.click('text=Go to Owner Dashboard');

    // Verify URL change
    await expect(page).toHaveURL(/.*\/dashboard/);
  });
});
