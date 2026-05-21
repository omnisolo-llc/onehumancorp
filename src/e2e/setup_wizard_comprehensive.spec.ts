import { test, expect } from './fixtures';

test.describe('Business Setup Wizard Comprehensive Flow', () => {
  test('traverses the current wizard from welcome to launch', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Start My Business Next/ }).click();
    await page.getByRole('button', { name: /Creative/ }).click();
    await page.getByPlaceholder('What is your business called?').fill('Alex Art');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByLabel(/Services/).check();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByPlaceholder('What is the name of this product?').fill('Portrait Session');
    await page.getByPlaceholder('0.00').fill('120');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Both Online/ }).click();
    await page.getByPlaceholder('e.g. Maya Smith').fill('Alex Artist');
    await page.getByPlaceholder('you@email.com').fill('alex@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: 'Modern' }).click();
    await page.getByRole('button', { name: /Connect Custom Domain/ }).click();

    const requestPromise = page.waitForRequest(request =>
      request.url().includes('/api/onboarding/start') && request.method() === 'POST'
    );

    await page.getByRole('button', { name: /Publish my business/ }).click();

    const request = await requestPromise;
    const postData = JSON.parse(request.postData() || '{}');

    expect(postData.business_type).not.toBe('');
    expect(postData.company_name).toBe('Alex Art');
    expect(postData.first_product_name).toBe('Portrait Session');
    expect(postData.first_product_price).toBe('120');
    expect(postData.website_template).toBe('Modern');

    await expect(page.getByText('Your business is now live!')).toBeVisible();
  });
});
