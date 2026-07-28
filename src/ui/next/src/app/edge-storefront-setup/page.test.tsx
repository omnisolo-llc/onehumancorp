import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { expect, test, describe, vi, beforeEach } from 'vitest';
import EdgeStorefrontSetupPage from './page';

// Mock Next.js router
vi.mock('next/navigation', () => {
  return {
    useRouter: () => ({
      push: vi.fn(),
      replace: vi.fn(),
      prefetch: vi.fn(),
      back: vi.fn(),
    }),
  };
});

// Mock AppShell to avoid complex routing/layout rendering
vi.mock("../components/AppShell", () => {
    return {
        AppShell: ({ children }: { children: React.ReactNode }) => <div data-testid="app-shell-mock">{children}</div>
    }
});

describe('Edge Storefront Setup Page UI', () => {
  test('renders storefront setup correctly', async () => {
    render(<EdgeStorefrontSetupPage />);

    // Using testid of mock to check if it renders properly
    expect(screen.getByTestId('app-shell-mock')).toBeDefined();

    // Let's verify we have the initial content
    expect(screen.getByText(/Publish Storefront/i)).toBeDefined();
    expect(screen.getByRole('button', { name: /Start Setup/i })).toBeDefined();
  });
});
