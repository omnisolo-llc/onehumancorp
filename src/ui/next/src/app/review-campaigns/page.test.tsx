import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ReviewCampaignsPage from './page';

// Mock next/navigation
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('ReviewCampaignsPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    const localStorageMock = {
      getItem: vi.fn(),
      setItem: vi.fn(),
      clear: vi.fn()
    };
    Object.defineProperty(window, 'localStorage', {
      value: localStorageMock,
      writable: true
    });

    global.fetch = vi.fn((url) => {
      if (url === '/api/v1/growth/audience') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({ recent: 48, loyal: 12, all: 156 }),
        });
      }
      if (url === '/api/v1/growth/campaign/generate-review') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({ message: "Mocked AI message for Magic Beans" }),
        });
      }
      return Promise.reject(new Error("unknown url"));
    }) as any;
  });

  it('renders initial state correctly and fetches audience', async () => {
    render(<ReviewCampaignsPage />);
    expect(screen.getByText('Automated Review Campaigns ⭐️')).toBeDefined();
    expect(screen.getByLabelText('Product to Feature (Optional)')).toBeDefined();
    expect(screen.getByLabelText('Target Audience')).toBeDefined();
    expect(screen.getByText('Generate Email Draft')).toBeDefined();

    // Generate first to reveal Send button properly
    fireEvent.click(screen.getByText('Generate Email Draft'));

    // Wait for the audience fetch to resolve and the button text to update
    await waitFor(() => {
      expect(screen.getByText(/Send to Audience.*48.*Customers/)).toBeDefined();
    });
  });

  it('generates email draft based on backend response', async () => {
    render(<ReviewCampaignsPage />);
    const productInput = screen.getByLabelText('Product to Feature (Optional)');
    fireEvent.change(productInput, { target: { value: 'Magic Beans' } });

    const generateButton = screen.getByText('Generate Email Draft');
    fireEvent.click(generateButton);

    await waitFor(() => {
      expect(screen.getByText(/Mocked AI message for Magic Beans/)).toBeDefined();
    });
  });

  it('shows soft paywall when Pro is required but not active', async () => {
    (window.localStorage.getItem as any).mockReturnValue('false');
    render(<ReviewCampaignsPage />);

    // Generate first to reveal Send button properly
    fireEvent.click(screen.getByText('Generate Email Draft'));

    await waitFor(() => {
      expect(screen.getByText(/Send to Audience.*48.*Customers/)).toBeDefined();
    });

    const sendButton = screen.getByText(/Send to Audience/);
    fireEvent.click(sendButton);

    expect(screen.getByText('Unlock Automated Campaigns')).toBeDefined();
    expect(screen.getByText('View Plans & Upgrade')).toBeDefined();
  });

  it('shows success message when sending as a Pro user', async () => {
    (window.localStorage.getItem as any).mockReturnValue('true');
    render(<ReviewCampaignsPage />);

    // Generate first
    fireEvent.click(screen.getByText('Generate Email Draft'));

    await waitFor(() => {
      expect(screen.getByText(/Send to Audience.*48.*Customers/)).toBeDefined();
    });

    const sendButton = screen.getByText(/Send to Audience/);
    fireEvent.click(sendButton);

    await waitFor(() => {
      expect(screen.getByText(/Campaign sent to.*48.*customers!/)).toBeDefined();
    });
  });
});
