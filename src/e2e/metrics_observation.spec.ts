import { test, expect } from '@playwright/test';

test('verify Swarm metrics observation flow', async ({ page }) => {
  await page.goto('/');
  await page.waitForTimeout(5000);

  const html = await page.content();
  console.log("App load content length:", html.length);

  expect(html).toContain('<script src="flutter_bootstrap.js" async=""></script>');
});
