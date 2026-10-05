import { test, expect } from '../../../../e2e/fixtures';

// Real backend and authenticated browser fixtures only; no substituted responses.
const aliases = [
  '/changelog.html',
  '/ui/changelog.html',
  '/api/ui/changelog.html',
  '/api/v1/ui/changelog.html',
] as const;
const query = '?audit=changelog&filter=release%20notes&filter=all';
// Compare decoded entries so space encoding can change without losing duplicate order.
const queryEntries = Array.from(new URLSearchParams(query).entries());

for (const viewport of [{ width: 1440, height: 900 }, { width: 390, height: 844 }]) {
  test.describe(`changelog retirement at ${viewport.width}px`, () => {
    test.use({ viewport });

    for (const legacy of aliases) {
      test(`${legacy} retains actual changelog content`, async ({ page }, testInfo) => {
        const assetFailures: string[] = [];
        page.on('response', (response) => {
          if (response.status() >= 400 && ['script', 'stylesheet'].includes(response.request().resourceType())) {
            assetFailures.push(`${response.status()} ${response.url()}`);
          }
        });
        page.on('requestfailed', (request) => {
          if (['script', 'stylesheet'].includes(request.resourceType())) assetFailures.push(request.url());
        });
        const redirectResponse = page.waitForResponse((response) => new URL(response.url()).pathname === legacy);
        const contentResponse = page.waitForResponse((response) => new URL(response.url()).pathname === '/api/v1/changelog' && response.request().method() === 'GET');
        await page.goto(`${legacy}${query}#release-notes`, { waitUntil: 'domcontentloaded' });
        await expect(page).toHaveURL((url) => url.pathname === '/changelog' && url.hash === '#release-notes');
        expect(Array.from(new URL(page.url()).searchParams.entries())).toEqual(queryEntries);
        const redirect = await redirectResponse;
        expect(redirect.status()).toBe(307);
        expect(redirect.headers()['cache-control']).toContain('no-store');
        const response = await contentResponse;
        expect(response.status()).toBe(200);
        const sections = await response.json();
        expect(Array.isArray(sections)).toBe(true);
        expect(sections.length).toBeGreaterThan(0);
        await expect(page.getByTestId('changelog-title')).toBeVisible();
        await expect(page.getByTestId('changelog-section')).toHaveCount(sections.length);
        const first = page.getByTestId('changelog-section').first();
        await expect(first.getByRole('heading', { name: sections[0].version, exact: true })).toBeVisible();
        expect(sections[0].contentLines.length).toBeGreaterThan(0);
        const firstLine = sections[0].contentLines[0].replace(/^(### |- )/, '').replace(/\[(.*?)\]\((.*?)\)/g, '$1');
        await expect(first).toContainText(firstLine);
        // Section zero is a fixed introduction; verify an actual repository release body too.
        const releaseIndex = sections.findIndex((section: { contentLines: string[] }, index: number) => index > 0 && section.contentLines.some(line => !line.startsWith('### ')));
        expect(releaseIndex).toBeGreaterThan(0);
        const release = sections[releaseIndex];
        const releaseCard = page.getByTestId('changelog-section').nth(releaseIndex);
        await expect(releaseCard.getByRole('heading', { name: release.version, exact: true })).toBeVisible();
        const releaseLine = release.contentLines.find((line: string) => !line.startsWith('### '));
        await expect(releaseCard).toContainText(releaseLine.replace(/^- /, '').replace(/\[(.*?)\]\((.*?)\)/g, '$1'));
        await expect(page.getByText('No changelog available.', { exact: true })).toHaveCount(0);
        await expect(page.getByText('Unable to load changelog.', { exact: false })).toHaveCount(0);
        expect(assetFailures).toEqual([]);
        await testInfo.attach('canonical-changelog', { body: await page.screenshot(), contentType: 'image/png' });
      });
    }
  });
}

test.describe('changelog alias access and methods', () => {
  for (const legacy of aliases) {
    test(`anonymous ${legacy} keeps its existing authentication response`, async ({ anonymousPage }) => {
      const response = await anonymousPage.request.get(`${legacy}${query}`, { maxRedirects: 0 });
      expect(response.status()).toBe(legacy.startsWith('/api/') ? 401 : 307);
      expect(response.headers()['cache-control']).toContain('no-store');
      if (legacy.startsWith('/api/')) {
        expect(await response.json()).toEqual({ error: 'authentication required' });
      } else {
        const location = new URL(response.headers().location, response.url());
        expect(location.pathname).toBe('/login');
        const returnTo = location.searchParams.get('next') ?? '';
        expect(returnTo.startsWith(`${legacy}?`)).toBe(true);
        const next = new URL(returnTo, response.url());
        expect(next.pathname).toBe(legacy);
        expect(next.hash).toBe('');
        expect(Array.from(next.searchParams.entries())).toEqual(queryEntries);
      }
    });

    test(`authenticated HEAD ${legacy} preserves the query`, async ({ page }) => {
      const response = await page.request.head(`${legacy}${query}`, { maxRedirects: 0 });
      expect(response.status()).toBe(307);
      const location = new URL(response.headers().location, response.url());
      expect(location.pathname).toBe('/changelog');
      expect(Array.from(location.searchParams.entries())).toEqual(queryEntries);
      expect(response.headers()['cache-control']).toContain('no-store');
      expect((await response.body()).length).toBe(0);
    });
  }
});
