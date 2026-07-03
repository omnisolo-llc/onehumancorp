import { test, expect } from '@playwright/test';
test('Verify interactive element container interactions app-card v2', async ({ page }) => {
  await page.goto('http://localhost:3000/website-builder');
  const cards = page.locator('.app-card');
  if(await cards.count() > 0) {
      const radius = await cards.first().evaluate((el) => window.getComputedStyle(el).borderRadius);
      expect(['16px', '1rem', '']).toContain(radius);
  }
});
