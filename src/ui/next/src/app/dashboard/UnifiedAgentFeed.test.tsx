import "@testing-library/jest-dom";
import { act, fireEvent, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { UnifiedAgentFeed } from "./UnifiedAgentFeed";
import { invalidateQueueOwner, readQueueOwner } from '@/lib/sync/queueIdentity';

beforeEach(async () => {
  invalidateQueueOwner();
  vi.stubGlobal('fetch', vi.fn(async () => Response.json({ userId: 'user-1', tenantId: 'tenant-1', expiresAt: Date.now() + 60_000 })));
  await readQueueOwner();
});

vi.mock("../utils/offlineQueue", () => ({
  getActions: vi.fn().mockResolvedValue([]),
  enqueueAction: vi.fn(),
  removeAction: vi.fn(),
}));

describe("UnifiedAgentFeed activity surfaces", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
    vi.useRealTimers();
  });

  it("exposes the real selected choice and restores Proposals after Activity", async () => {
    vi.stubGlobal("fetch", vi.fn().mockResolvedValue(Response.json({ items: [] })));
    render(<UnifiedAgentFeed initialData={{ items: [], activity: [] }} />);
    const proposals = screen.getByRole("button", { name: "Proposals" });
    const activity = screen.getByRole("button", { name: "Activity Feed" });
    expect(proposals).toHaveAttribute("aria-pressed", "true");
    expect(activity).toHaveAttribute("aria-pressed", "false");
    await userEvent.setup().click(activity);
    expect(activity).toHaveAttribute("aria-pressed", "true");
    expect(proposals).toHaveAttribute("aria-pressed", "false");
    await userEvent.setup().click(proposals);
    expect(proposals).toHaveAttribute("aria-pressed", "true");
    expect(activity).toHaveAttribute("aria-pressed", "false");
    await userEvent.setup().click(proposals);
    expect(proposals).toHaveAttribute("aria-pressed", "true");
  });

  it("renders activity entries as glass surfaces", async () => {
    render(
      <UnifiedAgentFeed
        initialData={{
          items: [
            {
              id: "activity-1",
              tenant_id: "tenant-1",
              event_source: "operations",
              context_payload: { description: "Completed prep" },
              proposed_action: { message: "Completed prep" },
              lifecycle_state: "APPROVED",
              created_at: "2026-08-10T00:00:00Z",
              updated_at: "2026-08-10T00:01:00Z",
            },
          ],
        }}
      />,
    );

    await userEvent.setup().click(screen.getByRole("button", { name: "Activity Feed" }));

    await waitFor(() => expect(screen.getByText("Completed prep")).toBeVisible());

    expect(screen.getByTestId("activity-feed-entry")).toHaveClass("glassmorphism");
  });

  it("refreshes through authenticated HTTP polling without opening a browser socket", async () => {
    vi.useFakeTimers();
    const fetchMock = vi.fn(async () => Response.json({ items: [] }));
    const webSocketMock = vi.fn();
    vi.stubGlobal("fetch", fetchMock);
    vi.stubGlobal("WebSocket", webSocketMock);

    render(
      <UnifiedAgentFeed
        initialData={{
          items: [],
          activity: [],
        }}
      />,
    );

    await act(async () => {
      await vi.advanceTimersByTimeAsync(5_000);
    });

    expect(fetchMock).toHaveBeenCalledWith("/api/v1/agent-feed");
    expect(webSocketMock).not.toHaveBeenCalled();
  });
});

afterEach(() => vi.unstubAllGlobals());

const pendingItem = {
  id: 'decision-1', tenant_id: 'tenant-1', event_source: 'operations',
  context_payload: { description: 'Review owner proposal' }, proposed_action: { message: 'Prepare the requested work' },
  lifecycle_state: 'PENDING_APPROVAL', created_at: '2026-09-30T00:00:00Z', updated_at: '2026-09-30T00:00:00Z',
};

const canonicalReceipt = (overrides = {}) => ({ ...pendingItem, lifecycle_state: 'APPROVED', decision_recorded: true, ...overrides });
const legacyReceipt = (edited: string | null = null) => ({
  success: true, decision_recorded: true, item: { id: pendingItem.id, tenant_id: pendingItem.tenant_id, lifecycle_state: 'APPROVED', edited_payload: edited },
});

const customerDraft = {
  ...pendingItem,
  id: 'customer-draft-1', event_source: 'CustomerSuccessAgent',
  context_payload: { description: 'Recorded customer inquiry' },
  proposed_action: { draft: 'Owner-reviewed response' },
};
// load_ui_triage_from_db includes an alternate projection of this same
// agent_feed_items record. It is not a second pending business action.
const customerTriageProjection = {
  ...customerDraft, source: 'CustomerSuccessAgent', action_type: 'approval',
};

it('revalidates canonical actions when a late dashboard aggregate arrives', async () => {
  const fetcher = vi.fn(async (_url?: RequestInfo | URL, options?: RequestInit) => Response.json(options?.method === 'PUT' ? canonicalReceipt() : { items: [pendingItem] }));
  vi.stubGlobal('fetch', fetcher);
  const { rerender } = render(<UnifiedAgentFeed initialData={{ items: [] }} />);
  const approve = await screen.findByRole('button', { name: 'Approve proposal' });
  expect(approve).toBeVisible();
  let finishRefresh!: (response: Response) => void;
  fetcher.mockImplementationOnce(() => new Promise<Response>((resolve) => { finishRefresh = resolve; }));

  // The dashboard aggregate can finish after this component's own read. Its
  // cached canonical list omits the new row, but triage still projects its ID
  // without the canonical proposal's context or action payload.
  rerender(<UnifiedAgentFeed initialData={{
    items: [{ ...customerDraft, lifecycle_state: 'APPROVED' }],
    triage: [{ id: pendingItem.id, tenant_id: pendingItem.tenant_id, source: 'operations', action_type: 'approval' }],
  }} />);
  expect(screen.getByRole('button', { name: 'Approve proposal' })).toBeVisible();
  await act(async () => finishRefresh(Response.json({ items: [pendingItem] })));
  await waitFor(() => expect(screen.getByRole('button', { name: 'Approve proposal' })).toBeVisible());
  expect(screen.getByTestId(`triage-card-${pendingItem.id}`)).toHaveTextContent('Review owner proposal');
  await userEvent.setup().click(screen.getByRole('button', { name: 'Approve proposal' }));
  expect(fetcher).toHaveBeenCalledWith(`/api/v1/agent-feed/${pendingItem.id}`, expect.objectContaining({
    method: 'PUT', body: JSON.stringify({ state: 'APPROVED' }),
  }));
  await waitFor(() => expect(screen.queryByTestId(`triage-card-${pendingItem.id}`)).not.toBeInTheDocument());
});

it('honors a newly committed decision when revalidating a late pending aggregate', async () => {
  const fetcher = vi.fn(async () => Response.json({ items: [pendingItem] }));
  vi.stubGlobal('fetch', fetcher);
  const { rerender } = render(<UnifiedAgentFeed initialData={{ items: [] }} />);
  await screen.findByRole('button', { name: 'Approve proposal' });
  fetcher.mockImplementation(async () => Response.json({ items: [{ ...pendingItem, lifecycle_state: 'APPROVED' }] }));
  rerender(<UnifiedAgentFeed initialData={{ items: [pendingItem] }} />);
  await waitFor(() => expect(screen.queryByTestId(`triage-card-${pendingItem.id}`)).not.toBeInTheDocument());
  await userEvent.setup().click(screen.getByRole('button', { name: 'Activity Feed' }));
  expect(screen.getByTestId('activity-feed-entry')).toHaveTextContent('Prepare the requested work');
});

it('renders one authoritative customer action when the same record is projected in triage', async () => {
  vi.stubGlobal('fetch', vi.fn(async () => Response.json({ items: [customerDraft] })));
  render(<UnifiedAgentFeed initialData={{ items: [customerDraft], triage: [customerTriageProjection, { id: 'distinct-triage', tenant_id: 'tenant-1', context: 'Separate recorded inquiry', action_payload: 'Separate draft' }] }} />);
  await screen.findByText('Recorded customer inquiry');
  expect(screen.getAllByTestId('triage-card-customer-draft-1')).toHaveLength(1);
  expect(screen.getByTestId('triage-draft-customer-draft-1')).toHaveTextContent('Owner-reviewed response');
  expect(screen.getByTestId('triage-card-distinct-triage')).toBeVisible();
});

it('does not reopen an approved canonical record from a stale triage projection', async () => {
  vi.stubGlobal('fetch', vi.fn(async () => Response.json({ items: [] })));
  render(<UnifiedAgentFeed initialData={{ items: [{ ...customerDraft, lifecycle_state: 'APPROVED' }], triage: [customerTriageProjection] }} />);
  await screen.findByRole('heading', { name: 'No pending proposals are recorded.' });
  expect(screen.queryByTestId('triage-card-customer-draft-1')).not.toBeInTheDocument();
  await userEvent.setup().click(screen.getByRole('button', { name: 'Activity Feed' }));
  expect(screen.getByTestId('activity-feed-entry')).toBeVisible();
});

it('honors an authoritative empty triage refresh after canonical approval', async () => {
  vi.useFakeTimers();
  try {
    const fetcher = vi.fn(async () => Response.json({ items: [], triage: [], priority_tasks: [] }));
    vi.stubGlobal('fetch', fetcher);
    render(<UnifiedAgentFeed initialData={{ items: [{ ...customerDraft, lifecycle_state: 'APPROVED' }], triage: [customerTriageProjection] }} />);
    expect(screen.queryByTestId('triage-card-customer-draft-1')).not.toBeInTheDocument();
    await act(async () => { await vi.advanceTimersByTimeAsync(5_000); });
    expect(fetcher).toHaveBeenCalledWith('/api/v1/agent-feed');
    expect(screen.queryByTestId('triage-card-customer-draft-1')).not.toBeInTheDocument();
    expect(screen.getByRole('heading', { name: 'No pending proposals are recorded.' })).toBeVisible();
  } finally { vi.useRealTimers(); }
});

it('preserves a distinct triage projection when a refresh omits that collection', async () => {
  vi.useFakeTimers();
  try {
    vi.stubGlobal('fetch', vi.fn(async () => Response.json({ items: [] })));
    render(<UnifiedAgentFeed initialData={{ items: [{ ...customerDraft, lifecycle_state: 'APPROVED' }], triage: [{ id: 'distinct-triage', tenant_id: 'tenant-1', context: 'Separate recorded inquiry', action_payload: 'Separate draft' }] }} />);
    expect(screen.getByTestId('triage-card-distinct-triage')).toBeVisible();
    await act(async () => { await vi.advanceTimersByTimeAsync(5_000); });
    expect(screen.getByTestId('triage-card-distinct-triage')).toBeVisible();
  } finally { vi.useRealTimers(); }
});

it.each(['triage', 'task', 'order'])('forwards the owner-edited %s draft to the durable action endpoint', async (event_source) => {
  vi.stubGlobal('fetch', vi.fn(async () => Response.json(legacyReceipt('Owner-reviewed draft'))));
  render(<UnifiedAgentFeed initialData={{ items: [{ ...pendingItem, event_source }] }} />);
  const user = userEvent.setup();
  await user.click(await screen.findByTestId('edit-proposal'));
  await user.clear(screen.getByTestId('edit-proposal-textarea'));
  await user.type(screen.getByTestId('edit-proposal-textarea'), 'Owner-reviewed draft');
  await user.click(screen.getByTestId('save-proposal'));
  const [url, options] = vi.mocked(fetch).mock.calls[0];
  expect(url).toBe('/api/v1/triage/action');
  expect(JSON.parse(String(options?.body))).toEqual({ triage_item_id: 'decision-1', approved: true, edited_payload: 'Owner-reviewed draft' });
  expect(screen.getByRole('status', { name: 'Decision status' })).toHaveTextContent('Approval recorded. Execution or delivery is not verified by this decision.');
  await waitFor(() => expect(screen.queryByTestId('triage-card-decision-1')).not.toBeInTheDocument());
});

it('keeps the approval transition visible until the successful decision has been acknowledged', async () => {
  let acknowledge!: (response: Response) => void;
  vi.stubGlobal('fetch', vi.fn(() => new Promise<Response>((resolve) => { acknowledge = resolve; })));
  render(<UnifiedAgentFeed initialData={{ items: [pendingItem] }} />);
  const card = await screen.findByTestId('triage-card-decision-1');
  await userEvent.setup().click(screen.getByRole('button', { name: 'Approve proposal' }));
  expect(card).toBeVisible();
  expect(screen.getByRole('status', { name: 'Decision status' })).toHaveTextContent('Waiting for the recorded decision');
  expect(card).not.toHaveClass('border-green-500', 'scale-95');
  await act(async () => acknowledge(Response.json(canonicalReceipt())));
  expect(card).toBeVisible();
  expect(screen.getByRole('status', { name: 'Decision status' })).toHaveTextContent('Approval recorded.');
  expect(card).toHaveClass('border-green-500', 'scale-95');
  await waitFor(() => expect(screen.queryByTestId('triage-card-decision-1')).toBeNull());
});

it('retains the proposal and reports a failed decision instead of silently hiding it', async () => {
  vi.stubGlobal('fetch', vi.fn(async () => new Response('{}', { status: 500 })));
  const errorLog = vi.spyOn(console, 'error').mockImplementation(() => {});
  try {
    render(<UnifiedAgentFeed initialData={{ items: [pendingItem] }} />);
    await screen.findByTestId('triage-card-decision-1');
    await userEvent.setup().click(screen.getByRole('button', { name: 'Approve proposal' }));
    expect(await screen.findByText('Failed to submit decision')).toBeVisible();
    expect(screen.getByTestId('triage-card-decision-1')).toBeVisible();
    expect(screen.getByTestId('triage-card-decision-1')).not.toHaveClass('border-green-500');
  } finally { errorLog.mockRestore(); }
});

const durableSync = vi.hoisted(() => vi.fn(async () => {}));
vi.mock('../../lib/sync/SyncManager', () => ({ SyncManager: { getInstance: () => ({ sync: durableSync }) } }));
it('delegates reconnection to the durable queue instead of sending queued approvals a second way', async () => {
  const queue = await import('../utils/offlineQueue');
  vi.mocked(queue.getActions).mockResolvedValue([{ id: 'queued-decision', type: 'approve_agent_feed', timestamp: 1, payload: { id: 'decision-1', approved: true, event_source: 'operations' } }]);
  const fetchMock = vi.fn(async () => Response.json({ success: true }));
  vi.stubGlobal('fetch', fetchMock);
  render(<UnifiedAgentFeed initialData={{ items: [pendingItem] }} />);
  await screen.findByTestId('triage-card-decision-1');
  await act(async () => { window.dispatchEvent(new Event('online')); });
  expect(durableSync).toHaveBeenCalledOnce();
  expect(fetchMock).not.toHaveBeenCalled();
  expect(queue.removeAction).not.toHaveBeenCalled();
  vi.mocked(queue.getActions).mockResolvedValue([]);
});

it('describes a verified empty feed without claiming that the business is healthy', async () => {
  vi.stubGlobal('fetch', vi.fn(async () => Response.json({ items: [] })));
  render(<UnifiedAgentFeed initialData={{ items: [], activity: [] }} />);
  expect(await screen.findByRole('heading', { name: 'No pending proposals are recorded.' })).toBeVisible();
  expect(screen.queryByText(/your business is running smoothly/i)).not.toBeInTheDocument();
});

it('does not claim an empty feed when its initial read failed', async () => {
  vi.stubGlobal('fetch', vi.fn(async () => new Response('', { status: 503 })));
  const errorLog = vi.spyOn(console, 'error').mockImplementation(() => {});
  try {
    render(<UnifiedAgentFeed initialData={{ items: [], activity: [] }} />);
    expect(await screen.findByText('Feed temporarily unavailable')).toBeVisible();
    expect(screen.queryByTestId('triage-feed-empty')).not.toBeInTheDocument();
    expect(screen.queryByText(/your business is running smoothly|no pending proposals are recorded/i)).not.toBeInTheDocument();
  } finally { errorLog.mockRestore(); }
});

it('does not claim all proposals are handled while a recorded proposal still awaits review', async () => {
  render(<UnifiedAgentFeed initialData={{ items: [pendingItem] }} />);
  await screen.findByTestId('triage-card-decision-1');
  expect(screen.queryByText('All caught up on automated triage proposals!')).not.toBeInTheDocument();
  expect(screen.getByText('1 recorded proposal in this feed.')).toBeVisible();
});

it.each([
  ['canonical empty success', 'operations', {}],
  ['canonical wrong id', 'operations', canonicalReceipt({ id: 'another-item' })],
  ['canonical wrong tenant', 'operations', canonicalReceipt({ tenant_id: 'another-tenant' })],
  ['canonical wrong lifecycle', 'operations', canonicalReceipt({ lifecycle_state: 'DISMISSED' })],
  ['canonical semantic failure', 'operations', canonicalReceipt({ success: false })],
  ['canonical unrecorded decision', 'operations', canonicalReceipt({ decision_recorded: false })],
  ['triage bare success', 'triage', { success: true }],
  ['task wrong tenant', 'task', { ...legacyReceipt(), item: { ...legacyReceipt().item, tenant_id: 'another-tenant' } }],
  ['order wrong lifecycle', 'order', { ...legacyReceipt(), item: { ...legacyReceipt().item, lifecycle_state: 'DISMISSED' } }],
])('retains the proposal after %s', async (_name, event_source, receipt) => {
  vi.stubGlobal('fetch', vi.fn(async () => Response.json(receipt)));
  render(<UnifiedAgentFeed initialData={{ items: [{ ...pendingItem, event_source }] }} />);
  await userEvent.setup().click(await screen.findByRole('button', { name: 'Approve proposal' }));
  expect(await screen.findByRole('alert')).toHaveTextContent('Outcome unconfirmed');
  expect(screen.getByTestId('triage-card-decision-1')).toBeVisible();
  expect(screen.getByRole('button', { name: 'Approve proposal' })).toBeEnabled();
  expect(screen.queryByText('Approval recorded.')).not.toBeInTheDocument();
});

it.each(['operations', 'triage', 'task', 'order'])('rejects a %s acknowledgment that did not persist the edited content', async (event_source) => {
  const receipt = event_source === 'operations'
    ? canonicalReceipt({ proposed_action: { message: 'Old draft', draft_reply: 'Owner-reviewed draft' } })
    : legacyReceipt('Old draft');
  vi.stubGlobal('fetch', vi.fn(async () => Response.json(receipt)));
  render(<UnifiedAgentFeed initialData={{ items: [{ ...pendingItem, event_source }] }} />);
  const user = userEvent.setup();
  await user.click(await screen.findByTestId('edit-proposal'));
  await user.clear(screen.getByTestId('edit-proposal-textarea'));
  await user.type(screen.getByTestId('edit-proposal-textarea'), 'Owner-reviewed draft');
  await user.click(screen.getByTestId('save-proposal'));
  expect(await screen.findByRole('alert')).toHaveTextContent('Outcome unconfirmed');
  expect(screen.getByTestId('triage-card-decision-1')).toBeVisible();
  expect(screen.getByTestId('edit-proposal-textarea')).toHaveValue('Owner-reviewed draft');
  expect(screen.getByTestId('save-proposal')).toBeEnabled();
});

it('records a canonical dismissal without a delivery claim', async () => {
  vi.stubGlobal('fetch', vi.fn(async () => Response.json(canonicalReceipt({ lifecycle_state: 'DISMISSED' }))));
  render(<UnifiedAgentFeed initialData={{ items: [pendingItem] }} />);
  await userEvent.setup().click(await screen.findByRole('button', { name: 'Reject proposal' }));
  expect(screen.getByRole('status', { name: 'Decision status' })).toHaveTextContent('Dismissal recorded.');
  await waitFor(() => expect(screen.queryByTestId('triage-card-decision-1')).not.toBeInTheDocument());
});


it('keeps an unconfirmed edited proposal through a background refresh', async () => {
  const fetcher = vi.fn(async (_url?: RequestInfo | URL, options?: RequestInit) => Response.json(options?.method === 'PUT'
    ? canonicalReceipt({ proposed_action: { message: 'Old draft' } })
    : { items: [canonicalReceipt({ proposed_action: { message: 'Old draft' } })] }));
  vi.stubGlobal('fetch', fetcher);
  const { rerender } = render(<UnifiedAgentFeed initialData={{ items: [pendingItem] }} />);
  const user = userEvent.setup();
  await user.click(await screen.findByTestId('edit-proposal'));
  await user.clear(screen.getByTestId('edit-proposal-textarea'));
  await user.type(screen.getByTestId('edit-proposal-textarea'), 'Owner-reviewed draft');
  await user.click(screen.getByTestId('save-proposal'));
  expect(await screen.findByRole('alert')).toHaveTextContent('Outcome unconfirmed');
  rerender(<UnifiedAgentFeed initialData={{ items: [] }} />);
  await waitFor(() => expect(fetcher).toHaveBeenCalledWith('/api/v1/agent-feed'));
  expect(screen.getByTestId('triage-card-decision-1')).toBeVisible();
  expect(screen.getByTestId('edit-proposal-textarea')).toHaveValue('Owner-reviewed draft');
});

it('accepts a canonical receipt only when all returned edit fields match', async () => {
  vi.stubGlobal('fetch', vi.fn(async () => Response.json(canonicalReceipt({ proposed_action: { message: 'Owner-reviewed draft', draft_reply: 'Owner-reviewed draft' } }))));
  render(<UnifiedAgentFeed initialData={{ items: [pendingItem] }} />);
  const user = userEvent.setup();
  await user.click(await screen.findByTestId('edit-proposal'));
  await user.clear(screen.getByTestId('edit-proposal-textarea'));
  await user.type(screen.getByTestId('edit-proposal-textarea'), 'Owner-reviewed draft');
  await user.click(screen.getByTestId('save-proposal'));
  expect(fetch).toHaveBeenCalledWith('/api/v1/agent-feed/decision-1', expect.objectContaining({ method: 'PUT', body: JSON.stringify({ state: 'APPROVED', modified_content: 'Owner-reviewed draft' }) }));
  expect(screen.getByRole('status', { name: 'Decision status' })).toHaveTextContent('Approval recorded.');
  expect(screen.getByTestId('triage-card-decision-1')).toHaveClass('border-green-500');
  await waitFor(() => expect(screen.queryByTestId('triage-card-decision-1')).not.toBeInTheDocument());
});

it('queues offline decisions with pending wording and no recorded acknowledgment', async () => {
  const queue = await import('../utils/offlineQueue');
  const online = vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
  vi.stubGlobal('fetch', vi.fn());
  try {
    render(<UnifiedAgentFeed initialData={{ items: [pendingItem] }} />);
    await userEvent.setup().click(await screen.findByRole('button', { name: 'Approve proposal' }));
    expect(queue.enqueueAction).toHaveBeenCalledWith(expect.objectContaining({ type: 'approve_agent_feed', payload: expect.objectContaining({ id: pendingItem.id, approved: true }) }));
    expect(screen.getByRole('status', { name: 'Decision status' })).toHaveTextContent('Decision queued offline. Approval or dismissal is not yet recorded.');
    expect(screen.getByText('Pending Sync (1)')).toBeVisible();
    expect(vi.mocked(fetch).mock.calls.filter(([, options]) => ['POST', 'PUT'].includes(options?.method ?? 'GET'))).toHaveLength(0);
  } finally { online.mockRestore(); }
});

it('retires an unconfirmed card and editor when the account is invalidated', async () => {
  vi.stubGlobal('fetch', vi.fn(async () => Response.json({})));
  render(<UnifiedAgentFeed initialData={{ items: [pendingItem] }} />);
  const user = userEvent.setup();
  await user.click(await screen.findByTestId('edit-proposal'));
  await user.clear(screen.getByTestId('edit-proposal-textarea'));
  await user.type(screen.getByTestId('edit-proposal-textarea'), 'Private owner draft');
  await user.click(screen.getByTestId('save-proposal'));
  expect(await screen.findByRole('alert')).toHaveTextContent('Outcome unconfirmed');
  await act(async () => { window.dispatchEvent(new Event('omnisolo_auth_changed')); });
  expect(screen.queryByTestId('triage-card-decision-1')).not.toBeInTheDocument();
  expect(screen.queryByDisplayValue('Private owner draft')).not.toBeInTheDocument();
});

it('does not restore retired aggregate projections when a new account feed omits them', async () => {
  vi.useFakeTimers();
  try {
    const oldAggregate = {
      items: [pendingItem],
      triage: [{ id: 'old-triage', tenant_id: 'tenant-1', context: 'Prior owner message', action_payload: 'Prior owner reply' }],
      priority_tasks: [{ id: 'old-task', tenant_id: 'tenant-1', description: 'Prior owner task', status: 'PENDING' }],
    };
    const newItem = { ...pendingItem, id: 'new-owner-item', tenant_id: 'tenant-2', context_payload: { description: 'Current owner proposal' } };
    const fetcher = vi.fn(async (url: RequestInfo | URL) => Response.json(String(url).includes('session-identity')
      ? { userId: 'user-2', tenantId: 'tenant-2', expiresAt: Date.now() + 60_000 }
      : { items: [newItem] }));
    vi.stubGlobal('fetch', fetcher);
    const { rerender } = render(<UnifiedAgentFeed initialData={oldAggregate} />);
    expect(screen.getByTestId('triage-card-old-triage')).toBeVisible();
    expect(screen.getByTestId('triage-card-old-task')).toBeVisible();
    await act(async () => {
      window.dispatchEvent(new Event('omnisolo_auth_changed'));
      await readQueueOwner();
      await vi.advanceTimersByTimeAsync(5_000);
    });
    expect(fetcher).toHaveBeenCalledWith('/api/v1/agent-feed');
    expect(screen.getByTestId('triage-card-new-owner-item')).toBeVisible();
    expect(screen.queryByTestId('triage-card-old-triage')).not.toBeInTheDocument();
    expect(screen.queryByTestId('triage-card-old-task')).not.toBeInTheDocument();
    expect(screen.queryByTestId('triage-card-decision-1')).not.toBeInTheDocument();
    await act(async () => { rerender(<UnifiedAgentFeed initialData={{ ...oldAggregate }} />); });
    expect(screen.getByTestId('triage-card-new-owner-item')).toBeVisible();
    expect(screen.queryByText('Prior owner message')).not.toBeInTheDocument();
    expect(screen.queryByText('Prior owner task')).not.toBeInTheDocument();
  } finally { vi.useRealTimers(); }
});

it.each(['draft_message', 'draft_action'])('accepts an edited canonical receipt using %s', async (field) => {
  vi.stubGlobal('fetch', vi.fn(async () => Response.json(canonicalReceipt({ proposed_action: { [field]: 'Owner-reviewed draft' } }))));
  render(<UnifiedAgentFeed initialData={{ items: [pendingItem] }} />);
  const user = userEvent.setup();
  await user.click(await screen.findByTestId('edit-proposal'));
  await user.clear(screen.getByTestId('edit-proposal-textarea'));
  await user.type(screen.getByTestId('edit-proposal-textarea'), 'Owner-reviewed draft');
  await user.click(screen.getByTestId('save-proposal'));
  expect(screen.getByRole('status', { name: 'Decision status' })).toHaveTextContent('Approval recorded.');
  await waitFor(() => expect(screen.queryByTestId('triage-card-decision-1')).not.toBeInTheDocument());
});

it.each(['draft_message', 'draft_action'])('rejects a stale canonical %s even when another edit field matches', async (field) => {
  vi.stubGlobal('fetch', vi.fn(async () => Response.json(canonicalReceipt({ proposed_action: { message: 'Owner-reviewed draft', [field]: 'Old draft' } }))));
  render(<UnifiedAgentFeed initialData={{ items: [pendingItem] }} />);
  const user = userEvent.setup();
  await user.click(await screen.findByTestId('edit-proposal'));
  await user.clear(screen.getByTestId('edit-proposal-textarea'));
  await user.type(screen.getByTestId('edit-proposal-textarea'), 'Owner-reviewed draft');
  await user.click(screen.getByTestId('save-proposal'));
  expect(await screen.findByRole('alert')).toHaveTextContent('Outcome unconfirmed');
  expect(screen.getByTestId('edit-proposal-textarea')).toHaveValue('Owner-reviewed draft');
});

it.each([
  ['dismissal', 'Reject proposal', 'DISMISSED', 'Dismissal recorded.'],
  ['approval', 'Approve proposal', 'APPROVED', 'Approval recorded.'],
])('removes a confirmed %s while another widget revalidates queue identity', async (_name, button, lifecycle_state, status) => {
  const fetcher = vi.fn(async (url: RequestInfo | URL) => Response.json(String(url).endsWith('/session-identity')
    ? { userId: 'user-1', tenantId: 'tenant-1', expiresAt: Date.now() + 60_000 }
    : canonicalReceipt({ lifecycle_state })));
  vi.stubGlobal('fetch', fetcher);
  render(<UnifiedAgentFeed initialData={{ items: [pendingItem] }} />);
  await screen.findByTestId('triage-card-decision-1');
  vi.useFakeTimers();
  try {
    await act(async () => { fireEvent.click(screen.getByRole('button', { name: button })); });
    expect(screen.getByRole('status', { name: 'Decision status' })).toHaveTextContent(status);

    // Queue summaries and sync both reverify identity independently. During
    // that read, the cached owner is deliberately unavailable, even though
    // this decision already has its matching tenant/id/state receipt.
    let finishIdentity!: (response: Response) => void;
    fetcher.mockImplementationOnce(() => new Promise<Response>(resolve => { finishIdentity = resolve; }));
    let revalidation!: Promise<unknown>;
    await act(async () => { revalidation = readQueueOwner(); });
    await act(async () => { await vi.advanceTimersByTimeAsync(500); });
    // Settle the unrelated request before asserting, so no verification is
    // left pending if this regression fails.
    await act(async () => {
      finishIdentity(Response.json({ userId: 'user-1', tenantId: 'tenant-1', expiresAt: Date.now() + 60_000 }));
      await revalidation;
    });
    expect(screen.queryByTestId('triage-card-decision-1')).not.toBeInTheDocument();
    expect(screen.getByRole('status', { name: 'Decision status' })).toHaveTextContent(status);
  } finally { vi.useRealTimers(); }
});

it('does not apply a confirmed decision exit after a different owner is verified', async () => {
  vi.stubGlobal('fetch', vi.fn(async (url: RequestInfo | URL) => Response.json(String(url).endsWith('/session-identity')
    ? { userId: 'other-user', tenantId: 'tenant-1', expiresAt: Date.now() + 60_000 }
    : canonicalReceipt({ lifecycle_state: 'DISMISSED' }))));
  render(<UnifiedAgentFeed initialData={{ items: [pendingItem] }} />);
  await screen.findByTestId('triage-card-decision-1');
  vi.useFakeTimers();
  try {
    await act(async () => { fireEvent.click(screen.getByRole('button', { name: 'Reject proposal' })); });
    expect(screen.getByRole('status', { name: 'Decision status' })).toHaveTextContent('Dismissal recorded.');
    await act(async () => { await readQueueOwner(); });
    await act(async () => { await vi.advanceTimersByTimeAsync(500); });
    expect(screen.getByTestId('triage-card-decision-1')).toBeVisible();
  } finally { vi.useRealTimers(); }
});

it('cancels a confirmed decision exit when the account is invalidated', async () => {
  const nextOwnerItem = { ...pendingItem, tenant_id: 'tenant-2', context_payload: { description: 'New owner proposal' } };
  vi.stubGlobal('fetch', vi.fn(async (_url: RequestInfo | URL, options?: RequestInit) => Response.json(options?.method === 'PUT'
    ? canonicalReceipt({ lifecycle_state: 'DISMISSED' })
    : { items: [nextOwnerItem] })));
  const { rerender } = render(<UnifiedAgentFeed initialData={{ items: [pendingItem] }} />);
  await screen.findByTestId('triage-card-decision-1');
  vi.useFakeTimers();
  try {
    await act(async () => { fireEvent.click(screen.getByRole('button', { name: 'Reject proposal' })); });
    expect(screen.getByRole('status', { name: 'Decision status' })).toHaveTextContent('Dismissal recorded.');
    await act(async () => { window.dispatchEvent(new Event('omnisolo_auth_changed')); });
    await act(async () => { rerender(<UnifiedAgentFeed initialData={{ items: [] }} />); });
    await act(async () => { await vi.advanceTimersByTimeAsync(500); });
    expect(screen.getByTestId('triage-card-decision-1')).toHaveTextContent('New owner proposal');
    expect(screen.queryByRole('status', { name: 'Decision status' })).not.toBeInTheDocument();
  } finally { vi.useRealTimers(); }
});
