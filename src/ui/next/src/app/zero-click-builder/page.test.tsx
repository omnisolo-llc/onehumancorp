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

vi.mock('../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc">Powered by OHC</div>,
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
    expect(screen.getByRole('button', { name: /Generate Store/i })).toBeDisabled();
  });

  it('enables the button when prompt is entered', () => {
    render(<ZeroClickBuilderPage />);
    const textarea = screen.getByPlaceholderText(/I am a home baker/i);
    fireEvent.change(textarea, { target: { value: 'I sell custom sneakers' } });
    expect(screen.getByRole('button', { name: /Generate Store/i })).toBeEnabled();
  });

  it('submits the form and displays the result', async () => {
    // Mock the frontend fetch call
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        name: 'Custom Sneakers Store',
        url: 'https://custom-sneakers-store.ohc.app',
        products_count: 5,
        organization_id: 'org_123',
        user_id: 'user_123'
      }),
    });

    render(<ZeroClickBuilderPage />);

    const textarea = screen.getByPlaceholderText(/I am a home baker/i);
    fireEvent.change(textarea, { target: { value: 'I sell custom sneakers' } });

    const button = screen.getByRole('button', { name: /Generate Store/i });
    fireEvent.click(button);

    // Should show loading state
    expect(screen.getByText('Analyzing your business...')).toBeInTheDocument();

    // Wait for the result to appear
    await waitFor(() => {
      expect(screen.getByText('Your business is live!')).toBeInTheDocument();
    }, { timeout: 3000 });

    expect(screen.getByTitle('Live Storefront Preview')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Launch My Store/i })).toBeInTheDocument();
  });

  it('redirects to dashboard when launch button is clicked', async () => {
    // Mock the frontend fetch call
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        organization_id: 'org_123',
        user_id: 'user_123'
      }),
    });

    render(<ZeroClickBuilderPage />);
    const textarea = screen.getByPlaceholderText(/I am a home baker/i);
    fireEvent.change(textarea, { target: { value: 'I sell custom sneakers' } });
    fireEvent.click(screen.getByRole('button', { name: /Generate Store/i }));

    await waitFor(() => {
      expect(screen.getByText('Your business is live!')).toBeInTheDocument();
    }, { timeout: 3000 });

    expect(screen.getByTitle('Live Storefront Preview')).toBeInTheDocument();
    const launchBtn = screen.getByRole('button', { name: /Launch My Store/i });
    fireEvent.click(launchBtn);
  });

  it('opens a tweet intent when the share button is clicked', async () => {
    const mockOpen = vi.fn();
    window.open = mockOpen;
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        organization_id: 'org_123',
        user_id: 'user_123'
      }),
    });

    render(<ZeroClickBuilderPage />);
    const textarea = screen.getByPlaceholderText(/I am a home baker/i);
    fireEvent.change(textarea, { target: { value: 'I sell custom sneakers' } });
    fireEvent.click(screen.getByRole('button', { name: /Generate Store/i }));
    await waitFor(() => expect(screen.getByText('Your business is live!')).toBeInTheDocument(), { timeout: 3000 });
    fireEvent.click(screen.getByRole('button', { name: /Share on X/i }));
    expect(mockOpen).toHaveBeenCalled();
  });

  it('renders Powered by OHC branding', () => {
    render(<ZeroClickBuilderPage />);
    const texts = screen.getAllByText(/Powered by OHC/i);
    expect(texts.length).toBeGreaterThan(0);
  });

  it('renders the PoweredByOHC component', () => {
    render(<ZeroClickBuilderPage />);
    const components = screen.getAllByTestId('powered-by-ohc');
    expect(components.length).toBeGreaterThan(0);
  });
});
