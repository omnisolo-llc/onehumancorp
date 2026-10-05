import { readFile } from 'node:fs/promises';
import { createServer, type ServerResponse } from 'node:http';
import { resolve } from 'node:path';
import { test, expect } from './fixtures';

const roots = ['src/ui/tauri/src/ui', 'src/ui/next/public', 'src/ui/next/public/ui', 'src/ui/next/public/api/ui', 'src/ui/next/public/api/v1/ui'];

// These isolated browser contracts serve the exact shipped HTML with explicit
// fixture data. The separate backend test below verifies the deployed entry.
for (const root of roots) {
  test(`isolated Help source ${root} preserves a search during initial loading`, async ({ page }) => {
    const html = await readFile(resolve(root, 'help.html'));
    const module = await readFile(resolve('src/ui/tauri/src/ui/safe-help-content.mjs'));
    const current = { title: 'Current product guide', desc: 'Matching guide', category: 'Guides', link: '/help/current' };
    let articles: ServerResponse | undefined;
    let videos: ServerResponse | undefined;
    const json = (response: ServerResponse, value: unknown) => {
      response.writeHead(200, { 'content-type': 'application/json' });
      response.end(JSON.stringify(value));
    };
    const server = createServer((request, response) => {
      const url = new URL(request.url || '/', 'http://fixture.invalid');
      if (url.pathname === '/api/v1/ui/help.html') {
        response.writeHead(200, { 'content-type': 'text/html' }); response.end(html);
      } else if (url.pathname === '/api/v1/ui/safe-help-content.mjs') {
        response.writeHead(200, { 'content-type': 'text/javascript' }); response.end(module);
      } else if (url.pathname === '/api/v1/help') articles = response;
      else if (url.pathname === '/api/v1/videos') videos = response;
      else if (url.pathname === '/api/v1/help/search' && url.searchParams.get('q') === 'product') json(response, [current]);
      else if (url.pathname === '/api/v1/tooltips') json(response, {});
      else { response.writeHead(404); response.end(); }
    });
    await new Promise<void>((resolve, reject) => { server.once('error', reject); server.listen(0, '127.0.0.1', resolve); });
    try {
      const address = server.address();
      if (!address || typeof address === 'string') throw new Error('Missing Help fixture address');
      await page.goto(`http://127.0.0.1:${address.port}/api/v1/ui/help.html`, { waitUntil: 'domcontentloaded' });
      await expect.poll(() => Boolean(articles && videos)).toBe(true);
      await expect(page.locator('#loading-state')).toBeVisible();
      const search = page.locator('#search-input');
      const results = page.locator('#results');
      await search.fill('product');
      await expect(results.getByRole('heading', { name: current.title, exact: true })).toBeVisible();
      json(articles!, [current, { title: 'Unrelated invoice guide', desc: 'Another guide', category: 'Guides', link: '/help/unrelated' }]);
      json(videos!, [
        { title: 'Product tutorial', duration: '1:00', video_url: 'https://cdn.example/product.mp4' },
        { title: 'Unrelated invoice tutorial', duration: '2:00', video_url: 'https://cdn.example/invoice.mp4' },
      ]);
      await expect(page.locator('#loading-state')).toBeHidden();
      await expect(results.getByRole('button', { name: /Product tutorial/ })).toBeVisible();
      await expect(search).toHaveValue('product');
      await expect(results.getByRole('heading', { name: current.title, exact: true })).toBeVisible();
      await expect(results).not.toContainText('Unrelated invoice');
    } finally {
      server.closeAllConnections();
      await new Promise<void>((resolve, reject) => server.close(error => error ? reject(error) : resolve()));
    }
  });
}

test('real Help backend preserves the current search when its initial response arrives late', async ({ page }, testInfo) => {
  // Delay only delivery of the genuine browser fetch Response. Do not replace
  // its status, headers, body, credentials, or the shared no-stubbing fixture.
  await page.addInitScript(() => {
    const originalFetch = window.fetch.bind(window);
    let release!: () => void;
    const gate = new Promise<void>(resolve => { release = resolve; });
    const barrier = { held: false, release };
    Object.assign(window, { __ohcHelpInitialResponseBarrier: barrier });
    window.fetch = async (...args) => {
      const response = await originalFetch(...args);
      const input = args[0];
      const url = new URL(input instanceof Request ? input.url : String(input), window.location.href);
      if (!barrier.held && url.origin === window.location.origin && url.pathname === '/api/v1/help') {
        window.fetch = originalFetch;
        barrier.held = true;
        // Drain a real clone so Playwright can read the completed transfer
        // while the application's original Response remains behind the gate.
        await response.clone().arrayBuffer();
        await gate;
      }
      return response;
    };
  });
  const initialResponse = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/help' && response.request().method() === 'GET');
  await page.goto('/api/v1/ui/help.html', { waitUntil: 'domcontentloaded' });
  await expect(page).toHaveURL(/\/api\/v1\/ui\/help\.html$/);
  const initial = await initialResponse;
  expect(initial.status()).toBe(200);
  const initialArticles = await initial.json() as { title: string; desc: string; category: string; link: string }[];
  expect(initialArticles.length).toBeGreaterThan(1);
  await page.waitForFunction(() => (window as unknown as { __ohcHelpInitialResponseBarrier: { held: boolean } }).__ohcHelpInitialResponseBarrier.held);
  await expect(page.locator('#loading-state')).toBeVisible();

  const query = initialArticles[0].title.toLowerCase();
  const searchResponse = page.waitForResponse(response => {
    const url = new URL(response.url());
    return url.pathname === '/api/v1/help/search' && url.searchParams.get('q') === query;
  });
  const search = page.locator('#search-input');
  const results = page.locator('#results');
  await search.fill(query);
  const searched = await searchResponse;
  expect(searched.status()).toBe(200);
  const matchingArticles = await searched.json() as typeof initialArticles;
  expect(matchingArticles.length).toBeGreaterThan(0);
  expect(matchingArticles.length).toBeLessThan(initialArticles.length);
  const expectedTitles = matchingArticles.map(article => article.title);
  await expect(results.locator('h4')).toHaveText(expectedTitles);
  await expect(page.locator('#loading-state')).toBeVisible();
  await testInfo.attach('help-search-before-initial-release', { body: await page.screenshot(), contentType: 'image/png' });

  const refreshedResponse = page.waitForResponse(response => {
    const url = new URL(response.url());
    return url.pathname === '/api/v1/help/search' && url.searchParams.get('q') === query;
  });
  await page.evaluate(() => (window as unknown as { __ohcHelpInitialResponseBarrier: { release: () => void } }).__ohcHelpInitialResponseBarrier.release());
  const refreshed = await refreshedResponse;
  expect(refreshed.status()).toBe(200);
  const refreshedArticles = await refreshed.json() as typeof initialArticles;
  expect(refreshedArticles.map(article => article.title)).toEqual(expectedTitles);
  await expect(page.locator('#loading-state')).toBeHidden();
  await expect(search).toHaveValue(query);
  await expect(results.locator('h4')).toHaveText(expectedTitles);
  await testInfo.attach('help-search-after-initial-release', { body: await page.screenshot(), contentType: 'image/png' });
  await testInfo.attach('real-help-search-receipt', {
    body: JSON.stringify({ initialStatus: initial.status(), searchStatus: searched.status(), refreshedSearchStatus: refreshed.status(), query, initialTitles: initialArticles.map(article => article.title), expectedTitles }),
    contentType: 'application/json',
  });
});

test('canonical Help ignores an obsolete initial response after current search results render', async ({ page }, testInfo) => {
  await page.addInitScript(() => {
    const originalFetch = window.fetch.bind(window);
    let releaseGate!: () => void;
    const gate = new Promise<void>(resolve => { releaseGate = resolve; });
    const barrier = {
      held: 0, bodiesSettled: 0, released: false,
      release: () => { barrier.released = true; window.fetch = originalFetch; releaseGate(); },
    };
    Object.assign(window, { __ohcHelpInitialResponseBarrier: barrier });
    window.fetch = async (...args) => {
      const input = args[0];
      const url = new URL(input instanceof Request ? input.url : String(input), window.location.href);
      const hold = !barrier.released && url.origin === window.location.origin && url.pathname === '/api/v1/help';
      // The page and the shared widget both request initial Help data. Hold
      // every initial response so the test cannot accidentally delay only the widget.
      if (hold) barrier.held += 1;
      const response = await originalFetch(...args);
      if (hold) {
        await response.clone().arrayBuffer();
        const readBody = response.json.bind(response);
        response.json = async () => {
          try { return await readBody(); }
          finally {
            // Observe the real body consumption, then allow its application
            // continuation and rendering frames before asserting no stale UI.
            requestAnimationFrame(() => requestAnimationFrame(() => { barrier.bodiesSettled += 1; }));
          }
        };
        await gate;
      }
      return response;
    };
  });
  const initialResponse = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/help' && response.request().method() === 'GET');
  await page.goto('/help', { waitUntil: 'domcontentloaded' });
  await expect(page).toHaveURL(/\/help$/);
  const initial = await initialResponse;
  expect(initial.status()).toBe(200);
  const initialArticles = await initial.json() as { title: string; desc: string; category: string; link: string }[];
  expect(initialArticles.length).toBeGreaterThan(1);
  const article = initialArticles.find(article => article.category !== 'Advanced');
  expect(article).toBeDefined();
  const query = article!.title.toLowerCase();
  await page.waitForFunction(() => (window as unknown as { __ohcHelpInitialResponseBarrier: { held: number } }).__ohcHelpInitialResponseBarrier.held > 0);

  const searchResponse = page.waitForResponse(response => {
    const url = new URL(response.url());
    return url.pathname === '/api/v1/help/search' && url.searchParams.get('q') === query;
  });
  const search = page.getByTestId('help-search-input');
  const titles = page.locator('a[href^="/help/"] h3');
  await expect(titles).toHaveCount(0);
  await search.fill(query);
  const searched = await searchResponse;
  expect(searched.status()).toBe(200);
  const matchingArticles = await searched.json() as typeof initialArticles;
  const expectedTitles = matchingArticles.filter(article => article.category !== 'Advanced').map(article => article.title);
  expect(expectedTitles.length).toBeGreaterThan(0);
  expect(expectedTitles.length).toBeLessThan(initialArticles.filter(article => article.category !== 'Advanced').length);
  await expect(titles).toHaveText(expectedTitles);
  expect(await page.evaluate(() => (window as unknown as { __ohcHelpInitialResponseBarrier: { bodiesSettled: number } }).__ohcHelpInitialResponseBarrier.bodiesSettled)).toBe(0);
  await testInfo.attach('canonical-help-before-initial-release', { body: await page.screenshot(), contentType: 'image/png' });

  await page.evaluate(() => (window as unknown as { __ohcHelpInitialResponseBarrier: { release: () => void } }).__ohcHelpInitialResponseBarrier.release());
  await page.waitForFunction(() => {
    const barrier = (window as unknown as { __ohcHelpInitialResponseBarrier: { held: number; bodiesSettled: number } }).__ohcHelpInitialResponseBarrier;
    return barrier.held > 0 && barrier.bodiesSettled === barrier.held;
  });
  await expect(search).toHaveValue(query);
  await expect(titles).toHaveText(expectedTitles);
  await expect(page.getByText('Something went wrong loading the help center.', { exact: true })).toHaveCount(0);
  await testInfo.attach('canonical-help-after-initial-release', { body: await page.screenshot(), contentType: 'image/png' });
  await testInfo.attach('canonical-help-search-receipt', {
    body: JSON.stringify({ initialStatus: initial.status(), searchStatus: searched.status(), obsoleteBodiesConsumed: await page.evaluate(() => (window as unknown as { __ohcHelpInitialResponseBarrier: { bodiesSettled: number } }).__ohcHelpInitialResponseBarrier.bodiesSettled), query, initialTitles: initialArticles.map(article => article.title), expectedTitles }),
    contentType: 'application/json',
  });
});
