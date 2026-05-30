import { test, expect } from './fixtures';

test.describe('Viral Trial Extension Growth Loop', () => {
  test('User can extend their trial by sharing on Twitter', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');

    // Wait for the page to load
    await expect(page.locator('h1', { hasText: 'Dashboard' })).toBeVisible();

    // Verify initial state
    await expect(page.locator('h2', { hasText: 'Extend Your Trial' })).toBeVisible();
    await expect(page.locator('h4', { hasText: 'Share on Twitter' })).toBeVisible();
    await expect(page.locator('button', { hasText: 'Share' })).toBeVisible();

    // Setup network intercept to verify the API call
    const requestPromise = page.waitForRequest(request =>
      request.url().includes('/api/v1/growth/trial/extend') && request.method() === 'POST'
    );

    // Get the current days left
    const daysLeftElement = page.locator('.text-5xl.font-outfit.font-bold');
    const initialDaysText = await daysLeftElement.innerText();
    const initialDays = parseInt(initialDaysText, 10);

    // Click the share button
    await page.locator('button', { hasText: 'Share' }).click();

    // Verify the API request was made
    await requestPromise;

    // Verify the UI updated (button state and days left)
    await expect(page.locator('button', { hasText: 'Shared' })).toBeVisible();
    await expect(daysLeftElement).toHaveText(String(initialDays + 7));

    // Verify Persistence (refresh and ensure the value remains)
    await page.reload();
    await expect(page.locator('h1', { hasText: 'Dashboard' })).toBeVisible();
    await expect(daysLeftElement).toHaveText(String(initialDays + 7));
  });
});
