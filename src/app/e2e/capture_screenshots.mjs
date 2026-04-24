import { chromium, firefox, webkit } from '@playwright/test';
import * as fs from 'fs';
import * as path from 'path';

const outDir = process.env.APP_SCREENSHOT_OUTPUT_DIR || './docs/public/assets/screenshots/app';
const baseUrl = process.env.PLAYWRIGHT_BASE_URL || 'http://127.0.0.1:8081';

const VIEWPORTS = [
  { name: 'iphone-se', width: 375, height: 667 },
  { name: 'iphone-14', width: 390, height: 844 },
  { name: 'iphone-plus', width: 414, height: 896 },
  { name: 'ipad', width: 768, height: 1024 },
  { name: 'desktop', width: 1440, height: 900 }
];

async function capture() {
  if (!fs.existsSync(outDir)) {
    fs.mkdirSync(outDir, { recursive: true });
  }

  const landingPageDir = path.join(outDir, 'landing-page');
  if (!fs.existsSync(landingPageDir)) {
    fs.mkdirSync(landingPageDir, { recursive: true });
  }

  console.log(`Navigating to ${baseUrl}`);

  for (const browserType of [chromium, firefox, webkit]) {
    const browserName = browserType.name();
    const browser = await browserType.launch();

    for (const viewport of VIEWPORTS) {
      console.log(`Capturing ${browserName} at ${viewport.name} (${viewport.width}x${viewport.height})...`);
      const page = await browser.newPage({
        viewport: { width: viewport.width, height: viewport.height }
      });
      await page.goto(baseUrl);
      await page.waitForTimeout(2000);

      // Save as home.png for desktop Chromium to maintain backward compatibility,
      // and use specific names for others.
      const filename = (browserName === 'chromium' && viewport.name === 'desktop')
        ? 'home.png'
        : `home-${browserName}-${viewport.name}.png`;

      await page.screenshot({ path: path.join(landingPageDir, filename) });
      await page.close();
    }
    await browser.close();
  }
}

capture().catch(e => {
  console.error(e);
  process.exit(1);
});
