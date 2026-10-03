import { test, expect } from './fixtures';
import { createRecordedInvitation, requireLoopbackUrl } from './support/recorded_invitation';

test.beforeEach(async ({ baseURL }) => {
  expect(baseURL).toBeTruthy();
  requireLoopbackUrl(baseURL!);
});

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
    const share = page.getByTestId('dashboard-viral-invite-widget').getByRole('link', { name: 'Share on X', exact: true });
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
