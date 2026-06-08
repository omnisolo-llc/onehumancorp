import { TooltipProvider } from '../../components/TooltipRegistry';
import { render, screen, waitFor } from '@testing-library/react';
import Dashboard from './page';
import { FloatingActionButton } from './components/FAB';
vi.mock('./components/FAB', () => ({
  FloatingActionButton: () => <div data-testid="mock-fab">Mock FAB</div>
}));

vi.mock('./UnifiedAgentFeed', () => ({
  UnifiedAgentFeed: () => <div data-testid="unified-agent-feed">Mock Feed</div>
}));
import { expect, test, vi } from 'vitest';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
    replace: vi.fn(),
    prefetch: vi.fn(),
  }),
  usePathname: () => '',
  useSearchParams: () => new URLSearchParams(),
}));

// Mock fetch to prevent valid Undici errors regarding absolute URLs or missing globals
global.fetch = vi.fn(() => Promise.resolve({
  ok: true,
  json: () => Promise.resolve({})
})) as any;

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
    replace: vi.fn(),
    prefetch: vi.fn(),
    back: vi.fn(),
    forward: vi.fn(),
    refresh: vi.fn(),
    pathname: '/',
    query: {},
  }),
  usePathname: () => '/',
  useSearchParams: () => new URLSearchParams(),
}));

test('renders dashboard with actionable feed', async () => {
  const { act } = await import('@testing-library/react');
  await act(async () => {
    render(<TooltipProvider><Dashboard /></TooltipProvider>);
  });

  await waitFor(() => {
    expect(screen.getAllByText("Business Analytics").length).toBeGreaterThan(0);
    expect(screen.getByTestId("unified-agent-feed")).toBeDefined();
  });

  expect(screen.getByTestId("unified-agent-feed")).toBeDefined();
});
