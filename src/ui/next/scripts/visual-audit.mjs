import { chmod, mkdir, stat, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { chromium } from '@playwright/test';
import {
  HYDRATION_FAILURE_PATTERN,
  classifyConsoleError,
  failureReasons,
  isCoverageComplete,
  shouldFailAudit,
} from './visual-audit-policy.mjs';

const baseUrl = process.env.VISUAL_AUDIT_BASE_URL || 'http://127.0.0.1:3000';
const outputDir = process.env.VISUAL_AUDIT_OUTPUT_DIR || '/tmp/ohc-visual-audit';
const executablePath = process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH;
const captureBodyText = process.env.VISUAL_AUDIT_CAPTURE_BODY_TEXT === '1';
const allowNoSandbox = process.env.VISUAL_AUDIT_ALLOW_NO_SANDBOX === '1';

const MAX_CONSOLE_ERRORS = 20;
const MAX_ERROR_LENGTH = 500;
const MAX_BODY_SAMPLE_LENGTH = 2_000;
const MAX_BODY_SUMMARY_LENGTH = 800;

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

const auditCases = Object.entries(viewports).flatMap(([viewportName, viewport]) =>
  routes.map((route) => ({ route, viewportName, viewport })));

function redactAndLimit(value, maxLength = MAX_ERROR_LENGTH) {
  const redacted = String(value ?? '')
    .replace(/\bBearer\s+[A-Za-z0-9._~+/-]+=*/gi, 'Bearer [REDACTED]')
    .replace(
      /([?&](?:api[_-]?key|access[_-]?token|auth(?:orization)?|token|password|secret|client[_-]?secret|signature)=)[^&#\s]*/gi,
      '$1[REDACTED]',
    )
    .replace(
      /(\b(?:api[_-]?key|access[_-]?token|auth(?:orization)?|token|password|secret|client[_-]?secret|signature)\b\s*[:=]\s*)(?:"[^"]*"|'[^']*'|[^\s,;&]+)/gi,
      '$1[REDACTED]',
    )
    .replace(/\b(?:sk|pk)[_-][A-Za-z0-9_-]{12,}\b/gi, '[REDACTED_KEY]')
    .replace(/\bAKIA[A-Z0-9]{16}\b/g, '[REDACTED_KEY]')
    .replace(
      /\b(?:gh[pousr]|github_pat|xox[baprs])_[A-Za-z0-9_-]{12,}\b/gi,
      '[REDACTED_KEY]',
    );

  if (redacted.length <= maxLength) return redacted;

  const truncationMarker = '…[truncated]';
  return `${redacted.slice(0, maxLength - truncationMarker.length)}${truncationMarker}`;
}

function emptyMetrics(viewport) {
  return {
    title: '',
    bodySummary: {
      captureEnabled: captureBodyText,
      text: null,
    },
    viewportWidth: viewport.width,
    documentWidth: null,
    horizontalOverflow: false,
    shellCounts: {
      sidebar: 0,
      topbar: 0,
      main: 0,
    },
    visibleOverflowingElements: [],
  };
}

function createResult({ route, viewportName, viewport }, attempted = true) {
  const slug = route.slice(1).replaceAll('/', '__') || 'home';
  return {
    route,
    viewport: viewportName,
    attempted,
    completed: false,
    status: null,
    ...emptyMetrics(viewport),
    consoleErrors: [],
    expectedServiceErrors: [],
    unexpectedConsoleErrors: [],
    hydrationErrors: [],
    pageErrors: [],
    screenshot: path.join(outputDir, `${viewportName}__${slug}.png`),
    screenshotWritten: false,
    navigationError: null,
    captureError: null,
    screenshotError: null,
  };
}

process.umask(0o077);

const results = [];
let browser;
let fatalError = null;
let outputReady = false;

try {
  await mkdir(outputDir, { recursive: true, mode: 0o700 });
  await chmod(outputDir, 0o700);
  outputReady = true;

  const launchOptions = {
    executablePath,
    headless: true,
    ...(allowNoSandbox ? { args: ['--no-sandbox'] } : {}),
  };
  browser = await chromium.launch(launchOptions);

  for (const auditCase of auditCases) {
    const result = createResult(auditCase);
    let context;
    let page;

    try {
      context = await browser.newContext({ viewport: auditCase.viewport });
      page = await context.newPage();

      const recordBounded = (collection, diagnostic) => {
        if (collection.length >= MAX_CONSOLE_ERRORS) return;
        collection.push({
          message: redactAndLimit(diagnostic.message),
          locationUrl: redactAndLimit(diagnostic.locationUrl),
          pageUrl: redactAndLimit(diagnostic.pageUrl),
        });
      };
      page.on('console', (message) => {
        if (message.type() !== 'error') return;
        const diagnostic = {
          message: message.text(),
          locationUrl: message.location().url || '',
          pageUrl: page.url(),
        };
        recordBounded(result.consoleErrors, diagnostic);
        const classification = classifyConsoleError(diagnostic);
        if (classification === 'expected-service') recordBounded(result.expectedServiceErrors, diagnostic);
        if (classification === 'unexpected') recordBounded(result.unexpectedConsoleErrors, diagnostic);
        if (classification === 'hydration') recordBounded(result.hydrationErrors, diagnostic);
      });
      page.on('pageerror', (error) => {
        const diagnostic = {
          message: error.message,
          locationUrl: error.stack || '',
          pageUrl: page.url(),
        };
        recordBounded(result.pageErrors, diagnostic);
        if (HYDRATION_FAILURE_PATTERN.test(error.message)) recordBounded(result.hydrationErrors, diagnostic);
      });

      try {
        const response = await page.goto(`${baseUrl}${auditCase.route}`, {
          waitUntil: 'domcontentloaded',
          timeout: 30_000,
        });
        result.status = response?.status() ?? null;
        await page.waitForFunction(() => document.querySelectorAll('.app-sidebar').length === 1
          && document.querySelectorAll('.app-topbar').length === 1
          && document.querySelectorAll('.app-main').length === 1, undefined, { timeout: 30_000 });
        if (auditCase.route === '/inbox') {
          await page.getByTestId('inbox-settled').waitFor({ state: 'visible', timeout: 30_000 });
        }
        await page.waitForLoadState('load', { timeout: 5_000 }).catch(() => {});
        await page.waitForTimeout(1_000);
      } catch (error) {
        result.navigationError = redactAndLimit(error instanceof Error ? error.message : error);
      }

      try {
        const metrics = await page.evaluate(
          ({ shouldCaptureBodyText, bodySampleLimit }) => {
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
                  id: element.id.slice(0, 200),
                  className: (element.getAttribute('class') || '').slice(0, 300),
                  left: Math.round(rect.left * 100) / 100,
                  right: Math.round(rect.right * 100) / 100,
                  width: Math.round(rect.width * 100) / 100,
                  visible,
                };
              })
              .filter((item) => item.visible && (item.left < -1 || item.right > viewportWidth + 1))
              .slice(0, 50)
              .map(({ visible: _visible, ...item }) => item);

            return {
              title: document.title.slice(0, 300),
              bodyTextSample: shouldCaptureBodyText
                ? document.body.innerText.replace(/\s+/g, ' ').trim().slice(0, bodySampleLimit)
                : null,
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
          },
          {
            shouldCaptureBodyText: captureBodyText,
            bodySampleLimit: MAX_BODY_SAMPLE_LENGTH,
          },
        );

        result.title = redactAndLimit(metrics.title, 300);
        result.bodySummary = {
          captureEnabled: captureBodyText,
          text: captureBodyText
            ? redactAndLimit(metrics.bodyTextSample, MAX_BODY_SUMMARY_LENGTH)
            : null,
        };
        result.viewportWidth = metrics.viewportWidth;
        result.documentWidth = metrics.documentWidth;
        result.horizontalOverflow = metrics.horizontalOverflow;
        result.shellCounts = metrics.shellCounts;
        result.visibleOverflowingElements = metrics.visibleOverflowingElements;
        result.completed = true;
      } catch (error) {
        result.captureError = redactAndLimit(error instanceof Error ? error.message : error);
      }

      try {
        await page.screenshot({ path: result.screenshot, fullPage: true });
        await chmod(result.screenshot, 0o600);
        result.screenshotWritten = true;
      } catch (error) {
        result.screenshotError = redactAndLimit(error instanceof Error ? error.message : error);
      }
    } catch (error) {
      result.navigationError ||= redactAndLimit(
        `case lifecycle error: ${error instanceof Error ? error.message : String(error)}`,
      );
    } finally {
      if (page) await page.close().catch(() => {});
      if (context) await context.close().catch(() => {});
      results.push(result);
    }
  }
} catch (error) {
  fatalError = redactAndLimit(error instanceof Error ? error.message : error);
} finally {
  if (browser) await browser.close().catch(() => {});

  const completedCases = new Set(results.map((result) => `${result.viewport}:${result.route}`));
  for (const auditCase of auditCases) {
    const key = `${auditCase.viewportName}:${auditCase.route}`;
    if (completedCases.has(key)) continue;

    const result = createResult(auditCase, false);
    result.navigationError = redactAndLimit(
      `audit case not run${fatalError ? `: ${fatalError}` : ''}`,
    );
    results.push(result);
  }

  results.sort((left, right) => {
    const leftIndex = auditCases.findIndex(
      (item) => item.viewportName === left.viewport && item.route === left.route,
    );
    const rightIndex = auditCases.findIndex(
      (item) => item.viewportName === right.viewport && item.route === right.route,
    );
    return leftIndex - rightIndex;
  });

  for (const result of results) {
    if (!result.screenshotWritten) continue;
    try {
      const screenshotStat = await stat(result.screenshot);
      if (!screenshotStat.isFile()) throw new Error('screenshot path is not a file');
    } catch (error) {
      result.screenshotWritten = false;
      result.screenshotError ||= redactAndLimit(
        `screenshot verification failed: ${error instanceof Error ? error.message : String(error)}`,
      );
    }
  }

  if (outputReady) {
    const reportPath = path.join(outputDir, 'report.json');
    await writeFile(reportPath, `${JSON.stringify(results, null, 2)}\n`, { mode: 0o600 });
    await chmod(reportPath, 0o600);
  }
}

const reportPath = path.join(outputDir, 'report.json');
const failures = results
  .map((result) => ({
    route: result.route,
    viewport: result.viewport,
    reasons: failureReasons(result),
  }))
  .filter((result) => result.reasons.length > 0);
const coverageComplete = isCoverageComplete(results, auditCases);

process.stdout.write(`${JSON.stringify({
  pages: results.length,
  failures: failures.length,
  failureCases: failures.slice(0, 10),
  failureCasesTruncated: Math.max(0, failures.length - 10),
  coverageComplete,
  fatalError,
  reportPath,
  screenshots: results.filter((result) => result.screenshotWritten).length,
}, null, 2)}\n`);

if (shouldFailAudit({ results, expectedCases: auditCases, fatalError, outputReady })) process.exitCode = 1;
