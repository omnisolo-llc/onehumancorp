import { test, expect, Page } from '@playwright/test';

async function waitForFlutter(page: Page, timeoutMs = 30_000): Promise<void> {
  await page.waitForFunction(
    () => {
      const body = document.body;
      return (
        body &&
        (body.querySelector('flt-glass-pane') !== null ||
          body.querySelector('canvas') !== null ||
          body.children.length > 0)
      );
    },
    { timeout: timeoutMs },
  );
}

async function loginAsSeededAdmin(page: Page): Promise<void> {
  await page.goto('/');
  await waitForFlutter(page);

  await page.keyboard.press('Tab');
  await page.keyboard.press('Tab');
  await page.keyboard.type('admin@test.local');
  await page.keyboard.press('Tab');
  await page.keyboard.type('adminpass123');
  await page.keyboard.press('Enter');
  await page.waitForTimeout(1000);
  await expect(page).not.toHaveURL(/\/login/);
}

const featureRoutes = [
  '/dashboard',
  '/agents',
  '/orchestration/tasks',
  '/swarm-memory',
  '/meetings',
  '/chat',
  '/handoffs',
  '/cost',
  '/scaling',
  '/pipelines',
  '/growth-experiments',
  '/referrals',
  '/integrations',
  '/users',
  '/channels',
  '/ai-config',
  '/skills',
  '/security',
  '/logs',
  '/settings',
  '/service',
  '/wizard',
  '/diagnostics',
];

test.describe('Flutter Web App – E2E', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.evaluate(async () => {
      await fetch(window.location.origin + '/api/dev/seed', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ scenario: 'launch-readiness' }),
      });
    });
    await page.goto('/');
    await waitForFlutter(page);
  });

  test('web app opens in app mode login view (no marketing ads)', async ({
    page,
  }) => {
    await expect(page).toHaveTitle(/One Human Corp|ohc_app/i);
    await expect(page).toHaveURL(/\/login/);
    await expect(page.getByText('Switch to Cloud mode')).toHaveCount(0);
    await expect(page.getByText('Download for Mac')).toHaveCount(0);
  });

  test('seeded user can sign in and reach dashboard', async ({ page }) => {
    await loginAsSeededAdmin(page);
    await expect(page).toHaveURL(/\/dashboard/);
  });

  for (const route of featureRoutes) {
    test(`feature route works: ${route}`, async ({ page }) => {
      await loginAsSeededAdmin(page);
      await page.goto(route);
      await waitForFlutter(page);
      const escaped = route.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
      await expect(page).toHaveURL(new RegExp(escaped));
      const html = await page.content();
      expect(html.length).toBeGreaterThan(100);
    });
  }
});
