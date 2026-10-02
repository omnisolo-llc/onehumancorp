import { test, expect } from './fixtures';
import type { Page } from '@playwright/test';

function requireLoopbackUrl(value: string) {
  const url = new URL(value);
  expect(['http:', 'https:']).toContain(url.protocol);
  expect(['localhost', '127.0.0.1', '[::1]']).toContain(url.hostname);
}

test.beforeEach(async ({ baseURL }) => {
  expect(baseURL).toBeTruthy();
  requireLoopbackUrl(baseURL!);
});

async function createRecordedInvitation(page: Page): Promise<string> {
  requireLoopbackUrl(page.url());
  const button = page.locator('#dashboard-invite-btn');
  await expect(button).toBeEnabled();
  const responsePromise = page.waitForResponse(response =>
    new URL(response.url()).pathname === '/api/v1/growth/cloud-bridge/invite'
    && response.request().method() === 'POST');
  await button.click();
  const response = await responsePromise;
  expect(response.status()).toBe(200);
  expect(response.request().postDataJSON()).toEqual({ invitee_id: 'pending' });
  const receipt: unknown = await response.json();
  if (!receipt || typeof receipt !== 'object' || Array.isArray(receipt)
    || !('invite_link' in receipt) || typeof receipt.invite_link !== 'string') {
    throw new Error('Invitation response lacks a confirmed link');
  }
  expect('success' in receipt ? receipt.success : undefined).not.toBe(false);
  expect(('error' in receipt ? receipt.error : null) ?? null).toBeNull();
  expect(receipt.invite_link).toMatch(/^https:\/\/(cloud\.)?omnisolo\.co\/invite\/[^/?#]+$/);
  expect(receipt.invite_link).not.toMatch(/\/(fallback|default)$/);
  await expect(page.locator('#dashboard-invite-link')).toHaveValue(receipt.invite_link);
  return receipt.invite_link;
}

test.describe('Recorded Invitations on Dashboard', () => {
  test('shows the real invitation action without unverified reward promises', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    const widget = page.getByTestId('dashboard-viral-invite-widget');
    await expect(widget.getByRole('heading', { name: 'Invite a Business Owner' })).toBeVisible();
    await expect(widget).not.toContainText('$50');
    await expect(widget).not.toContainText('1 month free');
    await expect(widget.getByRole('status', { name: 'Dashboard invitation status' })).toContainText('verified account');
    await expect(page.locator('#dashboard-invite-btn')).toBeEnabled();
  });

  test('displays the exact recorded link and retains the no-repeat hold after reload', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    let invitationPosts = 0;
    page.on('request', request => {
      if (new URL(request.url()).pathname === '/api/v1/growth/cloud-bridge/invite' && request.method() === 'POST') invitationPosts += 1;
    });
    const link = await createRecordedInvitation(page);
    await expect(page.locator('#dashboard-invite-link')).toHaveValue(link);
    await page.reload();
    await expect(page.getByRole('status', { name: 'Dashboard invitation status' })).toContainText('already created');
    await expect(page.locator('#dashboard-invite-btn')).toBeDisabled();
    await expect(page.locator('#dashboard-invite-link')).toHaveCount(0);
    expect(invitationPosts).toBe(1);
  });

  test('copies only the confirmed link through the actual clipboard', async ({ page, context, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    const link = await createRecordedInvitation(page);
    requireLoopbackUrl(page.url());
    await context.grantPermissions(['clipboard-read', 'clipboard-write'], { origin: new URL(page.url()).origin });
    await page.bringToFront();
    await expect.poll(() => page.evaluate(() => document.hasFocus())).toBe(true);
    await page.locator('#dashboard-copy-btn').click();
    await expect(page.locator('#dashboard-copy-btn')).toHaveText('Copied!');
    expect(await page.evaluate(() => navigator.clipboard.readText())).toBe(link);
  });

  test('exposes the exact confirmed link in a real X share-intent anchor', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    const link = await createRecordedInvitation(page);
    const share = page.getByRole('link', { name: 'Share on X', exact: true });
    await expect(share).toBeVisible();
    const href = await share.getAttribute('href');
    expect(href).not.toBeNull();
    const intent = new URL(href!);
    expect(intent.origin + intent.pathname).toBe('https://twitter.com/intent/tweet');
    expect(intent.searchParams.get('text')).toBe(`Join me on OmniSolo OneHumanCorp: ${link}`);
    await expect(share).toHaveAttribute('target', '_blank');
    await expect(share).toHaveAttribute('rel', 'noopener noreferrer');
    // The intent is prepared here; this test does not post to an external account.
  });
});
