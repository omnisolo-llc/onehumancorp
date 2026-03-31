import { test, expect, Page, request } from '@playwright/test';

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

test.describe('Flutter Web App – E2E CUJ', () => {
  let authToken = '';

  test.beforeAll(async () => {
    // 1. Seed the database
    const apiContext = await request.newContext({
      baseURL: 'http://127.0.0.1:8081',
    });

    // Actually, dev/seed might need admin auth or might be unauthenticated in tests.
    await apiContext.post('/api/dev/seed', {
      data: { scenario: 'test_cuj' }
    });

    // 2. Authenticate and get token
    const authResp = await apiContext.post('/api/auth/login', {
      data: { username: 'admin', password: 'adminpass123' },
    });
    const authData = await authResp.json();
    authToken = authData.token;
  });

  test.beforeEach(async ({ page }) => {
    // Navigate to root first so we are on the correct origin to set localStorage
    await page.goto('/');

    // Inject the token to bypass authentication UI
    await page.evaluate((token) => {
      window.localStorage.setItem('flutter.auth_token', `"${token}"`);
    }, authToken);

    // Reload so Flutter boots up with the auth token
    await page.reload();
    await waitForFlutter(page);
  });

  test('page title contains "One Human Corp"', async ({ page }) => {
    await expect(page).toHaveTitle(/One Human Corp/i);
  });

  test('Flutter root element is mounted', async ({ page }) => {
    const flutterPresent = await page.evaluate(() => {
      return (
        document.querySelector('flt-glass-pane') !== null ||
        document.querySelector('canvas') !== null ||
        document.body.innerHTML.length > 100
      );
    });
    expect(flutterPresent).toBe(true);
  });

  test('navigates to dashboard directly via auth bypass', async ({ page }) => {
    // Since we are authenticated, it should not redirect to login.
    await expect(page).not.toHaveURL(/.*login.*/);
  });

  test('flutter.js or main.dart.js is served', async ({ page }) => {
    const resources: string[] = [];
    page.on('response', (res) => resources.push(res.url()));
    await page.reload();
    await waitForFlutter(page);

    const hasFlutterAsset = resources.some(
      (url) =>
        url.includes('flutter.js') ||
        url.includes('main.dart.js') ||
        url.includes('flutter_bootstrap.js') ||
        url.includes('.wasm'),
    );
    expect(hasFlutterAsset).toBe(true);
  });
});
