import { test, expect } from '@playwright/test';

test('builder flow completes successfully', async ({ page }) => {

  // Mock API responses for E2E tests
  await page.route('/api/onboarding/intake', async route => {
    await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ business_name: 'Dog Grooming', business_type: 'Service', categories: ['physical'], initial_products: [{name: 'Grooming', price: '50'}] }) });
  });
  await page.route('/api/onboarding/start', async route => {
    await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({}) });
  });
  await page.route('/api/v1/builder/generate', async route => {
    // Delay to ensure the 'generating' screen stays visible for playwright to assert
    await new Promise(r => setTimeout(r, 1500));
    await route.fulfill({
      status: 200, contentType: 'application/json',
      body: JSON.stringify({
        pages: [{
          blocks: [
            { block_type: 'HeroBlock', content: { headline: 'Welcome', copy: 'Best grooming', image: 'test.jpg' } },
            { block_type: 'ProductGridBlock', content: { items: [{name: 'Grooming', price: '$50', description: 'desc'}] } }
          ]
        }]
      })
    });
  });
  await page.route('/api/v1/builder/publish_draft', async route => {
    await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ domain: 'myshop' }) });
  });

  const url = process.env.BASE_URL ? `${process.env.BASE_URL}/builder` : 'http://127.0.0.1:3000/builder';
  await page.goto(url);

  // 1. Onboarding Screen
  const textarea = page.getByPlaceholder(/e.g. I run a mobile dog grooming service/i);
  await expect(textarea).toBeVisible();

  // Fill the bio
  await textarea.fill('I run a mobile dog grooming service');
  await textarea.press('Tab');
  await page.waitForTimeout(500);

  // Click Generate
  const buildButton = page.getByRole('button', { name: /Build My Storefront/i });
  await buildButton.click();

  // 2. Generating Screen
  await expect(page.getByText(/The Promoter is picking colors/i)).toBeVisible();

  // 3. Draft Preview Screen
  await expect(page.getByText(/Preview Mode/i)).toBeVisible({ timeout: 5000 });
  await expect(page.getByText(/1-Tap Launch/i)).toBeVisible();

  // 4. Click Launch
  await page.getByRole('button', { name: /1-Tap Launch/i }).click();

  // 5. Launch Screen
  await expect(page.getByText(/You're Live/i)).toBeVisible({ timeout: 5000 });
});
