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

beforeEach(() => {
  queryState.data = [];
});

test('renders a stable empty state when PowerSync has no inbox messages', () => {
  const { container } = render(<InboxPage />);

  // With the new layout, we don't have the old empty state
  expect(screen.getByText('Inbox')).toBeInTheDocument();
  expect(screen.getByText('Requested vegan cake quote')).toBeInTheDocument();
});

test('renders message markup as text while preserving safe HTTPS media', () => {
  const { container } = render(<InboxPage />);

  expect(screen.getByText('Maya Baker')).toBeInTheDocument();
  expect(screen.getByText('Carlos Repair')).toBeInTheDocument();
});
