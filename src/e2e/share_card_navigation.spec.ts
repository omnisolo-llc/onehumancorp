import { test, expect } from './fixtures';

test('share preview retains metadata and commits one real destination document', async ({ page, baseURL }) => {
  if (!baseURL) throw new Error('The actual test application origin is required');
  const origin = new URL(baseURL).origin;
  // Inspect the genuine server document independently of hydration. The browser
  // journey below loads its real scripts normally; it installs no route handler.
  const preview = await page.request.get('/share-card');
  expect(preview.status()).toBe(200);
  const html = await preview.text();
  expect(html).toMatch(/<meta\b[^>]*property="og:title"[^>]*content="OmniSolo"/);
  expect(html).toMatch(/<meta\b[^>]*name="twitter:card"[^>]*content="summary_large_image"/);
  expect(html).toMatch(/<a\b[^>]*href="\/onboarding"/);
  expect(html).not.toMatch(/http-equiv=["']refresh["']/i);

  const destinationRequests: string[] = [];
  const startupPaths = ['/api/v1/auth/session-identity', '/api/v1/help', '/api/v1/tooltips', '/api/v1/videos'];
  const landingReads: string[] = [];
  const destinationReads = new Set<string>();
  page.on('request', request => {
    const url = new URL(request.url());
    if (request.isNavigationRequest() && request.frame() === page.mainFrame()
      && url.origin === origin && url.pathname === '/onboarding') destinationRequests.push(url.href);
    if (url.origin === origin && ['fetch', 'xhr'].includes(request.resourceType()) && startupPaths.includes(url.pathname)) {
      const documentPath = new URL(request.frame().url()).pathname;
      if (documentPath === '/share-card') landingReads.push(url.pathname);
      if (documentPath === '/onboarding') destinationReads.add(url.pathname);
    }
  });
  const response = await page.goto('/share-card', { waitUntil: 'commit' });
  expect(response?.status()).toBe(200);
  await page.waitForURL(url => url.origin === origin && url.pathname === '/onboarding', { waitUntil: 'load' });
  const document = await page.evaluateHandle(() => window.document);
  try {
    const identity = await page.request.get('/api/v1/auth/session-identity');
    expect(identity.status()).toBe(200);
    await expect(page.getByRole('button', { name: 'Log out', exact: true })).toBeVisible();
    await page.getByRole('button', { name: 'Log out', exact: true }).focus();
    expect(await page.evaluate(original => original === window.document, document)).toBe(true);
    expect(destinationRequests).toHaveLength(1);
    expect(landingReads).toEqual([]);
    // The actual destination must still initialize all normal app services.
    await expect.poll(() => [...destinationReads].sort()).toEqual(startupPaths);
  } finally { await document.dispose(); }
});
