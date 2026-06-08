import { TooltipProvider } from '../../components/TooltipRegistry';
import { render, screen, waitFor } from '@testing-library/react';
import Dashboard from './page';
import { expect, test, vi } from 'vitest';

// Mock the UnifiedAgentFeed since it fetches data
vi.mock('./UnifiedAgentFeed', () => ({
  UnifiedAgentFeed: () => <div data-testid="unified-agent-feed">Mock Feed</div>
}));

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

test('renders dashboard with unified agent feed', async () => {
  const { act } = await import('@testing-library/react');
  await act(async () => {
    render(<TooltipProvider><Dashboard /></TooltipProvider>);
  });

  await waitFor(() => {
    expect(screen.getByTestId("unified-agent-feed")).toBeDefined();
  });

  // Dashboard shell title
  expect(screen.getAllByText("Unified Agent Feed").length).toBeGreaterThan(0);
});
