import { chromium } from '@playwright/test';

const browser = await chromium.launch({
  headless: false,
  args: ['--no-sandbox', '--disable-dev-shm-usage', '--use-gl=egl', '--enable-webgl', '--enable-webgl2'],
});
const page = await browser.newPage({ viewport: { width: 1280, height: 800 } });
await page.goto('http://127.0.0.1:8765/');
console.log('Waiting for Flutter...');
try {
  await page.waitForFunction(
    () => Boolean(document.querySelector('flutter-view') || document.querySelector('flt-glass-pane') || document.querySelector('canvas')),
    { timeout: 30000 }
  );
  console.log('Flutter element found!');
} catch (e) {
  console.log('Flutter element NOT found:', e.message);
}
await page.waitForTimeout(3000);
await page.screenshot({ path: '/tmp/debug_landing.png', fullPage: true });
console.log('DOM:', (await page.content()).substring(0, 500));
await browser.close();
