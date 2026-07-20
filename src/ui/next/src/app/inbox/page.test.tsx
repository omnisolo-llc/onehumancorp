import React from 'react';
import { render, screen } from '@testing-library/react';
import { beforeEach, expect, test, vi } from 'vitest';
import InboxPage from './page';

const queryState = vi.hoisted(() => ({
  data: [] as Array<Record<string, string>>,
}));

vi.mock('@powersync/react', () => ({
  useQuery: () => ({ data: queryState.data }),
}));

vi.mock('../../lib/powersync/PowerSyncProvider', () => ({
  PowerSyncProvider: ({ children }: { children: React.ReactNode }) => children,
}));

vi.mock('../components/AppShell', () => ({
  AppShell: ({ children }: { children: React.ReactNode }) => <div>{children}</div>,
}));

vi.mock('next/navigation', () => ({
  useRouter: () => ({ replace: vi.fn(), refresh: vi.fn(), push: vi.fn() }),
  useSearchParams: () => new URLSearchParams(),
  usePathname: () => '/inbox',
}));

global.fetch = vi.fn(() => Promise.resolve({ ok: true, json: () => Promise.resolve({ pending_approvals: [] }) })) as any;

beforeEach(() => {
  queryState.data = [];
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
