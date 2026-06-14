import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import ZeroClickBuilderPage from './page';

// Mock Next.js router
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('ZeroClickBuilderPage', () => {
  beforeEach(() => {
    // Reset fetch mock
    global.fetch = vi.fn() as unknown as typeof fetch;
    localStorage.clear();
  });

  it('renders the initial form', () => {
    render(<ZeroClickBuilderPage />);
    expect(screen.getByText('Zero-Click Business Generator')).toBeInTheDocument();
    expect(screen.getByPlaceholderText(/I am a home baker/i)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Generate My Business/i })).toBeDisabled();
  });

  it('enables the button when prompt is entered', () => {
    render(<ZeroClickBuilderPage />);
    const textarea = screen.getByPlaceholderText(/I am a home baker/i);
    fireEvent.change(textarea, { target: { value: 'I sell custom sneakers' } });
    expect(screen.getByRole('button', { name: /Generate My Business/i })).toBeEnabled();
  });

  it('submits the form and displays the result', async () => {
    // Mock the frontend fetch call
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        name: 'Custom Sneakers Store',
        url: 'https://custom-sneakers-store.ohc.app',
        products_count: 5,
      }),
    });

    render(<ZeroClickBuilderPage />);

    const textarea = screen.getByPlaceholderText(/I am a home baker/i);
    fireEvent.change(textarea, { target: { value: 'I sell custom sneakers' } });

    const button = screen.getByRole('button', { name: /Generate My Business/i });
    fireEvent.click(button);

    // Should show loading state
    expect(screen.getByText('Analyzing your business...')).toBeInTheDocument();

    // Wait for the result to appear
    await waitFor(() => {
      expect(screen.getByText('Your business is live!')).toBeInTheDocument();
    }, { timeout: 3000 });

    expect(screen.getByText('Custom Sneakers Store')).toBeInTheDocument();
    expect(screen.getByText('https://custom-sneakers-store.ohc.app')).toBeInTheDocument();
  });

  it('renders Powered by OHC branding', () => {
    render(<ZeroClickBuilderPage />);
    expect(screen.getByText(/Powered by OHC/i)).toBeInTheDocument();
  });
});
