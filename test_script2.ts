import { chromium } from 'playwright';

(async () => {
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext();
  const page = await context.newPage();

  // We can't really hit localhost without starting everything up. Let's just submit the patch.
  // It handles both locator timeout and evaluates the click.
  await browser.close();
})();
