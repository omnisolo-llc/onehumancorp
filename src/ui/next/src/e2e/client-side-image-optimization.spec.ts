import { test, expect } from '../../../../e2e/fixtures';
import * as path from 'path';
import * as fs from 'fs';
import * as os from 'os';

test.describe('Client-side Image Optimization', () => {
  let testImagePath: string;

  test.beforeAll(async () => {
    const tempDir = os.tmpdir();
    testImagePath = path.join(tempDir, 'large-test-image.jpg');

    // We create a tiny valid image so the browser doesn't immediately fail processing
    // 1x1 transparent GIF
    const buffer = Buffer.from('R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7', 'base64');
    // Pad it out to 1MB so it triggers our size checks
    const paddedBuffer = Buffer.concat([buffer, Buffer.alloc(1024 * 1024 - buffer.length)]);
    fs.writeFileSync(testImagePath, paddedBuffer);
  });

  test.afterAll(() => {
    if (fs.existsSync(testImagePath)) {
      fs.unlinkSync(testImagePath);
    }
  });

  test('should intercept large images, compress to webp, and upload', async ({ page }) => {
    // Intercept network requests to /api/auto-catalog
    await page.route('**/api/auto-catalog', async route => {
      const request = route.request();
      const postData = request.postData();

      if (request.method() === 'POST' && postData) {
          // Verify the payload contains the webp extension replacement
          expect(postData).toContain('.webp');
          expect(postData).toContain('image/webp'); // Mimetype was replaced

          await route.fulfill({
            status: 200,
            contentType: 'application/json',
            body: JSON.stringify({
              title: 'Auto-cataloged Product',
              description: 'A test description',
              price: '10.00',
              category: 'Test Category'
            })
          });
      } else {
          route.continue();
      }
    });

    await page.goto('/products/new');

    // Wait for the UI to be ready
    const fileInput = page.locator('input[type="file"]');
    await expect(fileInput).toBeAttached();
    await expect(page.getByText('Take a photo or upload')).toBeVisible();

    // Trigger upload via the actual UI
    await fileInput.setInputFiles(testImagePath);

    // Because of our mock backend response, we should transition to seeing "Looks Good"
    // from the auto-cataloging UI flow inside the product creation page
    await expect(page.getByText('Looks Good')).toBeVisible({ timeout: 10000 });
  });
});
