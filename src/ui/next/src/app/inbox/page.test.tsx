import React from 'react';
import { act, render, screen, waitFor } from '@testing-library/react';
import { beforeEach, expect, test, vi } from 'vitest';
import InboxPage from './page';

const queryState = vi.hoisted(() => ({
  data: [] as Array<Record<string, string>>,
  unsupported: false,
}));

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
