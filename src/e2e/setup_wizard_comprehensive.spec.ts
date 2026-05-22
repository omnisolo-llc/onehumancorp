import { test, expect } from './fixtures';

test.describe('Business Setup Wizard Comprehensive Flow', () => {
  test('traverses the current wizard from welcome to launch', async ({ page }) => {
    await page.goto('/onboarding');

    // Step 1: Business Name
    await page.getByPlaceholder("e.g. Maya's Cakes").fill('Alex Art');
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 2: What do you sell?
    await page.getByPlaceholder("e.g. I bake custom wedding cakes and cupcakes.").fill('Portrait Session');
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 3: Preferred style
    await page.getByPlaceholder("e.g. Clean and modern with pastel colors").fill('Modern');
    await page.getByRole('button', { name: 'Generate Draft' }).click();

    // Step 4: AI Draft Preview
    await expect(page.getByText('Looks Great!')).toBeVisible({ timeout: 15000 });

    const requestPromise = page.waitForRequest(request =>
      request.url().includes('/api/onboarding/start') && request.method() === 'POST'
    );

    await page.getByRole('button', { name: 'Publish Now' }).click();

    const request = await requestPromise;
    const postData = JSON.parse(request.postData() || '{}');

    // Expected properties based on the new onboarding UI logic
    expect(postData.company_description).toBe('Modern');
    expect(postData.website_template).toBe('modern');

    // Step 5: Success Screen
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });
  });
});
