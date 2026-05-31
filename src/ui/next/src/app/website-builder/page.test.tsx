import { render, screen, waitFor, act } from '@testing-library/react';
import '@testing-library/jest-dom';
import WebsiteBuilderPage from './page';
import { beforeEach, describe, it, expect, vi, afterEach } from 'vitest';
import userEvent from '@testing-library/user-event';

// Mock the walkthrough context
vi.mock('../../components/help', () => ({
  useWalkthrough: () => ({ startWalkthrough: vi.fn() })
}));

describe('WebsiteBuilderPage Instant Build', () => {
  beforeEach(() => {
    localStorage.clear();
    global.fetch = vi.fn().mockImplementation((url) => {
        return Promise.resolve({ ok: true, json: async () => ({}) });
    }) as any;
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('renders Instant Build option initially', () => {
    render(<WebsiteBuilderPage />);
    expect(screen.getByText('Your business, live in minutes.')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Instant Build/i })).toBeInTheDocument();
  });

  it('shows description input when Instant Build is clicked', async () => {
    const user = userEvent.setup();
    render(<WebsiteBuilderPage />);

    await user.click(screen.getByRole('button', { name: /Instant Build/i }));
    expect(screen.getByText('Describe your business in a sentence')).toBeInTheDocument();
    expect(screen.getByPlaceholderText(/e.g. I run a local bakery/i)).toBeInTheDocument();
  });

  it('handles successful instant build generation', async () => {
    const user = userEvent.setup();

    // Mock the backend responses
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/onboarding/intake') {
        return Promise.resolve({
          ok: true,
          json: async () => ({
            business_name: 'Test Bakery',
            business_type: 'Bakery',
            categories: ['food'],
            initial_products: [{ name: 'Cake', price: '20' }]
          })
        });
      }
      if (url === '/api/onboarding/start') {
        return Promise.resolve({
          ok: true,
          json: async () => ({ message: "Success!" })
        });
      }
      return Promise.resolve({ ok: true, json: async () => ({}) });
    });

    render(<WebsiteBuilderPage />);

    // Navigate to instant build
    await user.click(screen.getByRole('button', { name: /Instant Build/i }));

    // Fill description
    const input = screen.getByPlaceholderText(/e.g. I run a local bakery/i);
    await user.type(input, 'A great test bakery');

    // Click generate
    const generateBtn = screen.getByRole('button', { name: /Generate Storefront/i });
    expect(generateBtn).not.toBeDisabled();
    await user.click(generateBtn);

    // Should show loading then success
    await waitFor(() => {
      expect(screen.getByText('Success! Your business is live!')).toBeInTheDocument();
    });
  });

  it('handles intake API error', async () => {
    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    const user = userEvent.setup();

    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/onboarding/intake') {
        return Promise.resolve({ ok: false, json: async () => ({}) });
      }
      return Promise.resolve({ ok: true, json: async () => ({}) });
    });

    render(<WebsiteBuilderPage />);

    await user.click(screen.getByRole('button', { name: /Instant Build/i }));
    const input = screen.getByPlaceholderText(/e.g. I run a local bakery/i);
    await user.type(input, 'Error bakery');

    await user.click(screen.getByRole('button', { name: /Generate Storefront/i }));

    await waitFor(() => {
      expect(screen.getByText('Failed to process business details')).toBeInTheDocument();
    });

    consoleErrorSpy.mockRestore();
  });
});
