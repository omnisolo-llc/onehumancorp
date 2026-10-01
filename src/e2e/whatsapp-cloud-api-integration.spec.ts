import { test, expect } from './fixtures';

test.describe('WhatsApp Cloud API availability', () => {
  test('unconfigured Meta sign-in makes no connection request or connected claim', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/integrations');
    const heading = page.getByRole('heading', { name: 'WhatsApp Cloud API', exact: true });
    await expect(heading).toBeVisible();
    const card = page.locator('div.rounded-2xl', { has: heading });
    await card.getByRole('button', { name: 'Connect', exact: true }).click();
    await expect(page.getByRole('heading', { name: 'Connect WhatsApp Cloud API' })).toBeVisible();

    // This CI journey verifies the unavailable contract. Never start a real
    // provider login if this environment unexpectedly has a configured SDK.
    const hasMetaSignIn = await page.evaluate(() => typeof (window as Window & { FB?: { login?: unknown } }).FB?.login === 'function');
    expect(hasMetaSignIn, 'This fixture must not have live Meta sign-in configured').toBe(false);
    const connectRequests: string[] = [];
    page.on('request', request => {
      if (request.method() === 'POST' && new URL(request.url()).pathname === '/api/v1/integrations/whatsapp_cloud_api/connect') connectRequests.push(request.url());
    });
    await page.getByRole('button', { name: 'Continue with Meta', exact: true }).click();
    await expect(page.getByText('WhatsApp connection is unavailable because Meta sign-in is not configured.', { exact: true })).toBeVisible();
    expect(connectRequests).toEqual([]);
    await expect(page.getByText('WhatsApp Cloud API connected.', { exact: true })).toHaveCount(0);
    await expect(card.getByRole('button', { name: 'Connect', exact: true })).toBeVisible();
    await expect(page).toHaveURL(/\/integrations$/);

    // The actual authenticated route also refuses to manufacture a verified
    // connection. An empty request carries no provider credential.
    const response = await page.evaluate(async () => {
      const reply = await fetch('/api/v1/integrations/whatsapp_cloud_api/connect', {
        method: 'POST', headers: { 'content-type': 'application/json' }, body: '{}',
      });
      return { status: reply.status, body: await reply.json() };
    });
    expect(response.status).toBe(501);
    expect(response.body).toMatchObject({ success: false, status: 'pending_verification', usable: false });
  });
});
