import { test, expect } from './fixtures';
import { createMarketplaceOwner } from './marketplace_fixtures';

test.describe('Unlock Pro Features referral progress', () => {
  test('creates one recorded invitation and copies its confirmed link without claiming entitlement', async ({ page, context, baseURL }) => {
    const owner = await createMarketplaceOwner(page, baseURL);
    const origin = new URL(baseURL!).origin;
    await context.grantPermissions(['clipboard-read', 'clipboard-write'], { origin });
    const initialRead = page.waitForResponse(response => new URL(response.url()).origin === origin && new URL(response.url()).pathname === '/api/v1/growth/team-invites/aggregated-metrics' && response.request().method() === 'GET' && response.request().headers()['x-ohc-expected-user'] === owner.userId && response.request().headers()['x-ohc-expected-tenant'] === owner.tenantId)
      .then(async response => ({ status: response.status(), body: await response.json(), headers: response.request().headers() }));
    await page.goto('/dashboard');
    const widget = page.getByTestId('unlock-pro-features-widget');
    await expect(widget).toBeVisible();
    await expect(widget.getByRole('heading', { name: /Unlock Pro Features/ })).toBeVisible();
    const initial = await initialRead;
    expect(initial.status).toBe(200); expect(initial.body.total_invites).toBe(0);
    expect(initial.headers['x-ohc-expected-user']).toBe(owner.userId);
    expect(initial.headers['x-ohc-expected-tenant']).toBe(owner.tenantId);
    await expect(widget).toHaveAttribute('aria-busy', 'false');
    await expect(widget.getByRole('progressbar', { name: 'Recorded invitation progress' })).toHaveAttribute('aria-valuenow', '0');
    await expect(widget.getByRole('button', { name: 'Copy Invite Link' })).toHaveCount(0);
    const creating = page.waitForResponse(response => new URL(response.url()).origin === origin && new URL(response.url()).pathname === '/api/v1/growth/cloud-bridge/invite' && response.request().method() === 'POST')
      .then(async response => ({ status: response.status(), body: await response.json(), headers: response.request().headers() }));
    const refresh = page.waitForResponse(response => new URL(response.url()).origin === origin && new URL(response.url()).pathname === '/api/v1/growth/team-invites/aggregated-metrics' && response.request().method() === 'GET' && response.request().headers()['x-ohc-expected-user'] === owner.userId && response.request().headers()['x-ohc-expected-tenant'] === owner.tenantId)
      .then(async response => ({ status: response.status(), body: await response.json() }));
    await widget.getByRole('button', { name: 'Create Invite Link', exact: true }).click();
    const receipt = await creating;
    expect(receipt.status).toBe(200); expect(receipt.body.error).toBeUndefined(); expect(receipt.body.success).not.toBe(false);
    expect(receipt.headers['x-ohc-expected-user']).toBe(owner.userId); expect(receipt.headers['x-ohc-expected-tenant']).toBe(owner.tenantId);
    expect(receipt.body.invite_link).toMatch(/^https:\/\/(cloud\.)?omnisolo\.co\/invite\/[^/?#]+$/);
    const progress = await refresh;
    expect(progress.status).toBe(200); expect(progress.body.total_invites).toBe(1);
    await expect(widget).toHaveAttribute('aria-busy', 'false');
    await expect(widget.getByRole('progressbar', { name: 'Recorded invitation progress' })).toHaveAttribute('aria-valuenow', '1');
    await page.bringToFront();
    await widget.getByRole('button', { name: 'Copy Invite Link', exact: true }).click();
    await expect(widget.getByRole('button', { name: 'Copied Link!', exact: true })).toBeVisible();
    expect(await page.evaluate(() => navigator.clipboard.readText())).toBe(receipt.body.invite_link);
    const share = new URL((await widget.getByRole('link', { name: 'Share on X', exact: true }).getAttribute('href'))!);
    expect(share.origin).toBe('https://twitter.com'); expect(share.searchParams.get('text')).toContain(receipt.body.invite_link);
    await expect(widget.getByRole('status', { name: 'Referral progress status' })).toContainText('Acceptance and Pro entitlement are not verified by this count');
    await expect(widget.getByText('Invite target reached', { exact: true })).toHaveCount(0);
    await page.reload();
    await expect(widget).toHaveAttribute('aria-busy', 'false');
    await expect(widget.getByRole('progressbar', { name: 'Recorded invitation progress' })).toHaveAttribute('aria-valuenow', '1');
    await expect(widget.getByRole('button', { name: 'Create Invite Link', exact: true })).toBeDisabled();
    await expect(widget.getByRole('status', { name: 'Referral invitation status' })).toContainText('already created');
  });
});
