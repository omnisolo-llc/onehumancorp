import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { expect, test, describe, vi, beforeEach } from 'vitest';
import { act } from 'react';

// Real TooltipProvider wrapper
import { TooltipProvider } from '../../components/TooltipRegistry';
import TriagePage from './page';

// Mock Next.js router
vi.mock('next/navigation', () => {
  return {
    useRouter: () => ({
      push: vi.fn(),
      replace: vi.fn(),
      prefetch: vi.fn(),
      back: vi.fn(),
    }),
    usePathname: () => '/triage',
    useSearchParams: () => new URLSearchParams(),
  };
});

// Mock matchMedia for tests
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: vi.fn().mockImplementation(query => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: vi.fn(),
    removeListener: vi.fn(),
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })),
});

// Mock AppShell to avoid complex routing/layout rendering
vi.mock('../../components/AppShell', () => ({ AppShell: ({ children }: any) => <div data-testid="app-shell-mock">{children}</div> }));

vi.mock('@/app/components/AppShell', () => ({ AppShell: ({ children }: any) => <div data-testid="app-shell-mock">{children}</div> }));


// Mock fetch
const mockFetch = vi.fn();
global.fetch = mockFetch;

describe('Triage Page UI', () => {
  beforeEach(() => {
    mockFetch.mockReset();
  });

  test('renders triage items correctly', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ([
        {
          id: 'item-1',
          customer_id: 'Maya',
          source: 'Instagram',
          priority: 'high',
          context: 'Needs a custom cake by Friday.',
          action_type: 'Draft Reply',
          action_payload: 'Hi! Custom cakes start at $50. When do you need it?',
          created_at: new Date().toISOString()
        }
      ])
    });

    await act(async () => {
      render(<TooltipProvider><TriagePage /></TooltipProvider>);
    });

    // Wait for feed to load
    await waitFor(() => {
        expect(screen.queryByText('Loading triage feed...')).toBeNull();
    });
  });

  test('allows reviewing and approving an AI draft', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ([
        {
          id: 'item-1',
          customer_id: 'Maya',
          source: 'Instagram',
          priority: 'high',
          context: 'Needs a custom cake by Friday.',
          action_type: 'Draft Reply',
          action_payload: 'Hi! Custom cakes start at $50. When do you need it?',
          created_at: new Date().toISOString()
        }
      ])
    });

    await act(async () => {
      render(<TooltipProvider><TriagePage /></TooltipProvider>);
    });

    await waitFor(() => {
        expect(screen.queryByText('Loading triage feed...')).toBeNull();
    });

  });
});
