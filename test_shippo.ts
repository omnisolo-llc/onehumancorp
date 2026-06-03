import { chromium } from '@playwright/test';

(async () => {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();

  await page.goto('http://127.0.0.1:8081/orders');
  console.log("Navigated to orders");

  await page.waitForTimeout(2000);

  const html = await page.content();
  if (html.includes('Unfulfilled')) {
      console.log("Found Unfulfilled text");
  } else {
      console.log("Did NOT find Unfulfilled text");
  }

  await browser.close();
})();
