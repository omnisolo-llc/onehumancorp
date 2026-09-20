import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import EdgeStorefrontSetupPage from './page';

// Mock Next.js router
const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: mockPush,
  }),
}));

// Mock AppShell so we just render children
vi.mock('../components/AppShell', () => ({
  AppShell: ({ children }: { children: React.ReactNode }) => <div data-testid="app-shell">{children}</div>,
}));

describe('EdgeStorefrontSetupPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders initial setup view', () => {
    render(<EdgeStorefrontSetupPage />);
    expect(screen.getByText('Publish Storefront')).toBeInTheDocument();
    expect(screen.getByText('Get your business online instantly with our AI Promoter Agent.')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Start Setup/i })).toBeInTheDocument();
  });

  it('navigates back to dashboard when back button is clicked', () => {
    render(<EdgeStorefrontSetupPage />);
    fireEvent.click(screen.getByRole('button', { name: /Go back/i }));
    expect(mockPush).toHaveBeenCalledWith('/dashboard');
  });

  it('transitions to setup step on Start Setup click', () => {
    render(<EdgeStorefrontSetupPage />);
    fireEvent.click(screen.getByRole('button', { name: /Start Setup/i }));

    expect(screen.getByText('Promoter Agent')).toBeInTheDocument();
    expect(screen.getByText('Custom Cakes')).toBeInTheDocument();
    expect(screen.getByText('Ready-to-buy')).toBeInTheDocument();

    // Generate button should be disabled initially
    const generateBtn = screen.getByRole('button', { name: /Generate & Publish/i });
    expect(generateBtn).toBeDisabled();
  });

  it('enables generate button when an option is selected', () => {
    render(<EdgeStorefrontSetupPage />);
    fireEvent.click(screen.getByRole('button', { name: /Start Setup/i }));

    const customCakesBtn = screen.getByText('Custom Cakes');
    fireEvent.click(customCakesBtn);

    const generateBtn = screen.getByRole('button', { name: /Generate & Publish/i });
    expect(generateBtn).toBeEnabled();
  });

  it('shows error when generate is clicked', async () => {
    render(<EdgeStorefrontSetupPage />);
    fireEvent.click(screen.getByRole('button', { name: /Start Setup/i }));

    const readyToBuyBtn = screen.getByText('Ready-to-buy');
    fireEvent.click(readyToBuyBtn);

    const generateBtn = screen.getByRole('button', { name: /Generate & Publish/i });
    fireEvent.click(generateBtn);

    await waitFor(() => {
      expect(screen.getByText('Storefront publishing is unavailable because no edge-publishing API is connected.')).toBeInTheDocument();
    });
  });
});
