import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { chromium } from '@playwright/test';

const baseUrl = process.env.VISUAL_AUDIT_BASE_URL || 'http://127.0.0.1:3000';
const outputDir = process.env.VISUAL_AUDIT_OUTPUT_DIR || '/tmp/ohc-visual-audit';
const executablePath = process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH;

const routes = [
  '/dashboard',
  '/assistant',
  '/orders',
  '/inventory',
  '/inbox',
  '/agents',
  '/settings',
  '/business-analytics',
  '/integrations',
  '/calendar',
  '/diagnostics',
  '/agent-marketplace',
  '/visual-workflow',
  '/website-builder',
  '/booking-widget',
  '/storefront-widget',
  '/onboarding',
  '/login',
];

const viewports = {
  desktop: { width: 1440, height: 1000 },
  mobile: { width: 390, height: 844 },
};

const emptyMetrics = (viewport) => ({
  title: '',
  bodySummary: '',
  viewportWidth: viewport.width,
  documentWidth: null,
  horizontalOverflow: false,
  shellCounts: {
    sidebar: 0,
    topbar: 0,
    main: 0,
  },
  visibleOverflowingElements: [],
});

await mkdir(outputDir, { recursive: true });
const results = [];
let browser;

try {
  browser = await chromium.launch({
    executablePath,
    headless: true,
    args: ['--no-sandbox'],
  });

  for (const [viewportName, viewport] of Object.entries(viewports)) {
    let context;
    try {
      context = await browser.newContext({ viewport });

      for (const route of routes) {
        let page;
        const slug = route.slice(1).replaceAll('/', '__') || 'home';
        const screenshot = path.join(outputDir, `${viewportName}__${slug}.png`);
        const consoleErrors = [];
        let status = null;
        let navigationError = null;
        let captureError = null;
        let screenshotError = null;
        let metrics = emptyMetrics(viewport);

        try {
          page = await context.newPage();
          page.on('console', (message) => {
            if (message.type() === 'error') consoleErrors.push(message.text());
          });
          page.on('pageerror', (error) => consoleErrors.push(error.message));

          try {
            const response = await page.goto(`${baseUrl}${route}`, {
              waitUntil: 'domcontentloaded',
              timeout: 30_000,
            });
            status = response?.status() ?? null;
            await page.waitForTimeout(750);
          } catch (error) {
            navigationError = error instanceof Error ? error.message : String(error);
          }

          try {
            metrics = await page.evaluate(() => {
              const viewportWidth = window.innerWidth;
              const documentWidth = document.documentElement.scrollWidth;
              const visibleOverflowingElements = [...document.body.querySelectorAll('*')]
                .map((element) => {
                  const rect = element.getBoundingClientRect();
                  const style = window.getComputedStyle(element);
                  const visible = rect.width > 0
                    && rect.height > 0
                    && style.display !== 'none'
                    && style.visibility !== 'hidden';

                  return {
                    tag: element.tagName.toLowerCase(),
                    id: element.id,
                    className: element.getAttribute('class') || '',
                    left: Math.round(rect.left * 100) / 100,
                    right: Math.round(rect.right * 100) / 100,
                    width: Math.round(rect.width * 100) / 100,
                    visible,
                  };
                })
                .filter((item) => item.visible && (item.left < -1 || item.right > viewportWidth + 1))
                .map(({ visible: _visible, ...item }) => item);

              return {
                title: document.title,
                bodySummary: document.body.innerText.replace(/\s+/g, ' ').trim().slice(0, 400),
                viewportWidth,
                documentWidth,
                horizontalOverflow: documentWidth > viewportWidth + 1,
                shellCounts: {
                  sidebar: document.querySelectorAll('.app-sidebar').length,
                  topbar: document.querySelectorAll('.app-topbar').length,
                  main: document.querySelectorAll('.app-main').length,
                },
                visibleOverflowingElements,
              };
            });
          } catch (error) {
            captureError = error instanceof Error ? error.message : String(error);
          }

          try {
            await page.screenshot({ path: screenshot, fullPage: true });
          } catch (error) {
            screenshotError = error instanceof Error ? error.message : String(error);
          }
        } finally {
          if (page) await page.close().catch(() => {});
        }

        results.push({
          route,
          viewport: viewportName,
          status,
          title: metrics.title,
          bodySummary: metrics.bodySummary,
          shellCounts: metrics.shellCounts,
          consoleErrors,
          viewportWidth: metrics.viewportWidth,
          documentWidth: metrics.documentWidth,
          horizontalOverflow: metrics.horizontalOverflow,
          visibleOverflowingElements: metrics.visibleOverflowingElements,
          screenshot,
          navigationError,
          captureError,
          screenshotError,
        });
      }
    } finally {
      if (context) await context.close().catch(() => {});
    }
  }
} finally {
  if (browser) await browser.close().catch(() => {});
}

const reportPath = path.join(outputDir, 'report.json');
await writeFile(reportPath, `${JSON.stringify(results, null, 2)}\n`);

const failures = results
  .map((result) => {
    const reasons = [];
    if (result.navigationError) reasons.push('navigation error');
    if (result.status !== null && result.status >= 400) reasons.push(`HTTP ${result.status}`);
    for (const [shell, count] of Object.entries(result.shellCounts)) {
      if (count !== 1) reasons.push(`${shell} count ${count}`);
    }
    if (result.horizontalOverflow) {
      reasons.push(`horizontal overflow ${result.documentWidth - result.viewportWidth}px`);
    }
    if (result.captureError) reasons.push('capture error');
    if (result.screenshotError) reasons.push('screenshot error');
    return { route: result.route, viewport: result.viewport, reasons };
  })
  .filter((result) => result.reasons.length > 0);

process.stdout.write(`${JSON.stringify({
  pages: results.length,
  failures: failures.length,
  failureCases: failures,
  reportPath,
  screenshots: results.length,
}, null, 2)}\n`);

if (failures.length > 0) process.exitCode = 1;
