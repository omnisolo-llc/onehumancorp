import { test, expect } from './fixtures';

test.describe('Instant Build Flow', () => {
  test('launches business directly using AI flow', async ({ page }) => {
    await page.goto('/website-builder');

    // Click Instant Build on setup screen
    await page.getByRole('button', { name: /Instant Build/ }).click();

    // Describe business and generate
    await expect(page.getByRole('heading', { name: 'Describe your business in a sentence' })).toBeVisible();
    await page.getByPlaceholder('e.g. I run a local bakery called Maya\'s Cakes...').fill('I sell custom handmade jewelry online');

    // Mock the backend requests that generateAI() makes
    await page.route('/api/v1/builder/generate', async route => {
      await route.fulfill({ json: { pages: [{ blocks: [] }] } });
    });

    const requestPromise = page.waitForRequest(request =>
      request.url().includes('/api/onboarding/start') && request.method() === 'POST'
    );

    await page.getByRole('button', { name: /Generate Storefront/ }).click();

    // It should hit the onboarding start endpoint with extrapolated data
    const request = await requestPromise;
    const postData = JSON.parse(request.postData() || '{}');
    expect(postData.company_description).toBe('I sell custom handmade jewelry online');
    expect(postData.business_type).toBe('AI Generated');
    expect(postData.selling_categories).toContain('digital');
    expect(postData.payment_pref).toBe('online');

    // It should land on the success screen (step-100)
    await expect(page.getByRole('heading', { name: /Success! Your business is live/ })).toBeVisible();
  });
});
