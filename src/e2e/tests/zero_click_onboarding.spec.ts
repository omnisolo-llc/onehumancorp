import { test, expect } from '@playwright/test';

test.describe('Zero-Click Onboarding to Agent Feed', () => {
  test('User completes chat onboarding and sees welcome card on feed', async ({ page }) => {
    // 1. Navigate to the onboarding route
    await page.goto('/onboarding');

    // 2. Wait for Setup Assistant's first message to appear
    await expect(page.locator('#chat-messages')).toContainText('What do you want to build or manage today?');

    // 3. Click the predefined chip "Cake Shop" (if available in the new React page)
    // or just input simple sentence and submit to test real backend
    await page.fill('#chat-input', 'I run a mobile dog grooming service in Austin');
    await page.click('#chat-send-btn');

    // 4. Since this uses the real backend, the UI will eventually redirect to /dashboard
    // We wait for the dashboard route and UI to load.
    await page.waitForURL('**/dashboard**', { timeout: 45000 });

    // 5. Verify the onboarding_welcome action card is present in the UnifiedAgentFeed
    const welcomeCard = page.getByTestId('onboarding-welcome-card');
    await expect(welcomeCard).toBeVisible({ timeout: 15000 });
    await expect(welcomeCard).toContainText('Setup Complete');
    await expect(welcomeCard).toContainText('Welcome to OHC!');

    // 6. Verify layout meets mobile viewport requirements (375px width, no horizontal scroll)
    await page.setViewportSize({ width: 375, height: 812 });

    // Check horizontal scroll by verifying document width equals window innerWidth
    const hasHorizontalScroll = await page.evaluate(() => {
        return document.documentElement.scrollWidth > window.innerWidth;
    });
    expect(hasHorizontalScroll).toBeFalsy();

    // Verify touch target for the review storefront button is at least 44x44px
    const reviewBtn = page.getByTestId('review-storefront-btn');
    await expect(reviewBtn).toBeVisible();
    const box = await reviewBtn.boundingBox();
    expect(box?.width).toBeGreaterThanOrEqual(44);
    expect(box?.height).toBeGreaterThanOrEqual(44);
  });
});
