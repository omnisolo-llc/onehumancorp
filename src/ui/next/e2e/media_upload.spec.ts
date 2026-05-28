import { test, expect } from '@playwright/test';

test('media upload background processing works smoothly', async ({ page }) => {
  // First complete the builder flow up to generating a storefront to ensure blocks are created
  await page.route('**/api/v1/builder/generate', route => route.fulfill({
    status: 200,
    json: { pages: [{ blocks: [{ block_type: 'HeroBlock', content: { headline: 'My Test Store' } }] }] }
  }));

  await page.route('**/api/v1/media/upload', async route => {
    return route.fulfill({
      status: 200,
      json: {
        status: "processing",
        url: "https://cdn.ohc.store/optimized/test-image.webp",
        message: "Media upload accepted. Processing in background."
      }
    });
  });

  await page.goto('http://localhost:3000/builder');

  // Go through builder flow
  await expect(page.getByText(/What are you building today/i)).toBeVisible();
  await page.getByText('Selling Products').click();

  const nameInput = page.getByPlaceholder(/e.g. Acme Corp/i);
  await nameInput.fill('My Test Store');

  const categoryInput = page.getByPlaceholder(/e.g. Retail, Consulting, Tech/i);
  await categoryInput.fill('Retail');

  await page.getByRole('button', { name: /Next: Choose Vibe/i }).click();

  await expect(page.getByText(/Select Your Vibe/i)).toBeVisible();
  await page.getByRole('button', { name: 'Friendly' }).click();
  await page.getByRole('button', { name: /Next: Details/i }).click();

  await expect(page.getByText(/Final Details/i)).toBeVisible();
  const textarea = page.getByPlaceholder(/e.g. I run a mobile dog grooming service/i);
  await textarea.fill('I run a friendly retail store selling amazing products');

  await page.getByRole('button', { name: /Build Store/i }).click();

  await expect(page.getByText(/Pick your draft/i)).toBeVisible({ timeout: 5000 });
  await page.getByRole('button', { name: /Customize Selected Draft/i }).click();

  // Now we are in the editor view with the generated blocks.
  // Click on the first block to open action sheet
  await page.getByRole('heading', { name: /My Test Store/i }).click({ force: true });

  // Wait for the action sheet to open
  await expect(page.getByText(/Upload Photo/i)).toBeVisible();

  // Trigger file upload
  const fileChooserPromise = page.waitForEvent('filechooser');
  // Need to click on the label/input area
  await page.getByText(/Upload Photo/i).click({ force: true });
  const fileChooser = await fileChooserPromise;
  await fileChooser.setFiles({
    name: 'test-image.jpg',
    mimeType: 'image/jpeg',
    buffer: Buffer.from('fake-image-data')
  });

  // Action sheet should close and we should see "AI enhancing image"
  await expect(page.getByText(/AI enhancing image.../i)).toBeVisible();

  // Open action sheet again to verify advanced settings
  await page.getByRole('heading', { name: /My Test Store/i }).click({ force: true });
  await expect(page.getByText(/Advanced Media Settings/i)).toBeVisible();
  await page.getByText(/Advanced Media Settings/i).click();
  await expect(page.getByText(/Auto AI Smart Crop/i)).toBeVisible();

});
