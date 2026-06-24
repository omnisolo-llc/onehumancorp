import { test, expect } from '@playwright/test';
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
    await page.goto('/products/new');

    // Wait for the UI to be ready
    const fileInput = page.locator('input[type="file"]');
    await expect(fileInput).toBeAttached();
    await expect(page.getByText('Take a photo or upload')).toBeVisible();

    // Set up request watcher to inspect payload without fulfilling it
    const requestPromise = page.waitForRequest(request =>
      request.url().includes('/api/auto-catalog') && request.method() === 'POST'
    );

    // Trigger upload via the actual UI
    await fileInput.setInputFiles(testImagePath);

    const request = await requestPromise;
    const postData = request.postData();
    expect(postData).toContain('.webp');
    expect(postData).toContain('image/webp');
  });
});
