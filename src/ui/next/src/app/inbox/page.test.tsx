import React from 'react';
import { render, screen } from '@testing-library/react';
import { expect, test, vi } from 'vitest';
import InboxPage from './page';

vi.mock('@powersync/react', () => ({
  useQuery: () => ({ data: [] }),
}));

vi.mock('../../lib/powersync/PowerSyncProvider', () => ({
  PowerSyncProvider: ({ children }: { children: React.ReactNode }) => children,
}));

vi.mock('../components/AppShell', () => ({
  AppShell: ({ children }: { children: React.ReactNode }) => <div>{children}</div>,
}));

test('renders a stable empty state when PowerSync has no inbox messages', () => {
  const { container } = render(<InboxPage />);

  expect(screen.getByText('No inbox messages found for this tenant.')).toBeInTheDocument();
  expect(screen.getByText('Select a database-backed message to inspect it.')).toBeInTheDocument();
  expect(container.textContent).not.toContain('\\n');
});
