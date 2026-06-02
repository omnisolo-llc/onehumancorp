import { test, expect } from '@playwright/test';

test.describe('Wall of Love Widget Growth Loop', () => {
  test('should fetch data and render reviews with viral loop link via full application stack', async ({ page, baseURL }) => {
    // Navigate to a valid domain so CORS/fetch issues don't occur when script executes
    await page.goto(`${baseURL}/help`);

    // 1. Inject the container
    await page.evaluate(() => {
      const container = document.createElement('div');
      container.id = 'ohc-wall-of-love';
      container.setAttribute('data-store', 'Test Store');
      document.body.appendChild(container);
    });

    // 2. We inject the script tag linking to our real application stack
    // Since we are running against a pre-built app running on localhost:3000 (baseURL),
    // we instruct the script to load from that domain. The script's logic
    // will detect the origin of the script tag and construct the fetch URL to point to baseURL.
    await page.evaluate((url) => {
      const script = document.createElement('script');
      script.src = url;
      script.async = true;
      document.head.appendChild(script);
    }, `${baseURL}/widgets/wall-of-love`);

    // 4. Assertions
    const container = page.locator('#ohc-wall-of-love');

    // Check for the header (waiting for it ensures the network fetch finishes and UI renders)
    await expect(container.locator('h3')).toContainText('What people say about Test Store');

    // Check for the review content (from the real mock data in the endpoint)
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
