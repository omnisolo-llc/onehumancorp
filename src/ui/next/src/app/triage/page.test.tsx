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
vi.mock('../../components/AppShell', () => {
    return {
        default: ({ children }: { children: React.ReactNode }) => <div data-testid="app-shell-mock">{children}</div>
    }
});
vi.mock('@/components/AppShell', () => {
    return {
        default: ({ children }: { children: React.ReactNode }) => <div data-testid="app-shell-mock">{children}</div>,
        AppShell: ({ children }: { children: React.ReactNode }) => <div data-testid="app-shell-mock">{children}</div>
    }
});
