import { test, expect } from '@playwright/test';

test.describe('Zero-Click Onboarding to Agent Feed', () => {
  test('User completes chat onboarding and sees welcome card on feed', async ({ page }) => {
    // Navigate to the zero-click-builder route
    await page.goto('/zero-click-builder');

    // Wait for the single prompt input to appear
    await expect(page.locator('text=Tell me about your business...')).toBeVisible();

    // The input might be a standard text area
    const chatInput = page.getByPlaceholder(/E\.g\., I'm a dog walker/);
    await expect(chatInput).toBeVisible();

    // Type a simple sentence and find the submit button
    await chatInput.fill('I run a mobile dog grooming service in Austin');
    const submitBtn = page.getByRole('button', { name: /Generate Store/i });
    await submitBtn.click();

    // Wait for the "Building Your Business..." or final state
    await expect(page.locator('text=Your business is live!')).toBeVisible({ timeout: 15000 });

    // The Launch store button should be available
    const launchBtn = page.getByRole('button', { name: /Launch My Store/i });
    await expect(launchBtn).toBeVisible();
    await launchBtn.click();

    // Since this uses the real backend, the UI will eventually redirect to /dashboard
    await page.waitForURL('**/dashboard**', { timeout: 30000 });

    // Verify the feed renders properly after onboarding
    await expect(page.locator('text=Feed')).toBeVisible({ timeout: 10000 });

    // Verify the specific "Your storefront is ready." card is added to the feed
    await expect(page.locator('text=Your storefront is ready.')).toBeVisible();

    // Verify layout meets mobile viewport requirements (375px width, no horizontal scroll)
    await page.setViewportSize({ width: 375, height: 812 });

    // Check horizontal scroll by verifying document width equals window innerWidth
    const hasHorizontalScroll = await page.evaluate(() => {
        return document.documentElement.scrollWidth > window.innerWidth;
    });
    expect(hasHorizontalScroll).toBeFalsy();
  });
});
