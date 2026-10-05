import { test, expect } from './fixtures';
import { expectEntitlementUnchanged, trackTrialClaims } from './support/entitlement_fixture';
import { trialUrlParts, useTrialSeedOwner } from './support/trial_retirement_fixture';

const aliases = ['/trial-extension.html', '/ui/trial-extension.html'] as const;
const query = '?source=legacy%20plan&tag=one&tag=two';

for (const viewport of [{ width: 1440, height: 900 }, { width: 390, height: 844 }]) {
  test.describe(`trial alias retirement at ${viewport.width}px`, () => {
    test.use({ viewport });

    for (const legacy of aliases) {
      test(`${legacy} preserves its query and reaches the real plan screen`, async ({ page, baseURL, loginAs }, testInfo) => {
        const fixture = await useTrialSeedOwner(page, loginAs);
        const claims = trackTrialClaims(page);
        const failedAssets: string[] = [];
        page.on('response', response => {
          if (response.status() >= 400 && ['script', 'stylesheet'].includes(response.request().resourceType())) failedAssets.push(response.url());
        });
        page.on('requestfailed', request => {
          if (['script', 'stylesheet'].includes(request.resourceType())) failedAssets.push(request.url());
        });
        const redirectResponse = page.waitForResponse(response => new URL(response.url()).pathname === legacy);
        const origin = new URL('/trial-extension', baseURL).origin;
        const planResponse = page.waitForResponse(response => {
          const request = response.request();
          const url = new URL(response.url());
          // loginAs visits /dashboard, which can still have a plan read in flight.
          // The request's Referer stays fixed even after its frame navigates away.
          const referer = request.headers()['referer'];
          const documentUrl = referer ? new URL(referer) : null;
          return url.origin === origin && url.pathname === '/api/v1/billing/my-plan'
            && request.method() === 'GET' && documentUrl?.origin === origin
            && documentUrl.pathname === '/trial-extension';
        });
        await page.goto(`${legacy}${query}#availability`, { waitUntil: 'domcontentloaded' });
        await expect.poll(() => trialUrlParts(new URL(page.url())))
          .toEqual(trialUrlParts(new URL(`/trial-extension${query}#availability`, baseURL)));
        const redirect = await redirectResponse;
        expect(redirect.status()).toBe(307);
        expect(redirect.headers()['cache-control']).toContain('no-store');
        const response = await planResponse;
        expect(response.status()).toBe(200);
        expect((await response.json()).current_plan.toLowerCase()).toBe('free');
        await expect(page.getByRole('heading', { name: 'Plan and Trial Availability' })).toBeVisible();
        await expect(page.getByRole('status', { name: 'Current plan' })).toHaveText('Current verified plan: Free.');
        await page.getByRole('button', { name: 'Check trial availability' }).click();
        await expect(page.getByText(/durable grant is not verified/)).toBeVisible();
        expect(claims).toEqual([]);
        expect(failedAssets).toEqual([]);
        await expectEntitlementUnchanged(page, fixture);
        await testInfo.attach('canonical-trial-availability', { body: await page.screenshot(), contentType: 'image/png' });
      });
    }
  });
}

for (const legacy of aliases) {
  test(`anonymous ${legacy} retains its login return path`, async ({ anonymousPage }) => {
    const response = await anonymousPage.request.get(`${legacy}${query}`, { maxRedirects: 0 });
    expect(response.status()).toBe(307);
    const destination = new URL(response.headers().location, response.url());
    expect(destination.origin).toBe(new URL(response.url()).origin);
    expect(destination.pathname).toBe('/login');
    expect([...destination.searchParams.keys()]).toEqual(['next']);
    const returnPath = new URL(destination.searchParams.get('next')!, response.url());
    expect(trialUrlParts(returnPath)).toEqual(trialUrlParts(new URL(`${legacy}${query}`, response.url())));
    expect(response.headers()['cache-control']).toContain('no-store');
  });

  test(`${legacy} redirects HEAD and rejects cross-origin mutation`, async ({ page, loginAs }) => {
    const fixture = await useTrialSeedOwner(page, loginAs);
    const head = await page.request.head(`${legacy}${query}`, { maxRedirects: 0 });
    expect(head.status()).toBe(307);
    const destination = new URL(head.headers().location, head.url());
    expect(trialUrlParts(destination)).toEqual(trialUrlParts(new URL(`/trial-extension${query}`, head.url())));
    expect(head.headers()['cache-control']).toContain('no-store');
    const post = await page.request.post(legacy, {
      maxRedirects: 0,
      headers: { origin: 'https://untrusted.example.test' },
      data: { source: 'retirement-contract' },
    });
    expect(post.status()).toBe(403);
    expect(post.headers().location).toBeUndefined();
    await expectEntitlementUnchanged(page, fixture);
  });
}
