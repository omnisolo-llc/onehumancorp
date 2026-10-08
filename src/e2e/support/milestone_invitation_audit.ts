import assert from 'node:assert/strict';
import type { ElementHandle, Page, Request } from '@playwright/test';
import { e2eDbQuery } from '../db_utils';
import { requireLoopbackUrl } from './recorded_invitation';
import type { ClickEffects } from './ui_click_audit';

/** Read-only activation evidence; synthetic clicks and other controls do not qualify. */
export function installMilestoneInvitationClickProbe(element: HTMLElement | SVGElement) {
  const view = element.ownerDocument.defaultView!;
  let clicks = 0;
  let startedAt = 0;
  const capture = (event: MouseEvent) => {
    if (!event.isTrusted || !event.composedPath().includes(element)) return;
    clicks += 1;
    if (clicks === 1) startedAt = view.performance.timeOrigin + view.performance.now();
  };
  view.addEventListener('click', capture, true);
  return { get clicks() { return clicks; }, get startedAt() { return startedAt; },
    dispose() { view.removeEventListener('click', capture, true); } };
}

/** Native scrolling is safe here only because exact owned mutation evidence is mandatory. */
export async function observeMilestoneInvitationAuditClick(
  page: Page,
  target: ElementHandle<HTMLElement | SVGElement>,
  owner: { userId: string; tenantId: string },
  observe: () => Promise<ClickEffects>,
): Promise<ClickEffects> {
  const milestoneControl = await target.evaluate(element => element.tagName === 'BUTTON'
    && element.textContent?.trim() === 'Create milestone invitation'
    && element.closest('section[aria-label="Recorded order milestone"]') !== null
    && element.ownerDocument.defaultView === element.ownerDocument.defaultView?.top);
  if (!milestoneControl) return observe();
  requireLoopbackUrl(page.url());
  const location = new URL(page.url());
  assert.ok(!location.username && !location.password);
  assert.ok(['/', '/dashboard'].includes(location.pathname));
  const persisted = () => e2eDbQuery(
    'SELECT id, tenant_id, team_id, inviter_id, invitee_id, status FROM team_invites WHERE tenant_id=$1 ORDER BY id',
    [owner.tenantId],
  );
  assert.deepEqual(await persisted(), [], 'The isolated invitation case must start without invitations');
  const counts = await e2eDbQuery('SELECT count(*)::int AS recorded_orders FROM orders WHERE tenant_id=$1', [owner.tenantId]);
  assert.equal(counts.length, 1);
  const count = counts[0].recorded_orders;
  assert.ok(Number.isSafeInteger(count) && count > 0, 'The invitation must have an actual owned recorded-order milestone');
  const isPost = (request: Request) => {
    const url = new URL(request.url());
    return url.origin === location.origin && url.pathname === '/api/v1/growth/cloud-bridge/invite'
      && !url.search && request.method() === 'POST';
  };
  const posts: Request[] = [];
  const onRequest = (request: Request) => { if (isPost(request)) posts.push(request); };
  const clickProbe = await target.evaluateHandle(installMilestoneInvitationClickProbe);
  page.on('request', onRequest);
  try {
    const responsePromise = page.waitForResponse(response => isPost(response.request()), { timeout: 5000 });
    void responsePromise.catch(() => undefined);
    // Exactly one genuine action, with native scrolling and all actionability
    // checks. Scroll-only DOM changes or unrelated requests earn no credit.
    await target.click({ timeout: 5000 });
    const response = await responsePromise;
    assert.equal(response.status(), 200);
    const request = response.request();
    const activation = await clickProbe.evaluate(probe => ({ clicks: probe.clicks, startedAt: probe.startedAt }));
    assert.equal(activation.clicks, 1, 'The invitation requires exactly one genuine activation of this control');
    assert.ok(Number.isFinite(activation.startedAt) && activation.startedAt > 0);
    const requestStartedAt = request.timing().startTime;
    assert.ok(Number.isFinite(requestStartedAt) && requestStartedAt >= activation.startedAt,
      'An invitation POST started before the trusted click cannot certify that control');
    assert.equal(request.headers()['x-ohc-expected-user'], owner.userId);
    assert.equal(request.headers()['x-ohc-expected-tenant'], owner.tenantId);
    assert.deepEqual(request.postDataJSON(), { invitee_id: 'pending-invite' });
    const receipt: unknown = await response.json();
    assert.ok(receipt && typeof receipt === 'object' && !Array.isArray(receipt));
    const body = receipt as Record<string, unknown>;
    assert.ok(body.error == null && (body.success === undefined || body.success === true));
    assert.equal(typeof body.invite_link, 'string');
    const link = body.invite_link as string;
    const invite = new URL(link);
    assert.ok(['https://omnisolo.co', 'https://cloud.omnisolo.co'].includes(invite.origin)
      && !invite.username && !invite.password && !invite.search && !invite.hash
      && /^\/invite\/[^/]+$/.test(invite.pathname) && !/\/(fallback|default)$/.test(invite.pathname));
    const id = invite.pathname.slice('/invite/'.length);
    const expected = [{ id, tenant_id: owner.tenantId, team_id: owner.tenantId,
      inviter_id: owner.userId, invitee_id: 'pending-invite', status: 'PENDING' }];
    assert.deepEqual(await persisted(), expected);
    const widget = page.getByRole('region', { name: 'Recorded order milestone', exact: true });
    const text = `We've recorded ${count} orders in OmniSolo. ${link}`;
    const preview = widget.getByLabel('Milestone share preview', { exact: true });
    await preview.waitFor({ state: 'visible', timeout: 5000 });
    assert.equal(await preview.inputValue(), text);
    await widget.getByText(`${count} recorded orders`, { exact: true }).waitFor({ state: 'visible', timeout: 5000 });
    const validThresholds = [1, 10, 50, 100, 1000].filter(value => count >= value);
    const threshold = validThresholds[validThresholds.length - 1];
    await widget.getByText(`Recorded-order milestone: ${threshold}`, { exact: true }).waitFor({ state: 'visible', timeout: 5000 });
    for (const [name, destination] of [['Share on X', 'https://twitter.com/intent/tweet'], ['Share to WhatsApp', 'https://wa.me/']]) {
      const expectedUrl = new URL(destination); expectedUrl.searchParams.set('text', text);
      const share = widget.getByRole('link', { name, exact: true });
      await share.waitFor({ state: 'visible', timeout: 5000 });
      assert.equal(await share.getAttribute('href'), expectedUrl.href);
    }
    assert.equal(posts.length, 1, 'The owned invitation must dispatch exactly one POST');
    assert.equal(posts[0], request, 'The persisted receipt must match the observed invitation POST');
    return { changed: true, requestSeen: true, downloadSeen: false, fileChooserSeen: false,
      popupSeen: false, validationSeen: false, dialogSeen: false, decisionSeen: false };
  } finally {
    page.off('request', onRequest);
    await clickProbe.evaluate(probe => probe.dispose()).catch(() => undefined);
    await clickProbe.dispose();
  }
}
