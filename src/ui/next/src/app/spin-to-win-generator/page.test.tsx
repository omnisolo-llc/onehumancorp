import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import SpinToWinGeneratorPage from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: mockPush }),
}));

vi.mock('../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc" />,
}));

describe('SpinToWinGeneratorPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    localStorage.clear();
  });

  it('renders correctly', () => {
    render(<SpinToWinGeneratorPage />);
    expect(screen.getByText('Spin to Win Generator 🎡')).toBeDefined();
    expect(screen.getByText('Configure Wheel')).toBeDefined();
  });

  it('generates embed code when button is clicked', async () => {
    render(<SpinToWinGeneratorPage />);

    const generateBtn = screen.getByText('Generate Widget');
    fireEvent.click(generateBtn);

    await waitFor(() => {
      expect(screen.getByText('Embed Spin to Win')).toBeDefined();
    });
  });

  it('shows soft paywall when toggling branding off without pro', async () => {
    render(<SpinToWinGeneratorPage />);

    const toggle = screen.getByRole('checkbox');
    fireEvent.click(toggle);

    await waitFor(() => {
      expect(screen.getAllByText('Upgrade to Pro').length).toBeGreaterThan(0);
    });
  });

  it('navigates back to dashboard when Back to Dashboard is clicked', () => {
    render(<SpinToWinGeneratorPage />);

    const backBtn = screen.getByText('Back to Dashboard');
    fireEvent.click(backBtn);

    expect(mockPush).toHaveBeenCalledWith('/dashboard');
  });

  it('allows changing the prizes text input', () => {
    render(<SpinToWinGeneratorPage />);

    const input = screen.getByPlaceholderText('10%, 20%, Free Shipping') as HTMLInputElement;
    fireEvent.change(input, { target: { value: '10%, 20%, 30%' } });

    expect(input.value).toBe('10%, 20%, 30%');
  });
});
