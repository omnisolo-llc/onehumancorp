import React from 'react';
import { act, fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { beforeEach, expect, test, vi } from 'vitest';
import InboxPage from './page';

const queryState = vi.hoisted(() => ({
  data: [] as Array<Record<string, string>>,
  unsupported: false,
  search: "",
}));

vi.mock('next/navigation', () => ({ useRouter: () => ({ push: vi.fn() }), useSearchParams: () => new URLSearchParams(queryState.search) }));

vi.mock('@powersync/react', () => ({
  useQuery: () => ({ data: queryState.data }),
}));

vi.mock('../../lib/powersync/PowerSyncProvider', () => ({
  PowerSyncProvider: ({ children, unsupportedFallback }: { children: React.ReactNode; unsupportedFallback: React.ReactNode }) => queryState.unsupported ? unsupportedFallback : children,
}));

vi.mock('../components/AppShell', () => ({
  AppShell: ({ children }: { children: React.ReactNode }) => <div>{children}</div>,
}));

beforeEach(() => {
  queryState.data = [];
  queryState.unsupported = false;
  queryState.search = "";
});

test('renders a stable empty state when PowerSync has no inbox messages', () => {
  const { container } = render(<InboxPage />);

  expect(screen.getByText('No inbox messages found for this tenant.')).toBeInTheDocument();
  expect(screen.getByText('Select a database-backed message to inspect it.')).toBeInTheDocument();
  expect(container.textContent).not.toContain('\\n');
});

test('renders message markup as text while preserving safe HTTPS media', () => {
  queryState.data = [{
    id: 'message-1',
    content: '<script>window.compromised = true</script>\n![Receipt](https://cdn.example.test/receipt.png)',
    draft_reply: '[Media: application/pdf - https://cdn.example.test/invoice.pdf]',
    status: 'resolved',
  }];

  const { container } = render(<InboxPage />);

  expect(screen.getByText('<script>window.compromised = true</script>')).toBeInTheDocument();
  expect(container.querySelector('script')).toBeNull();
  expect(screen.getByRole('img', { name: 'Receipt' })).toHaveAttribute(
    'src',
    'https://cdn.example.test/receipt.png',
  );
  expect(screen.getByRole('link', { name: 'Attached Media (application/pdf)' })).toHaveAttribute(
    'href',
    'https://cdn.example.test/invoice.pdf',
  );
});


test('marks the actual pending API surface busy until the workspace is committed', async () => {
  queryState.unsupported = true;
  let release!: (response: Response) => void;
  const pending = new Promise<Response>(resolve => { release = resolve; });
  const oldFetch = global.fetch;
  global.fetch = vi.fn(async url => String(url) === '/api/v1/ui/omni_inbox' ? pending : Response.json([]));
  try {
    const { container } = render(<InboxPage />);
    const loading = screen.getByText('Loading inbox messages...').closest('[aria-busy="true"]');
    expect(loading).not.toBeNull();
    expect(container.querySelector('[data-testid="inbox-settled"]')).toBeNull();
    await act(async () => { release(Response.json([])); });
    await waitFor(() => expect(screen.getByText('No inbox messages found for this tenant.')).toBeVisible());
    expect(container.querySelector('[aria-busy="true"]')).toBeNull();
    expect(container.querySelector('[data-testid="inbox-settled"]')).not.toBeNull();
  } finally { global.fetch = oldFetch; }
});

const detail = () => screen.getByText('Conversation Detail').closest('section')!;
test('opens the exact searched message from the fetched rows instead of the first inbox item', () => {
  queryState.search = 'messageId=second';
  queryState.data = [{ id: 'first', content: 'First conversation', status: 'resolved' }, { id: 'second', content: 'Searched conversation', status: 'resolved' }];
  render(<InboxPage />);
  expect(within(detail()).getByText('Searched conversation')).toBeVisible();
  expect(within(detail()).queryByText('First conversation')).toBeNull();
});
test('does not substitute another message for an absent or foreign search result', () => {
  queryState.search = 'messageId=not-in-this-workspace';
  queryState.data = [{ id: 'first', content: 'Unrelated conversation', status: 'pending' }];
  render(<InboxPage />);
  expect(within(detail()).getByText('The requested message is unavailable in this workspace.')).toBeVisible();
  expect(within(detail()).queryByText('Unrelated conversation')).toBeNull();
  expect(screen.queryByRole('button', { name: 'Send Reply' })).toBeNull();
  fireEvent.click(screen.getByRole('button', { name: /Unrelated conversation/ }));
  expect(within(detail()).getByText('Unrelated conversation')).toBeVisible();
});
test('uses a changed search destination immediately and clears an earlier reply draft', () => {
  queryState.search = 'messageId=first';
  queryState.data = [{ id: 'first', content: 'First conversation', status: 'pending' }, { id: 'second', content: 'Second conversation', status: 'pending' }];
  const view = render(<InboxPage />);
  fireEvent.change(screen.getByPlaceholderText('Type your reply here...'), { target: { value: 'Draft for first only' } });
  queryState.search = 'messageId=second'; view.rerender(<InboxPage />);
  expect(within(detail()).getByText('Second conversation')).toBeVisible();
  expect(screen.getByPlaceholderText('Type your reply here...')).toHaveValue('');
});

test('keeps unsent replies attached to their original conversation when search navigation changes', () => {
  queryState.search = 'messageId=first';
  queryState.data = [{ id: 'first', content: 'First conversation', status: 'pending' }, { id: 'second', content: 'Second conversation', status: 'pending' }];
  const view = render(<InboxPage />);
  fireEvent.change(screen.getByPlaceholderText('Type your reply here...'), { target: { value: 'First draft' } });
  queryState.search = 'messageId=second'; view.rerender(<InboxPage />);
  fireEvent.change(screen.getByPlaceholderText('Type your reply here...'), { target: { value: 'Second draft' } });
  queryState.search = 'messageId=first'; view.rerender(<InboxPage />);
  expect(screen.getByPlaceholderText('Type your reply here...')).toHaveValue('First draft');
});

test.each(['another message', 'newer text'])('does not erase %s when an older send finishes', async destination => {
  queryState.search = 'messageId=first';
  queryState.data = [{ id: 'first', content: 'First conversation', status: 'pending' }, { id: 'second', content: 'Second conversation', status: 'pending' }];
  let finish!: (response: Response) => void;
  const spy = vi.spyOn(globalThis, 'fetch').mockImplementation(async (_url, options) => options?.method === 'POST'
    ? new Promise<Response>(resolve => { finish = resolve; }) : Response.json([]));
  try {
    const view = render(<InboxPage />);
    fireEvent.change(screen.getByPlaceholderText('Type your reply here...'), { target: { value: 'Submitted first draft' } });
    fireEvent.click(screen.getByRole('button', { name: 'Send Reply' }));
    if (destination === 'another message') { queryState.search = 'messageId=second'; view.rerender(<InboxPage />); }
    fireEvent.change(screen.getByPlaceholderText('Type your reply here...'), { target: { value: 'Keep this newer draft' } });
    await act(async () => finish(Response.json({ success: true })));
    expect(screen.getByPlaceholderText('Type your reply here...')).toHaveValue('Keep this newer draft');
    if (destination === 'another message') expect(screen.queryByText('Manual reply sent.')).toBeNull();
  } finally { spy.mockRestore(); }
});

test.each(['omnisolo_auth_changed', 'storage'])('clears mounted reply drafts and retires an old completion on %s', async event => {
  queryState.search = 'messageId=first';
  queryState.data = [{ id: 'first', content: 'First conversation', status: 'pending' }];
  let finish!: (response: Response) => void;
  const spy = vi.spyOn(globalThis, 'fetch').mockImplementation(async (_url, options) => options?.method === 'POST'
    ? new Promise<Response>(resolve => { finish = resolve; }) : Response.json([]));
  try {
    render(<InboxPage />);
    fireEvent.change(screen.getByPlaceholderText('Type your reply here...'), { target: { value: 'Private old draft' } });
    fireEvent.click(screen.getByRole('button', { name: 'Send Reply' }));
    act(() => window.dispatchEvent(event === 'storage' ? new StorageEvent('storage', { key: 'omnisolo_queue_identity_epoch_v2' }) : new Event(event)));
    expect(screen.getByPlaceholderText('Type your reply here...')).toHaveValue('');
    await act(async () => finish(Response.json({ success: true })));
    expect(screen.queryByText('Manual reply sent.')).toBeNull();
  } finally { spy.mockRestore(); }
});

test('retires a manual selection when a searched destination changes and returns', () => {
  queryState.search = 'messageId=first';
  queryState.data = [{ id: 'first', content: 'First conversation', status: 'resolved' }, { id: 'second', content: 'Second conversation', status: 'resolved' }];
  const view = render(<InboxPage />);
  fireEvent.click(screen.getByRole('button', { name: /Second conversation/ }));
  queryState.search = 'messageId=second'; view.rerender(<InboxPage />);
  queryState.search = 'messageId=first'; view.rerender(<InboxPage />);
  expect(within(detail()).getByText('First conversation')).toBeVisible();
  expect(within(detail()).queryByText('Second conversation')).toBeNull();
});

test('resets original-language feedback when refreshed rows change the default selected message', () => {
  queryState.data = [{ id: 'first', content: 'First translation', original_content: 'First original', translated_from_language: 'Spanish', status: 'resolved' }];
  const view = render(<InboxPage />);
  fireEvent.click(screen.getByRole('button', { name: 'Original Spanish' }));
  expect(within(detail()).getByText('First original')).toBeVisible();
  queryState.data = [{ id: 'second', content: 'Second translation', original_content: 'Second original', translated_from_language: 'French', status: 'resolved' }];
  view.rerender(<InboxPage />);
  expect(within(detail()).getByText('Second translation')).toBeVisible();
  expect(screen.getByRole('button', { name: 'Original French' })).toBeVisible();
});
