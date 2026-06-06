import { test, expect } from '@playwright/test';

test.describe('Agentic Solutions UI Vertical Slice', () => {
  test('Zero-Touch Service Booking & Quoting', async ({ page }) => {
    // Navigate to the storefront booking page
    await page.goto('/storefront/booking');

    // Expect the page to load with the correct title
    await expect(page.getByText('Zero-Touch Service Booking & Quoting')).toBeVisible();

    // Fill out the form
    await page.setInputFiles('input[type="file"]', {
      name: 'broken-pipe.jpg',
      mimeType: 'image/jpeg',
      buffer: Buffer.from('fake image data')
    });

    await page.fill('textarea', 'A pipe under the sink is leaking');

    // Submit the form
    await page.click('button[type="submit"]');

    // Verify loading state
    await expect(page.getByText('Analyzing image & quoting...')).toBeVisible();

    // Verify the AI quote response
    await expect(page.getByText('Preliminary Quote: $150 - $250')).toBeVisible();

    // Verify description contains the problem text snippet
    await expect(page.getByText('Based on the image and description')).toBeVisible();

    // Verify time slots are presented
    await expect(page.getByText('Available Times')).toBeVisible();

    // Verify Stripe deposit link is available
    await expect(page.getByText('Pay $50 Deposit via Stripe')).toBeVisible();
  });

  test('Autonomous Inventory Scanner', async ({ page }) => {
    // Navigate to the inventory scanner page
    await page.goto('/inventory/scanner');

    // Expect the page to load with the correct title
    await expect(page.getByText('Autonomous Inventory Scanner')).toBeVisible();

    // Simulate capture image
    await page.click('button:has-text("Capture Image")');

    // Verify loading state
    await expect(page.getByText('Processing with Vision AI...')).toBeVisible();

    // Verify the AI extracted data
    await expect(page.getByText('Extracted Data')).toBeVisible();
    await expect(page.getByText('Scanned Boutique Shirt')).toBeVisible();
    await expect(page.getByText('Size S: 5 units')).toBeVisible();
    await expect(page.getByText('Size M: 12 units')).toBeVisible();
    await expect(page.getByText('Size L: 8 units')).toBeVisible();

    // Verify save button is available
    await expect(page.getByText('Confirm & Save to Catalog')).toBeVisible();
  });
});
