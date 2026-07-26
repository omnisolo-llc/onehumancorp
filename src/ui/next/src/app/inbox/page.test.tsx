import { render, screen } from '@testing-library/react';
import InboxPage from './page';
import { describe, it, expect, vi } from 'vitest';

vi.mock('@powersync/react', () => ({
  useQuery: () => ({ data: [], isLoading: false, error: null })
}));

vi.mock('../../lib/powersync/PowerSyncProvider', () => ({
  PowerSyncProvider: ({ children }: { children: React.ReactNode }) => <div data-testid="powersync-provider">{children}</div>
}));

vi.mock('../components/AppShell', () => ({
  AppShell: ({ children }: { children: React.ReactNode }) => <div data-testid="app-shell">{children}</div>
}));

vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: vi.fn() })
}));

describe('InboxPage', () => {
  it('renders correctly', () => {
    const { container } = render(<InboxPage />);
    expect(screen.getByText('Unified Inbox')).toBeInTheDocument();
    expect(container.querySelector('script')).toBeNull();
  });
});
