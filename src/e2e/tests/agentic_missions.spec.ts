import { test, expect } from '@playwright/test';

test.describe('Agentic Missions CUJ Verification', () => {

  test('Zero-Touch Onboarding: conversational start to live storefront', async ({ page }) => {
    // 1. Start from home and go to onboarding
    await page.goto('/onboarding');

    // 2. Select Conversational Setup
    await page.click('text=Conversational Setup');

    // 3. Enter business description
    const chatInput = page.locator('#chat-input');
    await chatInput.fill('I bake custom organic cakes in Seattle');
    await page.click('#chat-send-btn');

    // 4. Verify AI response (simulated in E2E tests via process_chat fallback)
    await expect(page.locator('#chat-messages')).toContainText('Seattle');

    // 5. Complete chat and see review screen (simulated complete)
    // Note: In real E2E environment without LLM, the mock returns is_complete: true after 2nd message.
    await chatInput.fill('My name is Seattle Sweets');
    await page.click('#chat-send-btn');

    // 6. Navigate through review and account setup
    await expect(page).toHaveURL(/.*onboarding/);
    await expect(page.locator('h2')).toContainText('Review Details');

    await page.click('text=Continue');

    // 7. Approve and Go Live
    await page.fill('input[placeholder="you@example.com"]', 'test-owner@seattlesweets.com');
    await page.fill('input[placeholder="••••••••"]', 'Password123');
    await page.click('text=Approve & Go Live');

    // 8. Verify live state
    await expect(page.locator('h2')).toContainText("You're Live!");
    await expect(page.locator('text=seattle-sweets.ohc.app')).toBeVisible();
  });

  test('Agent Feed: Predictive Inventory Alert Visibility', async ({ page }) => {
    // 1. Login and go to Feed (simulated context)
    await page.goto('/feed');

    // 2. Verify existence of Predictive Restock Alert (requires seeded data in real stack)
    // For verification, we check if the component renders the feature type correctly
    const feedItem = page.locator('.agent-feed-item').first();
    await expect(feedItem).toContainText('Predictive Restock Alert');
  });
});
