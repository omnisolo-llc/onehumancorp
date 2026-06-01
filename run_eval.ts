import { test, expect } from '@playwright/test';

test('Evaluate HTML', async ({ page }) => {
  await page.goto('http://localhost:3000/');
  const html = await page.content();
  console.log(html.substring(0, 500));
});
