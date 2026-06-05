import { test, expect } from '@playwright/test';

test('builder flow completes successfully', async ({ page }) => {
  await page.route('**/api/v1/builder/generate', route => route.fulfill({
    status: 200,
    json: { pages: [{ blocks: [{ block_type: 'HeroBlock', content: { headline: 'Test' } }] }] }
  }));
  await page.route('**/api/v1/builder/publish_draft', route => route.fulfill({
    status: 200,
    json: { domain: 'test' }
  }));

  await page.goto('http://localhost:3000/builder');

  await expect(page.getByText(/What are you building today/i)).toBeVisible();
  await page.getByText('Selling Products').click();
  await expect(page.getByText(/Let's build your store/i)).toBeVisible();

  const nameInput = page.getByPlaceholder(/e.g. Acme Corp/i);
  await nameInput.fill('My Awesome Store');

  const categoryInput = page.getByPlaceholder(/e.g. Retail, Consulting, Tech/i);
  await categoryInput.fill('Retail');

  await page.getByRole('button', { name: /Next: Choose Vibe/i }).click();

  await expect(page.getByText(/Select Your Vibe/i)).toBeVisible();
  await page.getByRole('button', { name: 'Friendly' }).click();
  await page.getByRole('button', { name: /Next: Details/i }).click();

  await expect(page.getByText(/Final Details/i)).toBeVisible();
  const textarea = page.getByPlaceholder(/e.g. I run a mobile dog grooming service/i);
  await expect(textarea).toBeVisible();

  await textarea.fill('I run a friendly retail store selling amazing products');

  const buildButton = page.getByRole('button', { name: /Build Store/i });
  await buildButton.click();

  await expect(page.getByText(/Pick your draft/i)).toBeVisible({ timeout: 5000 });
  await page.getByRole('button', { name: /Customize Selected Draft/i }).click();

  await expect(page.getByText(/1-Tap Launch/i)).toBeVisible({ timeout: 5000 });

  await page.getByRole('button', { name: /1-Tap Launch/i }).click();

  await expect(page.getByText(/You're Live/i)).toBeVisible({ timeout: 5000 });
});
