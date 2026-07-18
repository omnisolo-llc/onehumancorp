import { test, expect } from '@playwright/test';

test('End-to-End Promoter Product Creation Flow', async ({ page }) => {
  // 1. Navigate to Add Product page directly
  await page.goto('/products/new');
  await expect(page.locator('h1').filter({ hasText: 'Add Product' })).toBeVisible();

  // 2. Click the take photo area to trigger file upload (we mock the change event by setting the file programmatically if needed, but since it's an input we can just click and expect the loading state to trigger in our simplified UI if we mock it, or we bypass the file picker dialog and set the state directly in the app, but here we just wait for the loading state to appear after setting a file)
  // Actually, setting a file is the right way
  const fileChooserPromise = page.waitForEvent('filechooser');
  await page.locator('span:has-text("Take a photo or upload")').click();
  const fileChooser = await fileChooserPromise;
  await fileChooser.setFiles({
    name: 'test-image.jpg',
    mimeType: 'image/jpeg',
    buffer: Buffer.from('fake image data')
  });

  // 3. Loading state appears
  await expect(page.locator('p').filter({ hasText: 'The Promoter is working its magic...' })).toBeVisible();

  // 4. Form appears pre-filled after loading
  await expect(page.locator('input[value="Artisan Cupcake"]')).toBeVisible({ timeout: 5000 });
  await expect(page.locator('input[value="6.50"]')).toBeVisible();

  // 5. User taps "Looks Good"
  await page.click('button:has-text("Looks Good")');

  // 6. Success message
  await expect(page.locator('h2').filter({ hasText: 'Product Published!' })).toBeVisible();
  await expect(page.locator('p').filter({ hasText: 'Your new product is now live on your storefront.' })).toBeVisible();

  // 7. Wait a moment for the event to be processed by the agent
  await page.waitForTimeout(2000);

  // 8. Navigate to Agent Feed (Team page)
  await page.goto('/team');
  await expect(page.locator('h1').filter({ hasText: 'Your Team' })).toBeVisible();

  // 9. Verify the Promoter Action Card is present
  const actionCard = page.locator('div:has-text("Social Post Drafted")').first();
  await expect(actionCard).toBeVisible();

  // 10. Verify the multi-platform variants are present
  await expect(actionCard.locator('text=Instagram')).toBeVisible();

  // 11. Click Approve / Schedule Post
  await actionCard.locator('button:has-text("Schedule Post")').click();
});
