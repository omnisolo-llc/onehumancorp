import { EventEmitter } from 'node:events';
import { afterEach, expect, it, vi } from 'vitest';
import type { ElementHandle, Page } from '@playwright/test';
import { e2eDbQuery } from '../../../../e2e/db_utils';
import { installMilestoneInvitationClickProbe, observeMilestoneInvitationAuditClick } from '../../../../e2e/support/milestone_invitation_audit';

vi.mock('../../../../e2e/db_utils', () => ({ e2eDbQuery: vi.fn() }));
afterEach(() => { document.body.innerHTML = ''; vi.restoreAllMocks(); });

it('the read-only activation probe ignores synthetic clicks and other controls', () => {
  document.body.innerHTML = '<button>Invitation<span>icon</span></button><button>Other</button>';
  const button = document.querySelector('button')!;
  const added = vi.spyOn(window, 'addEventListener');
  const removed = vi.spyOn(window, 'removeEventListener');
  const probe = installMilestoneInvitationClickProbe(button);
  const capture = added.mock.calls.find(([type]) => type === 'click')![1] as (event: MouseEvent) => void;
  try {
    button.click();
    capture({ isTrusted: true, composedPath: () => [document.querySelectorAll('button')[1]] } as unknown as MouseEvent);
    expect(probe.clicks).toBe(0);
    expect(probe.startedAt).toBe(0);
    capture({ isTrusted: true, composedPath: () => [button.firstElementChild, button] } as unknown as MouseEvent);
    expect(probe.clicks).toBe(1);
    expect(probe.startedAt).toBeGreaterThan(0);
  } finally { probe.dispose(); }
  expect(removed).toHaveBeenCalledWith('click', capture, true);
});

function boundary(fault?: string) {
  const owner = { userId: 'owned-user', tenantId: 'owned-tenant' };
  const origin = 'http://127.0.0.1:18789';
  const link = 'https://omnisolo.co/invite/recorded-invitation';
  const text = `We've recorded 1 orders in OmniSolo. ${link}`;
  const row = { id: 'recorded-invitation', tenant_id: owner.tenantId, team_id: owner.tenantId,
    inviter_id: owner.userId, invitee_id: 'pending-invite', status: 'PENDING' };
  const query = vi.mocked(e2eDbQuery).mockReset();
  query.mockResolvedValueOnce(fault === 'preexisting-invite' ? [row] : []);
  query.mockResolvedValueOnce([{ recorded_orders: fault === 'no-orders' ? 0 : 1 }]);
  query.mockResolvedValueOnce(fault === 'missing-sql' ? [] : fault === 'duplicate-sql' ? [row, row]
    : [{ ...row, ...(fault === 'wrong-sql' ? { tenant_id: 'foreign' } : {}) }]);
  const request = {
    url: () => `${origin}${fault === 'unrelated-request' ? '/scroll-read' : '/api/v1/growth/cloud-bridge/invite'}`,
    method: () => 'POST',
    headers: () => ({ 'x-ohc-expected-user': fault === 'wrong-user' ? 'foreign' : owner.userId,
      'x-ohc-expected-tenant': fault === 'wrong-tenant' ? 'foreign' : owner.tenantId }),
    postDataJSON: () => ({ invitee_id: fault === 'wrong-body' ? 'other' : 'pending-invite' }),
    timing: () => ({ startTime: fault === 'post-before-click' ? 100 : 300 }),
  };
  const response = { request: () => request, url: request.url,
    status: () => fault === 'failed-response' ? 503 : 200,
    json: async () => ({ invite_link: fault === 'invalid-link' ? 'https://omnisolo.co/invite/default' : link,
      ...(fault === 'error-body' ? { error: 'unconfirmed' } : {}), ...(fault === 'false-success' ? { success: false } : {}) }),
  };
  type ReceiptResponse = typeof response;
  let matches!: (value: ReceiptResponse) => boolean;
  let resolve!: (value: ReceiptResponse) => void;
  let reject!: (error: Error) => void;
  const preview = { waitFor: async () => { if (fault === 'missing-preview') throw new Error('No mounted preview'); },
    inputValue: async () => fault === 'wrong-preview' ? 'Scroll-only update' : text };
  const region = {
    getByLabel: () => preview,
    getByText: (text: string) => ({ waitFor: async () => {
      if (fault === 'missing-order-count' && text === '1 recorded orders') throw new Error('Owned order count not shown');
      if (fault === 'missing-threshold' && text === 'Recorded-order milestone: 1') throw new Error('Owned threshold not shown');
    } }),
    getByRole: (_role: string, { name }: { name: string }) => ({ waitFor: async () => {
      if (fault === `hidden-${name}`) throw new Error('Share control is not visible');
    }, getAttribute: async () => {
      const url = new URL(name === 'Share on X' ? 'https://twitter.com/intent/tweet' : 'https://wa.me/');
      url.searchParams.set('text', fault === `wrong-${name}` ? 'unrelated' : text);
      return url.href;
    } }),
  };
  const page = Object.assign(new EventEmitter(), { url: () => origin + '/dashboard', getByRole: () => region,
    waitForResponse: (predicate: typeof matches) => { matches = predicate; return new Promise<typeof response>((yes, no) => { resolve = yes; reject = no; }); },
  });
  const click = vi.fn(async () => {
    if (fault === 'scroll-only') { reject(new Error('No invitation POST after scroll-only changes')); return; }
    page.emit('request', request);
    if (fault === 'duplicate-post') page.emit('request', request);
    if (matches(response)) resolve(response); else reject(new Error('No matching invitation POST'));
  });
  document.body.innerHTML = fault === 'other-control' ? '<button>Create milestone invitation</button>'
    : '<section aria-label="Recorded order milestone"><button>Create milestone invitation</button></section>';
  const element = document.querySelector('button')!;
  const probe = { clicks: fault === 'no-trusted-click' ? 0 : fault === 'multiple-trusted-clicks' ? 2 : 1, startedAt: 200, dispose: vi.fn() };
  const target = { evaluate: async (read: (element: HTMLButtonElement) => unknown) => read(element), click,
    evaluateHandle: async () => ({ evaluate: async (read: (value: typeof probe) => unknown) => read(probe), dispose: vi.fn() }),
  };
  const fallback = vi.fn(async () => ({ changed: true, requestSeen: true, downloadSeen: false, fileChooserSeen: false,
    popupSeen: false, validationSeen: false, dialogSeen: false, decisionSeen: false }));
  const run = () => observeMilestoneInvitationAuditClick(page as unknown as Page, target as unknown as ElementHandle<HTMLElement>, owner, fallback);
  return { owner, page, target, fallback, query, row, run };
}

it('credits one native-scroll invitation click only after owned POST, SQL and mounted share proof', async () => {
  const b = boundary();
  expect(await b.run()).toMatchObject({ changed: true, requestSeen: true });
  expect(b.target.click).toHaveBeenCalledExactlyOnceWith({ timeout: 5000 });
  expect(b.fallback).not.toHaveBeenCalled();
  expect(b.query).toHaveBeenCalledTimes(3);
  expect(b.query.mock.calls[0][0]).toMatch(/FROM team_invites WHERE tenant_id=\$1/);
  expect(b.query.mock.calls[1][0]).toMatch(/FROM orders WHERE tenant_id=\$1/);
  expect(b.query.mock.calls[2]).toEqual([expect.stringMatching(/FROM team_invites WHERE tenant_id=\$1/), [b.owner.tenantId]]);
  expect(b.page.listenerCount('request')).toBe(0);
});

for (const fault of ['scroll-only', 'unrelated-request', 'wrong-user', 'wrong-tenant', 'wrong-body', 'failed-response',
  'invalid-link', 'error-body', 'false-success', 'missing-sql', 'duplicate-sql', 'wrong-sql', 'duplicate-post',
  'missing-preview', 'wrong-preview', 'missing-order-count', 'missing-threshold', 'post-before-click', 'no-trusted-click', 'multiple-trusted-clicks',
  'hidden-Share on X', 'hidden-Share to WhatsApp', 'wrong-Share on X', 'wrong-Share to WhatsApp']) it(`never credits or replays an unproved invitation: ${fault}`, async () => {
  const b = boundary(fault);
  await expect(b.run()).rejects.toThrow();
  expect(b.target.click).toHaveBeenCalledTimes(1);
  expect(b.fallback).not.toHaveBeenCalled();
  expect(b.page.listenerCount('request')).toBe(0);
});

for (const fault of ['preexisting-invite', 'no-orders']) it(`refuses an invalid owned baseline before any click: ${fault}`, async () => {
  const b = boundary(fault);
  await expect(b.run()).rejects.toThrow();
  expect(b.target.click).not.toHaveBeenCalled();
  expect(b.fallback).not.toHaveBeenCalled();
});

it('delegates unrelated controls unchanged without invitation reads or native-scroll override', async () => {
  const b = boundary('other-control');
  await b.run();
  expect(b.fallback).toHaveBeenCalledTimes(1);
  expect(b.query).not.toHaveBeenCalled();
  expect(b.target.click).not.toHaveBeenCalled();
});
