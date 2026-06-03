import { chromium } from 'playwright';

(async () => {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();

  await page.goto('http://127.0.0.1:8081/inbox');
  console.log("Navigated to inbox");

  await page.waitForTimeout(2000);

  const html = await page.content();
  if (html.includes('Simulate Incoming Message')) {
      console.log("Found Simulate Incoming Message button");
      await page.getByRole('button', { name: '🤖 Simulate Incoming Message' }).click();
      console.log("Clicked Simulate Incoming Message button");
      await page.waitForTimeout(2000);
      const html2 = await page.content();
      if (html2.includes('AI Replied')) {
          console.log("Found AI Replied");
      } else {
          console.log("Did NOT find AI Replied");
      }
  } else {
      console.log("Did NOT find Simulate Incoming Message button");
  }

  await browser.close();
})();
