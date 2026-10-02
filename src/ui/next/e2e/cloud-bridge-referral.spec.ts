import { test, expect } from '../../../e2e/fixtures';

test.describe('Cloud Bridge Referral Workflow', () => {
  test('shows the actual owner-bound invitation receipt and holds repeat creation after reload', async ({ page, loginAs, adminUser }, testInfo) => {
    expect(testInfo.project.use.baseURL, 'The mutation journey requires an explicit isolated local base URL').toBeTruthy();
    expect(['localhost', '127.0.0.1', '[::1]']).toContain(new URL(testInfo.project.use.baseURL!).hostname);
    await loginAs(page, adminUser);
    await page.goto('/referrals');
    expect(['localhost', '127.0.0.1', '[::1]'], 'This mutation journey requires the isolated local acceptance stack').toContain(new URL(page.url()).hostname);
    await expect(page.getByRole('heading', { name: 'Cloud Bridge Invite', exact: true })).toBeVisible();
    const verified = await page.evaluate(async () => {
      const response = await fetch('/api/v1/auth/session-identity', { cache: 'no-store' });
      return { status: response.status, body: await response.json() };
    });
    expect(verified.status).toBe(200);
    const requests: import('@playwright/test').Request[] = [];
    page.on('request', request => {
      if (request.method() === 'POST' && new URL(request.url()).pathname === '/api/v1/growth/cloud-bridge/invite') requests.push(request);
    });
    await page.locator('#cloud-bridge-email').fill('team-member@example.test');
    const pending = page.waitForResponse(response => response.request().method() === 'POST' && new URL(response.url()).pathname === '/api/v1/growth/cloud-bridge/invite');
    await page.getByRole('button', { name: 'Generate Cloud Invite', exact: true }).click();
    const response = await pending;
    expect(response.status()).toBe(200);
    const body = await response.json();
    expect(body.error).toBeUndefined();
    expect(body.invite_link).toMatch(/^https:\/\/(?:cloud\.)?omnisolo\.co\/invite\/[^/?#]+$/);
    expect(requests).toHaveLength(1);
    expect(requests[0].postDataJSON()).toEqual({ invitee_id: 'team-member@example.test' });
    const headers = await requests[0].allHeaders();
    expect(headers['x-ohc-expected-user']).toBe(verified.body.userId);
    expect(headers['x-ohc-expected-tenant']).toBe(verified.body.tenantId);
    const status = page.getByRole('status', { name: 'Cloud invitation status', exact: true });
    await expect(status).toContainText(`Cloud Invite generated: ${body.invite_link}`);
    await expect(status).toContainText('Sending or joining is not verified here.');
    await expect(page.getByRole('button', { name: 'Generate Cloud Invite', exact: true })).toBeDisabled();
    await page.reload();
    await expect(status).toContainText('An invitation was already created in this browser.');
    await expect(status).not.toContainText(body.invite_link);
    await expect(page.getByRole('button', { name: 'Generate Cloud Invite', exact: true })).toBeDisabled();
    expect(requests).toHaveLength(1);
  });
});
