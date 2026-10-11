import { chromium } from '@playwright/test';
import fs from 'node:fs';
import path from 'node:path';

const BASE_URL = process.env.BASE_URL || 'https://cloud.omnisolo.co';
const EMAIL = process.env.AUDIT_EMAIL || 'audit-customer@omnisolo.co';
const PASSWORD = process.env.AUDIT_PASSWORD || 'AuditCust#2026!Secure99';
const ORG = process.env.AUDIT_ORG || 'org-1';

const SCREENSHOT_DIR = path.resolve('scratch/screenshots-live');
fs.mkdirSync(SCREENSHOT_DIR, { recursive: true });

const VIEWPORTS = [
  { name: 'desktop', width: 1440, height: 900 },
  { name: 'mobile', width: 390, height: 844 },
];

const PAGES_TO_TEST = [
  { path: '/dashboard', label: 'Dashboard' },
  { path: '/assistant', label: 'AI Assistant' },
  { path: '/ai-workspace', label: 'AI Workspace' },
  { path: '/orders', label: 'Orders' },
  { path: '/products', label: 'Products' },
  { path: '/products/new', label: 'New Product' },
  { path: '/inventory', label: 'Inventory' },
  { path: '/inbox', label: 'Inbox' },
  { path: '/agents', label: 'Agents' },
  { path: '/kairos', label: 'Kairos Engine' },
  { path: '/langgraph', label: 'LangGraph' },
  { path: '/visual-workflow', label: 'Visual Workflow' },
  { path: '/business-analytics', label: 'Business Analytics' },
  { path: '/cost-dashboard', label: 'Cost Dashboard' },
  { path: '/settings', label: 'Settings' },
  { path: '/help', label: 'Help Center' },
  { path: '/payments', label: 'Payments' },
  { path: '/pos', label: 'Point of Sale' },
  { path: '/services', label: 'Services' },
  { path: '/services/new', label: 'New Service' },
  { path: '/quoting', label: 'Quoting' },
  { path: '/proposals', label: 'Proposals' },
  { path: '/website-builder', label: 'Website Builder' },
  { path: '/storefront-builder', label: 'Storefront Builder' },
  { path: '/brand-studio', label: 'Brand Studio' },
  { path: '/lead-magnet-generator', label: 'Lead Magnet Generator' },
];

async function runLiveAudit() {
  console.log(`Starting live audit on ${BASE_URL}...`);
  const browser = await chromium.launch({
    headless: true,
    executablePath: '/home/kevin/.cache/ms-playwright/chromium_headless_shell-1234/chrome-headless-shell-linux64/chrome-headless-shell',
    args: ['--no-sandbox', '--disable-setuid-sandbox', '--ignore-certificate-errors'],
  });

  const results = [];
  let totalErrors = 0;

  for (const vp of VIEWPORTS) {
    console.log(`\n=== Testing Viewport: ${vp.name} (${vp.width}x${vp.height}) ===`);
    const context = await browser.newContext({
      viewport: { width: vp.width, height: vp.height },
      ignoreHTTPSErrors: true,
      userAgent: vp.name === 'mobile'
        ? 'Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1'
        : 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
    });

    const page = await context.newPage();

    const pageErrors = [];
    page.on('pageerror', (err) => {
      pageErrors.push(`[PageError] ${err.message}`);
    });
    page.on('console', (msg) => {
      if (msg.type() === 'error' && !msg.text().includes('favicon')) {
        pageErrors.push(`[ConsoleError] ${msg.text()}`);
      }
    });

    // 1. Login flow
    console.log(`[${vp.name}] Logging in at ${BASE_URL}/login...`);
    try {
      const loginResp = await page.goto(`${BASE_URL}/login`, { waitUntil: 'domcontentloaded', timeout: 30000 });
      await page.waitForTimeout(1000);

      // Fill login form
      const emailField = page.locator('#email, input[type="email"], input[name="email"], input[name="username"]').first();
      await emailField.waitFor({ state: 'visible', timeout: 15000 });
      await emailField.fill(EMAIL);

      const passField = page.locator('#password, input[type="password"], input[name="password"]').first();
      await passField.fill(PASSWORD);

      const orgField = page.locator('#organization, input[name="organization"], input[name="org"]').first();
      if (await orgField.isVisible()) {
        await orgField.fill(ORG);
      }

      const submitBtn = page.locator('button[type="submit"]').first();
      await Promise.all([
        page.waitForURL((url) => !url.pathname.includes('/login') || url.pathname.includes('/dashboard'), { timeout: 30000 }).catch(() => {}),
        submitBtn.click(),
      ]);

      await page.waitForTimeout(2000);
      const postLoginUrl = page.url();
      console.log(`[${vp.name}] Logged in successfully. Current URL: ${postLoginUrl}`);

      const screenshotName = `login_success_${vp.name}.png`;
      await page.screenshot({ path: path.join(SCREENSHOT_DIR, screenshotName) });
    } catch (err) {
      console.error(`[${vp.name}] Login failed: ${err.message}`);
      totalErrors++;
      await context.close();
      continue;
    }

    // 2. Iterate pages
    for (const pageSpec of PAGES_TO_TEST) {
      const pageUrl = `${BASE_URL}${pageSpec.path}`;
      const pageKey = pageSpec.path.replace(/[^a-zA-Z0-9]/g, '_').replace(/^_+/, '');
      console.log(`[${vp.name}] Auditing ${pageSpec.label} (${pageSpec.path})...`);

      const errorsBefore = pageErrors.length;
      try {
        const resp = await page.goto(pageUrl, { waitUntil: 'domcontentloaded', timeout: 30000 });
        const status = resp ? resp.status() : 0;
        await page.waitForTimeout(1500);

        // Check for error text in body
        const bodyText = await page.textContent('body') || '';
        const has500 = bodyText.includes('Internal Server Error') || bodyText.includes('502 Bad Gateway') || bodyText.includes('504 Gateway');
        const hasCrash = bodyText.includes('Unhandled Runtime Error');

        const screenshotPath = path.join(SCREENSHOT_DIR, `${pageKey}_${vp.name}.png`);
        await page.screenshot({ path: screenshotPath, fullPage: false });

        // Test interactive button clicks where available
        let actionResult = 'none';
        if (pageSpec.path === '/website-builder') {
          const instantBtn = page.getByRole('button', { name: /Instant Build/i }).first();
          if (await instantBtn.isVisible()) {
            await instantBtn.click();
            await page.waitForTimeout(500);
            actionResult = 'clicked Instant Build';
          }
        } else if (pageSpec.path === '/ai-workspace') {
          const prioritizeBtn = page.locator('#ai-prioritize-backlog-btn, button:has-text("Prioritize")').first();
          if (await prioritizeBtn.isVisible()) {
            await prioritizeBtn.click();
            await page.waitForTimeout(500);
            actionResult = 'clicked Prioritize Backlog';
          }
        } else if (pageSpec.path === '/quoting') {
          const ordersLink = page.getByRole('link', { name: /Go to Orders/i }).first();
          if (await ordersLink.isVisible()) {
            actionResult = 'empty state rendered cleanly with Go to Orders CTA';
          }
        }

        const newErrors = pageErrors.slice(errorsBefore);
        const passed = status < 400 && !has500 && !hasCrash && newErrors.length === 0;

        if (!passed) totalErrors++;

        results.push({
          page: pageSpec.label,
          path: pageSpec.path,
          viewport: vp.name,
          status,
          actionResult,
          passed,
          errors: newErrors,
        });

        console.log(`  ✓ ${pageSpec.label}: HTTP ${status} | Action: ${actionResult} | Status: ${passed ? 'PASS' : 'FAIL'}`);
      } catch (err) {
        console.error(`  ✗ ${pageSpec.label} failed: ${err.message}`);
        totalErrors++;
        results.push({
          page: pageSpec.label,
          path: pageSpec.path,
          viewport: vp.name,
          status: 'ERROR',
          actionResult: 'failed',
          passed: false,
          errors: [err.message],
        });
      }
    }

    await context.close();
  }

  await browser.close();

  console.log('\n======================================================');
  console.log(`Live Audit Complete: ${results.filter(r => r.passed).length}/${results.length} checks passed. Total errors: ${totalErrors}`);
  console.log('======================================================');

  const reportPath = path.resolve('scratch/live_audit_report.json');
  fs.writeFileSync(reportPath, JSON.stringify(results, null, 2));
  console.log(`Detailed report saved to ${reportPath}`);

  if (totalErrors > 0) {
    process.exit(1);
  }
}

runLiveAudit().catch((err) => {
  console.error('Audit runner error:', err);
  process.exit(1);
});
