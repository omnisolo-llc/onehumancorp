import { test, expect } from './fixtures';

test.describe('Business Setup Wizard Comprehensive Flow', () => {
  test('traverses the current wizard from welcome to launch', async ({ page }) => {
    await page.goto('/website-builder');

    await expect(page.getByRole('heading', { name: 'Describe your business in a sentence' })).toBeVisible({ timeout: 15000 });
    await page.getByPlaceholder(/e\.g\. I run a local bakery called Maya's Cakes\.\.\./).fill("Alex creates custom art in Seattle.");

    const requestPromise = page.waitForRequest(request =>
      request.url().includes('/api/onboarding/start') && request.method() === 'POST'
    );

    await page.getByRole('button', { name: /Launch your business in 10 minutes/ }).click();

    const request = await requestPromise;
    const postData = JSON.parse(request.postData() || '{}');

    expect(postData.business_type).not.toBe('');
    expect(postData.company_name).toBe('Alex');

    await expect(page.getByRole('heading', { name: 'Edit Website' })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Publish Changes/ }).click();

    await expect(page.getByText("You're set up! Here's what to do next:")).toBeVisible({ timeout: 15000 });
  });
});
