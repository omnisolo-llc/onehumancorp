import { test, expect } from './fixtures';

test.describe('Business Setup Wizard Comprehensive Flow', () => {
  test('traverses the current wizard from welcome to launch', async ({ page }) => {
    await page.goto('/onboarding');

    // Step 1: Tell us about your business (chatStep = 1 is shown immediately)
    // Chat Step 1: Business Name
    // Using Maya to hit the mock route in next server
    await page.locator('input[placeholder="e.g. Maya\'s Custom Cakes"]').fill("Maya's Cakes");
    await page.getByRole('button', { name: 'Next' }).click();

    // Chat Step 2: What do you sell?
    await page.locator('textarea[placeholder*="I bake custom vegan cakes"]').fill('Portrait Session');
    await page.getByRole('button', { name: 'Next' }).click();

    // Chat Step 3: Where are you located?
    await page.locator('input[placeholder="e.g. Portland, OR"]').fill('New York');

    await page.getByRole('button', { name: 'Generate My Business' }).click();

    // Review Details Step
    await page.getByRole('button', { name: 'Continue' }).click();

    // Style & Team Step
    await page.getByText('Modern').click();

    const requestPromise = page.waitForRequest(request =>
      request.url().includes('/api/onboarding/start') && request.method() === 'POST'
    );

    await page.getByRole('button', { name: 'Launch Store' }).click();

    const request = await requestPromise;
    const postData = JSON.parse(request.postData() || '{}');

    expect(postData.company_name).toBe("Maya's Cakes");
    expect(postData.website_template).toBe('Modern');

    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });
  });
});
