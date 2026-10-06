import type { Request } from '@playwright/test';
import { test, expect } from './fixtures';
import { createGrowthOwner } from './growth_owner';
import { e2eDbQuery } from './db_utils';
import { createRecordedInvitation, requireLoopbackUrl } from './support/recorded_invitation';

test.describe('Growth Cloud Bridge Loop', () => {
  test('creates and copies one owned dashboard invitation with matching share intents', async ({ page, baseURL, adminUser, loginAs }) => {
    requireLoopbackUrl(baseURL!);
    const owner = await createGrowthOwner(page, baseURL);
    await loginAs(page, { ...adminUser, email: owner.email, organizationId: owner.tenantId });
    const origin = new URL(page.url()).origin;
    const writes: Request[] = [];
    page.on('request', request => {
      const url = new URL(request.url());
      if (url.origin === origin && url.pathname === '/api/v1/growth/cloud-bridge/invite' && request.method() === 'POST') writes.push(request);
    });

    await expect(page.getByRole('heading', { name: 'Dashboard', exact: true })).toBeVisible();
    // This widget is always mounted. The separate Grow Your Team card is only
    // shown when the proposal feed is empty and is not the dashboard invitation.
    const widget = page.getByTestId('dashboard-viral-invite-widget');
    await expect(widget.getByRole('heading', { name: 'Invite a Business Owner' })).toBeVisible();
    const link = await createRecordedInvitation(page, {
      button: widget.getByRole('button', { name: 'Get My Invite Link' }),
      input: widget.getByRole('textbox', { name: 'Invitation link' }), invitee: 'pending',
    });
    expect(writes).toHaveLength(1);
    const inviteId = new URL(link).pathname.split('/').at(-1);
    const persisted = async () => e2eDbQuery(
      'SELECT id, team_id, inviter_id, invitee_id, status FROM team_invites WHERE tenant_id=$1 ORDER BY id',
      [owner.tenantId],
    );
    const expected = [{ id: inviteId, team_id: owner.tenantId, inviter_id: owner.userId, invitee_id: 'pending', status: 'PENDING' }];
    expect(await persisted()).toEqual(expected);

    await page.context().grantPermissions(['clipboard-read', 'clipboard-write'], { origin });
    await widget.getByRole('button', { name: 'Copy', exact: true }).click();
    await expect(widget.getByRole('button', { name: 'Copied!', exact: true })).toBeVisible();
    expect(await page.evaluate(() => navigator.clipboard.readText())).toBe(link);

    // Inspect the real external share URLs without sending a message or opening
    // either third-party site. A recorded invitation is not an accepted invite.
    const shareText = `Join me on OmniSolo OneHumanCorp: ${link}`;
    for (const [name, destination] of [['Share on WhatsApp', 'https://wa.me/'], ['Share on X', 'https://twitter.com/intent/tweet']]) {
      const share = widget.getByRole('link', { name, exact: true });
      await expect(share).toBeVisible();
      const url = new URL((await share.getAttribute('href'))!);
      expect(`${url.origin}${url.pathname}`).toBe(destination);
      expect(url.searchParams.get('text')).toBe(shareText);
    }
    await expect(widget.getByRole('status', { name: 'Dashboard invitation status' })).toContainText('Sending or joining is not verified here.');

    await page.reload();
    await expect(widget.getByRole('button', { name: 'Get My Invite Link' })).toBeDisabled();
    await expect(widget.getByRole('status', { name: 'Dashboard invitation status' })).toContainText('already created');
    await expect(widget.getByRole('textbox', { name: 'Invitation link' })).toHaveCount(0);
    expect(writes).toHaveLength(1);
    expect(await persisted()).toEqual(expected);
  });
});
