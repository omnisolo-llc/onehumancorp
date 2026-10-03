import { test, expect } from './fixtures';

test('share preview waits for hydration and commits one real destination document', async ({ page, baseURL }) => {
  if (!baseURL) throw new Error('The actual test application origin is required');
  const origin = new URL(baseURL).origin;
  let release!: () => void;
  const scripts = new Promise<void>(resolve => { release = resolve; });
  const destinationRequests: string[] = [];
  page.on('request', request => {
    const url = new URL(request.url());
    if (request.isNavigationRequest() && request.frame() === page.mainFrame()
      && url.origin === origin && url.pathname === '/onboarding') destinationRequests.push(url.href);
  });
  // Hold only the application's genuine static JavaScript. No API response,
  // authentication result or destination HTML is replaced by this barrier.
  await page.route(url => url.origin === origin && url.pathname.startsWith('/_next/static/') && url.pathname.endsWith('.js'), async route => {
    await scripts;
    await route.continue();
  });
  try {
    const response = await page.goto('/share-card', { waitUntil: 'commit' });
    expect(response?.status()).toBe(200);
    await response?.finished();
    await expect(page.getByRole('link', { name: '/onboarding', exact: true })).toBeVisible();
    await expect(page.locator('meta[property="og:title"]')).toHaveAttribute('content', 'OmniSolo');
    await expect(page.locator('meta[name="twitter:card"]')).toHaveAttribute('content', 'summary_large_image');
    expect(destinationRequests).toEqual([]);
    release();
    await page.waitForURL(url => url.origin === origin && url.pathname === '/onboarding', { waitUntil: 'load' });
    const document = await page.evaluateHandle(() => window.document);
    const identity = await page.request.get('/api/v1/auth/session-identity');
    expect(identity.status()).toBe(200);
    await expect(page.getByRole('button', { name: 'Log out', exact: true })).toBeVisible();
    await page.getByRole('button', { name: 'Log out', exact: true }).focus();
    expect(await page.evaluate(original => original === window.document, document)).toBe(true);
    expect(destinationRequests).toHaveLength(1);
    await document.dispose();
  } finally { release(); await page.unrouteAll({ behavior: 'wait' }); }
});
