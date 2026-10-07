import { test, expect } from './fixtures';
import { createRecordedOrderOwner, captureRecordedOrders } from './support/recorded_order_fixture';
import { requireLoopbackUrl } from './support/recorded_invitation';
import { observeMilestoneInvitationAuditClick } from './support/milestone_invitation_audit';

test.describe('Success Milestone Widget', () => {
  test('the owned click audit scrolls to one real invitation and verifies its persisted record and mounted share preview', async ({ page, baseURL }) => {
    const owner = await createRecordedOrderOwner(page, baseURL, 1);
    const reading = captureRecordedOrders(page, owner);
    await page.goto('/dashboard'); await reading;
    const widget = page.getByRole('region', { name: 'Recorded order milestone', exact: true });
    const button = widget.getByRole('button', { name: 'Create milestone invitation', exact: true });
    await expect(button).toBeEnabled();
    await page.evaluate(() => window.scrollTo(0, 0));
    const target = await button.elementHandle();
    expect(target).not.toBeNull();
    expect(await target!.evaluate(element => element.getBoundingClientRect().top >= element.ownerDocument.defaultView!.innerHeight)).toBe(true);
    const effect = await observeMilestoneInvitationAuditClick(page, target!, owner, async () => {
      throw new Error('The owned milestone control must use its persisted invitation proof');
    });
    expect(effect.changed).toBe(true);
    expect(effect.requestSeen).toBe(true);
  });

  test('shares only the recorded owned aggregate with a confirmed link and actual clipboard acknowledgement', async ({ page, baseURL }) => {
    const owner = await createRecordedOrderOwner(page, baseURL, 101);
    const reading = captureRecordedOrders(page, owner);
    await page.goto('/dashboard'); await reading;
    const widget = page.getByRole('region', { name: 'Recorded order milestone', exact: true });
    await expect(widget.getByText('101 recorded orders', { exact: true })).toBeVisible();
    await expect(widget.getByText('Recorded-order milestone: 100', { exact: true })).toBeVisible();
    await expect(widget.getByRole('link', { name: 'Share on X', exact: true })).toHaveCount(0);
    requireLoopbackUrl(page.url()); expect(new URL(page.url()).origin).toBe(owner.origin);
    const pending = page.waitForResponse(response => new URL(response.url()).origin === owner.origin
      && new URL(response.url()).pathname === '/api/v1/growth/cloud-bridge/invite' && response.request().method() === 'POST');
    await widget.getByRole('button', { name: 'Create milestone invitation', exact: true }).click();
    const response = await pending;
    expect(response.status()).toBe(200); expect(response.request().postDataJSON()).toEqual({ invitee_id: 'pending-invite' });
    expect(response.request().headers()['x-ohc-expected-user']).toBe(owner.userId);
    expect(response.request().headers()['x-ohc-expected-tenant']).toBe(owner.tenantId);
    const receipt = await response.json(); expect(receipt.error ?? null).toBeNull(); expect([undefined,true]).toContain(receipt.success);
    expect(receipt.invite_link).toMatch(/^https:\/\/(cloud\.)?omnisolo\.co\/invite\/[^/?#]+$/);
    expect(receipt.invite_link).not.toMatch(/\/(fallback|default)$/);
    const text = `We've recorded 101 orders in OmniSolo. ${receipt.invite_link}`;
    await expect(widget.getByLabel('Milestone share preview')).toHaveValue(text);
    const twitter = new URL((await widget.getByRole('link', { name: 'Share on X', exact: true }).getAttribute('href'))!);
    expect(twitter.origin + twitter.pathname).toBe('https://twitter.com/intent/tweet'); expect(twitter.searchParams.get('text')).toBe(text);
    requireLoopbackUrl(page.url()); expect(new URL(page.url()).origin).toBe(owner.origin);
    await page.context().grantPermissions(['clipboard-read','clipboard-write'], { origin: owner.origin });
    await page.bringToFront();
    await widget.getByRole('button', { name: 'Copy milestone share text', exact: true }).click();
    await expect(widget.getByRole('status', { name: 'Milestone clipboard' })).toHaveText(/copied/i);
    expect(await page.evaluate(() => navigator.clipboard.readText())).toBe(text);
  });
});
