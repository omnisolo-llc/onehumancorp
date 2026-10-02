import "@testing-library/jest-dom";
import { act, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, it, vi } from "vitest";
import { UnifiedAgentFeed } from "./UnifiedAgentFeed";

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
    const fetchMock = vi.fn().mockResolvedValue(
      new Response(JSON.stringify({ items: [] }), { status: 200 }),
    );
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

it.each(['triage', 'task', 'order'])('forwards the owner-edited %s draft to the durable action endpoint', async (event_source) => {
  vi.stubGlobal('fetch', vi.fn(async () => Response.json({ success: true })));
  render(<UnifiedAgentFeed initialData={{ items: [{ ...pendingItem, event_source }] }} />);
  const user = userEvent.setup();
  await user.click(await screen.findByTestId('edit-proposal'));
  await user.clear(screen.getByTestId('edit-proposal-textarea'));
  await user.type(screen.getByTestId('edit-proposal-textarea'), 'Owner-reviewed draft');
  await user.click(screen.getByTestId('save-proposal'));
  const [url, options] = vi.mocked(fetch).mock.calls[0];
  expect(url).toBe('/api/v1/triage/action');
  expect(JSON.parse(String(options?.body))).toEqual({ triage_item_id: 'decision-1', approved: true, edited_payload: 'Owner-reviewed draft' });
});

it('keeps the approval transition visible until the successful decision has been acknowledged', async () => {
  let acknowledge!: (response: Response) => void;
  vi.stubGlobal('fetch', vi.fn(() => new Promise<Response>((resolve) => { acknowledge = resolve; })));
  render(<UnifiedAgentFeed initialData={{ items: [pendingItem] }} />);
  const card = await screen.findByTestId('triage-card-decision-1');
  await userEvent.setup().click(screen.getByRole('button', { name: 'Approve proposal' }));
  expect(card).toBeVisible();
  expect(card).toHaveClass('border-green-500', 'scale-95');
  await act(async () => acknowledge(new Response('{}', { status: 200 })));
  expect(card).toBeVisible();
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
