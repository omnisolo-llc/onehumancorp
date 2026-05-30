import { test, expect } from './fixtures';

test.describe('Business Setup Wizard Magic Flow', () => {
  test('uses one-tap setup to launch a business instantly', async ({ page }) => {
    await page.goto('/onboarding');
    await page.waitForTimeout(1000);

    await expect(page.getByRole('heading', { name: "One-Tap Business Setup", exact: false })).toBeVisible({ timeout: 15000 });

    await page.getByPlaceholder("e.g. Handyman in Chicago").fill("Handyman in Chicago");

    const requestPromise = page.waitForRequest(request =>
      request.url().includes('/api/onboarding/magic') && request.method() === 'POST'
    );

    await page.getByRole('button', { name: "Magic Setup" }).click();

    const request = await requestPromise;
    const postData = JSON.parse(request.postData() || '{}');
    expect(postData.description).toBe('Handyman in Chicago');

    await expect(page.getByRole('heading', { name: "You're Live!", exact: false })).toBeVisible({ timeout: 15000 });
  });
});
