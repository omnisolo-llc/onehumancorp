import { test, expect } from '@playwright/test';
import fs from 'fs';
import path from 'path';

test.describe('Wall of Love Widget Growth Loop', () => {
  test('should fetch data and render reviews with viral loop link', async ({ page }) => {
    // We intercept the API call to return a controlled response
    await page.route('**/api/v1/growth/wall_of_love/data?store=Test%20Store', async route => {
      const json = {
        reviews: [
          {
            id: 'rev_1',
            author: 'Jane D.',
            rating: 5,
            content: 'Absolutely amazing! Best purchase I have made this year.',
            date: 'Oct 12, 2023'
          }
        ]
      };
      await route.fulfill({ json });
    });

    // Instead of setting up a separate HTML file and serving it, we can navigate to a blank page
    // and inject the DOM structure and script dynamically to test the widget rendering behavior.
    await page.goto('about:blank');

    // 1. Inject the container
    await page.evaluate(() => {
      const container = document.createElement('div');
      container.id = 'ohc-wall-of-love';
      container.setAttribute('data-store', 'Test Store');
      document.body.appendChild(container);
    });

    // 2. Read the widget script content from the local file system
    const scriptPath = path.resolve(__dirname, '../../public/widgets/wall-of-love.js');
    const scriptContent = fs.readFileSync(scriptPath, 'utf-8');

    // 3. Execute the script within the page context
    await page.evaluate((script) => {
      const scriptEl = document.createElement('script');
      scriptEl.textContent = script;
      document.body.appendChild(scriptEl);
    }, scriptContent);

    // 4. Assertions
    const container = page.locator('#ohc-wall-of-love');

    // Check for the header
    await expect(container.locator('h3')).toContainText('What people say about Test Store');

    // Check for the review content
    await expect(container).toContainText('Jane D.');
    await expect(container).toContainText('Absolutely amazing!');

    // Verify the Viral Growth Loop (Powered by OHC link)
    const viralLink = container.locator('a', { hasText: 'OHC' });
    await expect(viralLink).toBeVisible();
    await expect(viralLink).toHaveAttribute('href', 'https://ohc.app?ref=wall-of-love-widget');

    // Ensure the "Powered by" text exists
    await expect(container).toContainText('⚡ Powered by');
  });
});
