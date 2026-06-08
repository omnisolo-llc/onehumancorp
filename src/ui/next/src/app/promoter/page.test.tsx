import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import PromoterPage from './page';
import { vi, describe, it, expect, beforeEach } from 'vitest';

// Mock next/navigation
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('PromoterPage', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    global.fetch = vi.fn();
    // Mock navigator.clipboard
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn().mockResolvedValue(undefined),
      },
    });
  });

  it('renders the form correctly', () => {
    render(<PromoterPage />);
    expect(screen.getByText('Promote a Product')).toBeInTheDocument();
    expect(screen.getByLabelText(/Product\/Service Name/i)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Generate Posts/i })).toBeDisabled();
  });

  it('enables the generate button when product name is entered', () => {
    render(<PromoterPage />);
    const input = screen.getByLabelText(/Product\/Service Name/i);
    fireEvent.change(input, { target: { value: 'New Cake' } });
    expect(screen.getByRole('button', { name: /Generate Posts/i })).toBeEnabled();
  });

  it('generates posts and displays them', async () => {
    const mockPosts = {
      instagram: 'IG Content ⚡ Powered by OHC',
      twitter: 'TW Content ⚡ Powered by OHC',
      email: 'EM Content ⚡ Powered by OHC'
    };

    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => mockPosts
    });

    render(<PromoterPage />);

    // Fill out form
    fireEvent.change(screen.getByLabelText(/Product\/Service Name/i), { target: { value: 'New Cake' } });
    fireEvent.change(screen.getByLabelText(/Key Selling Points/i), { target: { value: 'Delicious and sweet' } });

    // Submit
    fireEvent.click(screen.getByRole('button', { name: /Generate Posts/i }));

    // Verify loading state
    expect(screen.getByText(/Generating\.\.\./i)).toBeInTheDocument();

    // Wait for results
    await waitFor(() => {
      expect(screen.getByText('IG Content ⚡ Powered by OHC')).toBeInTheDocument();
      expect(screen.getByText('TW Content ⚡ Powered by OHC')).toBeInTheDocument();
      expect(screen.getByText('EM Content ⚡ Powered by OHC')).toBeInTheDocument();
    });

    // Verify fetch was called correctly
    expect(global.fetch).toHaveBeenCalledWith('/api/v1/growth/promoter/generate', expect.objectContaining({
      method: 'POST',
      body: expect.stringContaining('New Cake')
    }));
  });

  it('copies text to clipboard', async () => {
    const mockPosts = {
      instagram: 'IG Content',
      twitter: 'TW Content',
      email: 'EM Content'
    };

    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => mockPosts
    });

    render(<PromoterPage />);
    fireEvent.change(screen.getByLabelText(/Product\/Service Name/i), { target: { value: 'New Cake' } });
    fireEvent.click(screen.getByRole('button', { name: /Generate Posts/i }));

    await waitFor(() => {
      expect(screen.getByText('IG Content')).toBeInTheDocument();
    });

    const copyBtn = screen.getByTestId('copy-instagram');
    fireEvent.click(copyBtn);

    await waitFor(() => {
      expect(navigator.clipboard.writeText).toHaveBeenCalledWith('IG Content');
      expect(copyBtn).toHaveTextContent('Copied!');
    });
  });
});
