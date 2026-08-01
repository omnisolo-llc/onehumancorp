import { render, screen, waitFor } from '@testing-library/react';
import { expect, test, describe, vi, beforeEach } from 'vitest';
import { act } from 'react';
import GlobalCommerceSettings from './page';

// Mock Next.js router
vi.mock('next/navigation', () => {
  return {
    useRouter: () => ({
      push: vi.fn(),
      replace: vi.fn(),
      prefetch: vi.fn(),
      back: vi.fn(),
    }),
    usePathname: () => '/settings/global-commerce',
    useSearchParams: () => new URLSearchParams(),
  };
});

// Mock AppShell properly for Vitest
vi.mock('@/app/components/AppShell', () => {
    return {
        default: ({ children }: { children: React.ReactNode }) => <div data-testid="app-shell-mock">{children}</div>,
        AppShell: ({ children }: { children: React.ReactNode }) => <div data-testid="app-shell-mock">{children}</div>
    }
});

// Mock TopNav
vi.mock('@/app/components/TopNav', () => {
    return {
        default: () => <div data-testid="top-nav-mock">TopNav</div>
    }
});

// Mock fetch
const mockFetch = vi.fn();
global.fetch = mockFetch;

describe('GlobalCommerceSettings', () => {
  beforeEach(() => {
    mockFetch.mockReset();
  });

  test('renders loading state initially', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({})
    });

    await act(async () => {
      render(<GlobalCommerceSettings />);
    });
  });
});
