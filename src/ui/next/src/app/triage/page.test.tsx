import { act, render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import TriagePage from './page';

vi.mock('../components/AppShell', () => ({
  AppShell: ({ children }: { children: React.ReactNode }) => <main>{children}</main>,
}));
const enqueue = vi.hoisted(() => vi.fn());
vi.mock('../../lib/sync/SyncManager', () => ({ SyncManager: { getInstance: () => ({ enqueue }) } }));
vi.mock('../utils/offlineQueue', () => ({ getActions: vi.fn(async () => []) }));

const item = {
  id: 'item-1', tenant_id: 'tenant-1', customer_id: 'Maya', source: 'Instagram', priority: 'high',
  context: 'Needs a custom cake by Friday.', action_type: 'Draft Reply',
  action_payload: 'Hi! Custom cakes start at $50.', created_at: '2026-10-01T00:00:00Z',
};
const receipt = (approved = true, edited: string | null = null) => ({
  status: 'success', success: true, decision_recorded: true,
  item: { id: item.id, tenant_id: item.tenant_id, lifecycle_state: approved ? 'APPROVED' : 'DISMISSED', edited_payload: edited },
  dispatch: { status: 'NOT_REQUESTED' },
});
const fetcher = vi.fn<typeof fetch>();
const mutations = () => fetcher.mock.calls.filter(([, options]) => options?.method === 'POST');

beforeEach(() => {
  fetcher.mockReset(); enqueue.mockReset(); enqueue.mockResolvedValue(undefined);
  vi.stubGlobal('fetch', fetcher);
  vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(true);
  fetcher.mockResolvedValueOnce(Response.json([item]));
});
afterEach(() => { vi.restoreAllMocks(); vi.unstubAllGlobals(); });

async function openItem() {
  render(<TriagePage />);
  const header = await screen.findByTestId('triage-card-header-item-1');
  await userEvent.setup().click(header);
  return userEvent.setup();
}

describe('Triage decisions', () => {
  it('exposes the context and lets the keyboard expand and collapse the draft', async () => {
    const user = userEvent.setup();
    render(<TriagePage />);
    const header = await screen.findByRole('button', { name: /Maya/ });
    expect(header).toHaveTextContent(item.context);
    expect(header).toHaveAttribute('aria-expanded', 'false');
    expect(screen.queryByText(item.action_payload)).not.toBeInTheDocument();
    header.focus();
    await user.keyboard('{Enter}');
    expect(header).toHaveAttribute('aria-expanded', 'true');
    expect(screen.getByText(item.action_payload)).toBeVisible();
    expect(document.getElementById(header.getAttribute('aria-controls')!)).toBeVisible();
    await user.keyboard(' ');
    expect(header).toHaveAttribute('aria-expanded', 'false');
    expect(screen.queryByText(item.action_payload)).not.toBeInTheDocument();
    expect(mutations()).toHaveLength(0);
  });

  it.each([true, false])('waits for a matching durable %s decision before recording and removing the card', async (approved) => {
    let acknowledge!: (response: Response) => void;
    fetcher.mockImplementationOnce(() => new Promise<Response>(resolve => { acknowledge = resolve; }));
    const user = await openItem();
    await user.click(screen.getByTestId(`triage-${approved ? 'approve' : 'dismiss'}-item-1`));
    expect(screen.getByTestId('triage-card-item-1')).toBeVisible();
    expect(screen.getByRole('status')).toHaveTextContent('Waiting for the recorded decision');
    expect(screen.getByTestId('triage-approve-item-1')).toBeDisabled();
    expect(mutations()).toHaveLength(1);
    expect(JSON.parse(String(mutations()[0][1]?.body))).toEqual({ triage_item_id: item.id, approved });
    await act(async () => acknowledge(Response.json(receipt(approved))));
    expect(screen.getByRole('status')).toHaveTextContent(approved
      ? 'Approval recorded. Execution or delivery is not verified by this decision.' : 'Dismissal recorded.');
    await waitFor(() => expect(screen.queryByTestId('triage-card-item-1')).not.toBeInTheDocument());
  });

  it('sends the edited draft and requires its persisted value in the receipt', async () => {
    fetcher.mockResolvedValueOnce(Response.json(receipt(true, 'Owner-reviewed draft')));
    const user = await openItem();
    await user.click(screen.getByRole('button', { name: 'Review Draft' }));
    const editor = screen.getByRole('textbox', { name: 'Edit draft' });
    expect(editor).toHaveValue(item.action_payload);
    await user.clear(editor); await user.type(editor, 'Owner-reviewed draft');
    await user.click(screen.getByTestId('triage-save-btn-item-1'));
    expect(JSON.parse(String(mutations()[0][1]?.body))).toEqual({ triage_item_id: item.id, approved: true, edited_payload: 'Owner-reviewed draft' });
    expect(await screen.findByRole('status')).toHaveTextContent('Approval recorded.');
    await waitFor(() => expect(screen.queryByTestId('triage-card-item-1')).not.toBeInTheDocument());
  });

  it.each([
    ['HTTP failure', () => Response.json({}, { status: 503 })],
    ['empty success', () => Response.json({})],
    ['semantic failure', () => Response.json({ ...receipt(), success: false })],
    ['unrecorded decision', () => Response.json({ ...receipt(), decision_recorded: false })],
    ['wrong id', () => Response.json({ ...receipt(), item: { ...receipt().item, id: 'another-item' } })],
    ['wrong tenant', () => Response.json({ ...receipt(), item: { ...receipt().item, tenant_id: 'another-tenant' } })],
    ['wrong lifecycle', () => Response.json(receipt(false))],
    ['missing edited content', () => Response.json(receipt())],
    ['stale edited content', () => Response.json(receipt(true, 'Old draft'))],
    ['unreadable receipt', () => new Response('invalid json')],
  ])('retains the edited draft and card after %s', async (_name, response) => {
    fetcher.mockResolvedValueOnce(response());
    vi.spyOn(console, 'error').mockImplementation(() => {});
    const user = await openItem();
    await user.click(screen.getByRole('button', { name: 'Review Draft' }));
    const editor = screen.getByTestId('triage-edit-textarea-item-1');
    await user.clear(editor); await user.type(editor, 'Owner-reviewed draft');
    await user.click(screen.getByTestId('triage-save-btn-item-1'));
    await waitFor(() => expect(screen.getByRole('status')).toHaveTextContent(/unconfirmed|not recorded/i));
    expect(screen.getByTestId('triage-card-item-1')).toBeVisible();
    expect(editor).toHaveValue('Owner-reviewed draft');
    expect(screen.getByTestId('triage-save-btn-item-1')).toBeEnabled();
    expect(mutations()).toHaveLength(1);
    expect(screen.getByRole('status')).not.toHaveTextContent('Approval recorded.');
  });

  it('cancels an edit without mutating the original draft', async () => {
    const user = await openItem();
    await user.click(screen.getByRole('button', { name: 'Review Draft' }));
    await user.type(screen.getByTestId('triage-edit-textarea-item-1'), ' Unsaved');
    await user.click(screen.getByRole('button', { name: 'Cancel' }));
    expect(screen.queryByRole('textbox')).not.toBeInTheDocument();
    expect(screen.getByText(item.action_payload)).toBeVisible();
    expect(mutations()).toHaveLength(0);
  });

  it('describes an offline decision as queued rather than recorded or delivered', async () => {
    vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
    const user = await openItem();
    await user.click(screen.getByTestId('triage-approve-item-1'));
    expect(enqueue).toHaveBeenCalledWith(expect.objectContaining({ type: 'triage_action', payload: { triage_item_id: item.id, approved: true } }));
    expect(screen.getByRole('status')).toHaveTextContent('Decision queued offline. Approval or dismissal is not yet recorded.');
    expect(screen.getByText('Pending Sync (1)')).toBeVisible();
    expect(mutations()).toHaveLength(0);
  });

  it('retains the editor when offline queue storage fails', async () => {
    vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
    enqueue.mockRejectedValueOnce(new Error('Storage unavailable'));
    vi.spyOn(console, 'error').mockImplementation(() => {});
    const user = await openItem();
    await user.click(screen.getByRole('button', { name: 'Review Draft' }));
    await user.click(screen.getByTestId('triage-save-btn-item-1'));
    expect(await screen.findByRole('status')).toHaveTextContent('Decision was not queued. Your card and draft are retained.');
    expect(screen.getByTestId('triage-edit-textarea-item-1')).toHaveValue(item.action_payload);
    expect(mutations()).toHaveLength(0);
  });
});

it('retires an unconfirmed triage editor when the account is invalidated', async () => {
  fetcher.mockResolvedValueOnce(Response.json({}));
  const user = await openItem();
  await user.click(screen.getByRole('button', { name: 'Review Draft' }));
  await user.clear(screen.getByTestId('triage-edit-textarea-item-1'));
  await user.type(screen.getByTestId('triage-edit-textarea-item-1'), 'Private prior owner draft');
  await user.click(screen.getByTestId('triage-save-btn-item-1'));
  expect(screen.getByRole('status')).toHaveTextContent('Outcome unconfirmed');
  await act(async () => { window.dispatchEvent(new Event('omnisolo_auth_changed')); });
  expect(screen.queryByTestId('triage-card-item-1')).not.toBeInTheDocument();
  expect(screen.queryByDisplayValue('Private prior owner draft')).not.toBeInTheDocument();
  expect(screen.queryByRole('status')).not.toBeInTheDocument();
  expect(screen.getByText('Your session changed. Reload to review work for the current account.')).toBeVisible();
  expect(mutations()).toHaveLength(1);
});

it.each([true, false])('ignores a late %s triage decision receipt after account invalidation', async (confirmed) => {
  let resolveDecision!: (response: Response) => void;
  fetcher.mockImplementationOnce(() => new Promise<Response>(resolve => { resolveDecision = resolve; }));
  const user = await openItem();
  await user.click(screen.getByTestId('triage-approve-item-1'));
  await act(async () => { window.dispatchEvent(new Event('omnisolo_auth_changed')); });
  await act(async () => resolveDecision(Response.json(confirmed ? receipt() : {})));
  expect(screen.queryByTestId('triage-card-item-1')).not.toBeInTheDocument();
  expect(screen.queryByRole('status')).not.toBeInTheDocument();
  expect(screen.getByText('Your session changed. Reload to review work for the current account.')).toBeVisible();
  expect(mutations()).toHaveLength(1);
});

it('ignores a late triage list response after account invalidation', async () => {
  let resolveList!: (response: Response) => void;
  fetcher.mockReset();
  fetcher.mockImplementationOnce(() => new Promise<Response>(resolve => { resolveList = resolve; }));
  render(<TriagePage />);
  await act(async () => { window.dispatchEvent(new Event('omnisolo_auth_changed')); });
  await act(async () => resolveList(Response.json([item])));
  expect(screen.queryByTestId('triage-card-item-1')).not.toBeInTheDocument();
  expect(screen.getByText('Your session changed. Reload to review work for the current account.')).toBeVisible();
  expect(mutations()).toHaveLength(0);
});
