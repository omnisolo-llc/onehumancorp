import { test, expect } from './fixtures';

test.describe('Business Setup Wizard Comprehensive Flow', () => {
  test('traverses the current wizard from welcome to launch', async ({ page }) => {
    await page.goto('/onboarding');
    await page.waitForTimeout(1000);

    await page.getByRole('button', { name: "Or do Step-by-Step Setup" }).click();

    // Step 1: Chat 1
    await expect(page.getByRole('heading', { name: "What's the name of your business?", exact: false })).toBeVisible({ timeout: 15000 });
    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill('Alex Art');
    await page.getByRole('button', { name: "Next" }).click();

    // Step 1: Chat 2
    await expect(page.getByRole('heading', { name: "What do you sell?", exact: false })).toBeVisible({ timeout: 15000 });
    await page.getByPlaceholder("e.g. I bake custom vegan cakes for weddings and parties...").fill('alex sells art');
    await page.getByRole('button', { name: "Next" }).click();

    // Chat Step 3
    await expect(page.getByRole('heading', { name: "Where are you located?", exact: false })).toBeVisible({ timeout: 15000 });
    await page.getByPlaceholder("e.g. Portland, OR").fill("Seattle, WA");
    await page.getByRole('button', { name: "Generate My Business" }).click();

    // Step 2: Review
    await expect(page.getByRole('heading', { name: "Review Details", exact: false })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Continue/i }).click();

    // Step 3: Style
    await expect(page.getByRole('heading', { name: "Style & Team", exact: false })).toBeVisible({ timeout: 15000 });
    await page.getByText('Modern').click();

    const requestPromise = page.waitForRequest(request =>
      request.url().includes('/api/onboarding/start') && request.method() === 'POST'
    );

    await page.getByRole('button', { name: /Launch Store/i }).click();

    const request = await requestPromise;
    const postData = JSON.parse(request.postData() || '{}');

    expect(postData.business_type).not.toBe('');
    expect(postData.company_name).toBe('Alex Art');
    expect(postData.first_product_name).not.toBe('');
    expect(postData.first_product_price).not.toBe('');
    expect(postData.website_template).toBe('Modern');

    await expect(page.getByRole('heading', { name: "You're Live!", exact: false })).toBeVisible({ timeout: 15000 });
    expect(postData.domain_choice).toBe('subdomain');
  });
});
