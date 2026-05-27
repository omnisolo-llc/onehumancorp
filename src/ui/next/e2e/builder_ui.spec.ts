import { test, expect } from '@playwright/test';

test('builder flow completes successfully', async ({ page }) => {
  await page.goto('http://localhost:3000/builder');

  await page.waitForLoadState('networkidle');

  await page.route('**/api/onboarding/state', route => route.fulfill({ status: 200, json: {} }));

  await page.route('**/api/v1/builder/generate', route => route.fulfill({
    status: 200,
    json: { pages: [{ blocks: [{ block_type: 'HeroBlock', content: { headline: 'Test' } }] }] }
  }));
  await page.route('**/api/v1/builder/publish_draft', route => route.fulfill({
    status: 200,
    json: { domain: 'my-awesome-store' }
  }));

  await expect(page.getByText('Selling Products')).toBeVisible({ timeout: 10000 });
  await page.getByText('Selling Products').click();

  await expect(page.getByText(/Let's build your store/i)).toBeVisible({ timeout: 10000 });

  const inputs = page.getByRole('textbox');
  await inputs.nth(0).fill('My Awesome Store');
  await inputs.nth(1).fill('Retail');

  await page.getByRole('button', { name: /Next: Choose Vibe/i }).click();

  await expect(page.getByText(/Select Your Vibe/i)).toBeVisible();
  await page.getByRole('button', { name: 'Friendly' }).click();
  await page.getByRole('button', { name: /Next: Details/i }).click();

  await expect(page.getByText(/Final Details/i)).toBeVisible();
  const textarea = page.getByRole('textbox').last();
  await expect(textarea).toBeVisible();

  await textarea.fill('I run a friendly retail store selling amazing products');

  const buildButton = page.getByRole('button', { name: /Build Store/i });
  await buildButton.click();

  await expect(page.getByText(/Designing your custom storefront/i)).toBeVisible();

  // "Pick your draft" selection screen
  await expect(page.getByText(/Pick your draft/i)).toBeVisible({ timeout: 15000 });

  // Click the first Draft Preview card
  await page.getByText(/Draft 1/i).click();

  // Click Customize Selected Draft
  await page.getByText(/Customize Selected Draft/i).click();

  // Draft Preview Screen (Launch Live is visible when editor opens)
  await expect(page.getByText(/1-Tap Launch/i)).toBeVisible({ timeout: 10000 });

  await page.getByText(/1-Tap Launch/i).click();

  // Launch Screen
  await expect(page.getByText(/You're Live/i)).toBeVisible({ timeout: 5000 });
});
