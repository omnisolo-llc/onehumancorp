import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import WebsiteBuilderPage from './page';

// Mock the walkthrough hook
vi.mock('../../components/help', () => ({
  useWalkthrough: () => ({ startWalkthrough: vi.fn() }),
  Tooltip: ({ children }: any) => <>{children}</>
}));

describe('WebsiteBuilderPage', () => {

  beforeEach(() => {
    global.fetch = vi.fn();
    localStorage.clear();
  });


  afterEach(() => {
    vi.resetAllMocks();
  });

  test('renders initial idle state correctly', () => {
    render(<WebsiteBuilderPage />);
    expect(screen.getByText('Welcome to OHC Smart Builder')).toBeInTheDocument();
    expect(screen.getByPlaceholderText(/e\.g\. I run a mobile dog grooming service in Portland/i)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Build My Storefront' })).toBeDisabled();
  });

  test('enables generate button when bio length > 5', () => {
    render(<WebsiteBuilderPage />);
    const input = screen.getByPlaceholderText(/e\.g\. I run a mobile dog grooming service in Portland/i);
    fireEvent.change(input, { target: { value: 'This is my bio' } });
    expect(screen.getByRole('button', { name: 'Build My Storefront' })).toBeEnabled();
  });

  test('handles generate workflow correctly', async () => {
    (global.fetch as vi.Mock).mockResolvedValueOnce({
      json: async () => ({
        pages: [{
          blocks: [
            { block_type: 'HeroBlock', content: { headline: 'Test Hero' } }
          ]
        }]
      })
    });

    render(<WebsiteBuilderPage />);
    const input = screen.getByPlaceholderText(/e\.g\. I run a mobile dog grooming service in Portland/i);
    fireEvent.change(input, { target: { value: 'This is my bio' } });

    const generateBtn = screen.getByRole('button', { name: 'Build My Storefront' });
    fireEvent.click(generateBtn);

    expect(screen.getByText('Agents are building your store...')).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText('Preview Mode')).toBeInTheDocument();
    });

    // Check draft blocks rendered
    // Not testing deep rendering of SmartBlock here, just presence of Launch button
    expect(screen.getByRole('button', { name: '1-Tap Launch' })).toBeInTheDocument();
  });

  test('handles publish workflow correctly', async () => {
    // Generate Phase
    (global.fetch as vi.Mock).mockResolvedValueOnce({
      json: async () => ({
        pages: [{
          blocks: [
            { block_type: 'HeroBlock', content: { headline: 'Test Hero' } }
          ]
        }]
      })
    });

    render(<WebsiteBuilderPage />);
    const input = screen.getByPlaceholderText(/e\.g\. I run a mobile dog grooming service in Portland/i);
    fireEvent.change(input, { target: { value: 'This is my bio' } });
    fireEvent.click(screen.getByRole('button', { name: 'Build My Storefront' }));

    await waitFor(() => {
      expect(screen.getByText('Preview Mode')).toBeInTheDocument();
    });

    // Publish Phase
    (global.fetch as vi.Mock).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        domain: 'test-domain'
      })
    });

    fireEvent.click(screen.getByRole('button', { name: '1-Tap Launch' }));

    await waitFor(() => {
      expect(screen.getByText("You're Live!")).toBeInTheDocument();
      expect(screen.getByText("https://test-domain.ohc.store")).toBeInTheDocument();
    });

    // Return to dashboard
    fireEvent.click(screen.getByRole('button', { name: 'Go to Dashboard' }));
    expect(screen.getByText('Welcome to OHC Smart Builder')).toBeInTheDocument();
  });

  test('handles generation error gracefully', async () => {
    (global.fetch as vi.Mock).mockRejectedValueOnce(new Error("Network Error"));

    // Spy on console.error to avoid test output noise
    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    render(<WebsiteBuilderPage />);
    const input = screen.getByPlaceholderText(/e\.g\. I run a mobile dog grooming service in Portland/i);
    fireEvent.change(input, { target: { value: 'This is my bio' } });
    fireEvent.click(screen.getByRole('button', { name: 'Build My Storefront' }));

    // Should return to idle on failure
    await waitFor(() => {
      expect(screen.getByText('Welcome to OHC Smart Builder')).toBeInTheDocument();
    });

    consoleSpy.mockRestore();
  });

  test('handles publish error gracefully', async () => {
    // Generate Phase
    (global.fetch as vi.Mock).mockResolvedValueOnce({
      json: async () => ({
        pages: [{
          blocks: [
            { block_type: 'HeroBlock', content: { headline: 'Test Hero' } }
          ]
        }]
      })
    });

    render(<WebsiteBuilderPage />);
    const input = screen.getByPlaceholderText(/e\.g\. I run a mobile dog grooming service in Portland/i);
    fireEvent.change(input, { target: { value: 'This is my bio' } });
    fireEvent.click(screen.getByRole('button', { name: 'Build My Storefront' }));

    await waitFor(() => {
      expect(screen.getByText('Preview Mode')).toBeInTheDocument();
    });

    // Publish Phase (Fail)
    (global.fetch as vi.Mock).mockResolvedValueOnce({
      ok: false
    });

    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    fireEvent.click(screen.getByRole('button', { name: '1-Tap Launch' }));

    // Should stay in draft mode on failure
    await waitFor(() => {
      expect(consoleSpy).toHaveBeenCalledWith('Failed to publish');
      expect(screen.getByText('Preview Mode')).toBeInTheDocument();
    });

    consoleSpy.mockRestore();
  });

  test('handles invalid saved blocks gracefully', () => {
    localStorage.setItem("ohc_builder_blocks", "invalid-json");
    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    render(<WebsiteBuilderPage />);
    expect(consoleSpy).toHaveBeenCalledWith("Failed to parse saved blocks", expect.any(Error));
    consoleSpy.mockRestore();
  });

  test('handles fetch errors on publish', async () => {
    // Generate Phase
    (global.fetch as vi.Mock).mockResolvedValueOnce({
      json: async () => ({
        pages: [{
          blocks: [
            { block_type: 'HeroBlock', content: { headline: 'Test Hero' } }
          ]
        }]
      })
    });

    render(<WebsiteBuilderPage />);
    const input = screen.getByPlaceholderText(/e\.g\. I run a mobile dog grooming service in Portland/i);
    fireEvent.change(input, { target: { value: 'This is my bio' } });
    fireEvent.click(screen.getByRole('button', { name: 'Build My Storefront' }));

    await waitFor(() => {
      expect(screen.getByText('Preview Mode')).toBeInTheDocument();
    });

    // Publish Phase (Fail with network error)
    (global.fetch as vi.Mock).mockRejectedValueOnce(new Error("Network Error"));

    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    fireEvent.click(screen.getByRole('button', { name: '1-Tap Launch' }));

    await waitFor(() => {
      expect(consoleSpy).toHaveBeenCalledWith('Error publishing:', expect.any(Error));
    });

    consoleSpy.mockRestore();
  });

  test('verifies design tokens: border-radius of main card is 16px', () => {
    const { container } = render(<WebsiteBuilderPage />);
    const card = container.querySelector('.shadow-2xl');
    expect(card).toHaveStyle({ borderRadius: '16px' });
  });

  test('verifies design tokens: button border-radius is 8px', () => {
    render(<WebsiteBuilderPage />);
    const btn = screen.getByRole('button', { name: 'Build My Storefront' });
    expect(btn).toHaveStyle({ borderRadius: '8px' });
  });

  test('verifies design tokens: glass background rgba is correct', () => {
    const { container } = render(<WebsiteBuilderPage />);
    const card = container.querySelector('.shadow-2xl');
    expect(card).toHaveStyle({ background: 'rgba(255, 255, 255, 0.65)' });
  });

  test('verifies design tokens: glass backdrop filter is correct', () => {
    const { container } = render(<WebsiteBuilderPage />);
    const card = container.querySelector('.shadow-2xl');
    expect(card).toHaveStyle({ backdropFilter: 'blur(30px) saturate(210%)' });
  });

  test('verifies design tokens: border is correct', () => {
    const { container } = render(<WebsiteBuilderPage />);
    const card = container.querySelector('.shadow-2xl');
    expect(card).toHaveStyle({ border: '1px solid rgba(255, 255, 255, 0.4)' });
  });
});