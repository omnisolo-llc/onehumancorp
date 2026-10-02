import { test, expect } from './fixtures';
import { createRecordedInvitation, requireLoopbackUrl } from './support/recorded_invitation';

test.beforeEach(async ({ baseURL }) => {
  expect(baseURL).toBeTruthy();
  requireLoopbackUrl(baseURL!);
});

test.describe('Recorded referral invitations', () => {
  test('displays the verified dashboard invitation and copies its exact receipt link', async ({ page, context, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');
    const widget = page.getByTestId('dashboard-viral-invite-widget');
    await expect(widget.getByRole('heading', { name: 'Invite a Business Owner', exact: true })).toBeVisible();
    await expect(widget).not.toContainText('$50');
    await expect(widget).not.toContainText('1 month free');
    const link = await createRecordedInvitation(page);
    const copy = widget.locator('#dashboard-copy-btn');
    await expect(widget.locator('#dashboard-share-x-btn')).toBeVisible();
    requireLoopbackUrl(page.url());
    await context.grantPermissions(['clipboard-read', 'clipboard-write'], { origin: new URL(page.url()).origin });
    await page.bringToFront();
    await expect.poll(() => page.evaluate(() => document.hasFocus())).toBe(true);
    await copy.click();
    await expect(copy).toHaveText('Copied!');
    expect(await page.evaluate(() => navigator.clipboard.readText())).toBe(link);
  });

  test('creates the exact recorded team invitation link', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/team');
    await expect(page.getByRole('heading', { name: 'Grow Your Team', exact: true })).toBeVisible();
    const input = page.locator('#cloud-bridge-invite-link');
    const link = await createRecordedInvitation(page, {
      button: page.getByRole('button', { name: 'Unlock Cloud Collaboration', exact: true }),
      input,
      invitee: 'pending-invite',
    });
    await expect(input).toHaveValue(link);
  });
});
