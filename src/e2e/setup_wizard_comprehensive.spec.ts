import { test, expect } from './fixtures';

test.describe('Business Setup Wizard Comprehensive Flow', () => {
  test('traverses the current wizard from welcome to launch', async ({ page }) => {
    // Intercept intake and start API calls
    await page.route('**/api/onboarding/intake', async route => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ business_name: "Alex Art", business_type: "Creative", categories: ["services", "physical"], initial_products: [{ name: "Portrait Session", price: "120.00" }] }) }));
    await page.route('**/api/onboarding/start', async route => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ message: "Your business has been successfully launched.", organization_id: "org_123" }) }));

    await page.goto('/website-builder');

    // Step 1
    await page.getByPlaceholder('e.g. I bake custom vegan cakes').fill('I do art portraits.');
    await page.getByRole('button', { name: /Generate Storefront/ }).click();

    // Step 2
    await expect(page.getByRole('heading', { name: 'Review Details' })).toBeVisible();
    await page.getByRole('button', { name: /Continue/ }).click();

    // Step 3
    await expect(page.getByRole('heading', { name: 'Style & Team' })).toBeVisible();

    const requestPromise = page.waitForRequest(request =>
      request.url().includes('/api/onboarding/start') && request.method() === 'POST'
    );

    await page.getByRole('button', { name: /Launch Store/ }).click();

    const request = await requestPromise;
    const postData = JSON.parse(request.postData() || '{}');

    expect(postData.business_type).toBe('Creative');
    expect(postData.company_name).toBe('Alex Art');
    expect(postData.first_product_name).toBe('Portrait Session');
    expect(postData.first_product_price).toBe('120.00');
    expect(postData.website_template).toBe('Modern');

    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });
  });
});
