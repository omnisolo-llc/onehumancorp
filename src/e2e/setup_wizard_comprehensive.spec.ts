import { test, expect } from './fixtures';

test.describe('Business Setup Wizard Comprehensive Flow', () => {
  test('traverses the current wizard from welcome to launch', async ({ page }) => {
    await page.goto('/onboarding');

    // Step 1 - Chat 1
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill("Alex Art maya");
    await page.getByRole('button', { name: 'Next', exact: true }).click();

    // Step 1 - Chat 2
    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
    await page.getByPlaceholder('e.g. I bake custom vegan cakes for weddings and parties...').fill('Portrait Sessions');
    await page.getByRole('button', { name: 'Next', exact: true }).click();

    // Step 1 - Chat 3
    await expect(page.getByRole('heading', { name: 'Where are you located?' })).toBeVisible();
    await page.getByPlaceholder('e.g. Portland, OR').fill('New York, NY');

    // Click Generate
    await page.getByRole('button', { name: /Generate My Business/i }).click();

    // Step 2 - Review
    await expect(page.getByRole('heading', { name: "Review Details" })).toBeVisible({ timeout: 15000 });
    // Wait for network mock to be applied
    // The backend mock in intake returns "Maya's Cakes" or "Carlos Plumbing" depending on what we typed.
    // Wait for it to become visible and have value.
    await expect(page.locator('input').first()).toHaveValue("Maya's Cakes", { timeout: 15000 });
    await page.getByRole('button', { name: /Continue/i }).click();

    // Step 3 - Style
    await expect(page.getByRole('heading', { name: "Style & Team" })).toBeVisible();
    await page.locator('div').filter({ hasText: /^Modern$/ }).first().click();

    const requestPromise = page.waitForRequest(request =>
      request.url().includes('/api/onboarding/start') && request.method() === 'POST'
    );

    await page.getByRole('button', { name: /Launch Store/i }).click();

    const request = await requestPromise;
    const postData = JSON.parse(request.postData() || '{}');

    expect(postData.business_type).toBe('Bakery');
    expect(postData.company_name).toBe("Maya's Cakes");
    expect(postData.first_product_name).toBe('Custom Vegan Cake');
    expect(postData.website_template).toBe('Modern');

    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });
  });
});
