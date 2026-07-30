import { render, screen } from '@testing-library/react';
import { expect, test, describe, vi } from 'vitest';
import GlobalCommerceSettings from './page';

vi.mock('../../components/AppShell', () => {
    return {
        AppShell: function MockAppShell({ children }: { children: any }) { return <div data-testid="app-shell-mock">{children}</div>; },
        default: function MockAppShell({ children }: { children: any }) { return <div data-testid="app-shell-mock">{children}</div>; }
    }
});

vi.mock('@/app/components/AppShell', () => {
    return {
        AppShell: function MockAppShell({ children }: { children: any }) { return <div data-testid="app-shell-mock">{children}</div>; },
        default: function MockAppShell({ children }: { children: any }) { return <div data-testid="app-shell-mock">{children}</div>; }
    }
});

// Mock Next.js router
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('GlobalCommerceSettings', () => {
  test('renders loading state initially', () => {
    render(<GlobalCommerceSettings />);
    expect(screen.getByTestId('app-shell-mock')).toBeInTheDocument();
  });
});
