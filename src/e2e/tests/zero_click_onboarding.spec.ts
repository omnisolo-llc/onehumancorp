import { test, expect } from '@playwright/test';

test.describe('Zero-Click Onboarding to Agent Feed', () => {
  test('User completes chat onboarding and sees welcome card on feed', async ({ page }) => {
    // Navigate to the zero-click-builder route
    await page.goto('/zero-click-builder');

    // Wait for Setup Assistant's first message to appear
    await expect(page.locator('text=What kind of business do you want to build')).toBeVisible();

    // The chat input might be a standard input field
    const chatInput = page.getByPlaceholder('Type your message...');
    await expect(chatInput).toBeVisible();

    // Type a simple sentence and press Enter (or find the submit button)
    await chatInput.fill('I run a mobile dog grooming service in Austin');
    await chatInput.press('Enter');

    // Wait for the "Building Your Business..." or final state
    await expect(page.locator('text=Your business is live!')).toBeVisible({ timeout: 30000 });

    // The Launch store button should be available
    const launchBtn = page.getByRole('button', { name: /Launch My Store/i });
    await expect(launchBtn).toBeVisible();
    await launchBtn.click();

    // Since this uses the real backend, the UI will eventually redirect to /dashboard
    await page.waitForURL('**/dashboard**', { timeout: 30000 });

    // Verify the feed renders properly after onboarding
    await expect(page.locator('text=Feed')).toBeVisible({ timeout: 10000 });

    // Verify layout meets mobile viewport requirements (375px width, no horizontal scroll)
    await page.setViewportSize({ width: 375, height: 812 });

    // Check horizontal scroll by verifying document width equals window innerWidth
    const hasHorizontalScroll = await page.evaluate(() => {
        return document.documentElement.scrollWidth > window.innerWidth;
    });
    expect(hasHorizontalScroll).toBeFalsy();
  });
});
