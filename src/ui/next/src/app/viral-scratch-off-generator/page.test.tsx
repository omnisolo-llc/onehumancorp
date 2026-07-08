import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ViralScratchOffGeneratorPage from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: mockPush }),
}));

vi.mock('../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc" />,
}));

describe('ViralScratchOffGeneratorPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    localStorage.clear();
  });

  it('renders correctly', () => {
    render(<ViralScratchOffGeneratorPage />);
    expect(screen.getByText('scratch off Generator 🎡')).toBeDefined();
    expect(screen.getByText('Configure Scratch Off')).toBeDefined();
  });

  it('generates embed code when button is clicked', async () => {
    render(<ViralScratchOffGeneratorPage />);

    const generateBtn = screen.getByText('Generate Widget');
    fireEvent.click(generateBtn);

    await waitFor(() => {
      expect(screen.getByText('Embed scratch off')).toBeDefined();
    });
  });

  it('shows soft paywall when toggling branding off without pro', async () => {
    render(<ViralScratchOffGeneratorPage />);

    const toggle = screen.getByRole('checkbox');
    fireEvent.click(toggle);

    await waitFor(() => {
      expect(screen.getAllByText('Upgrade to Pro').length).toBeGreaterThan(0);
    });
  });

  it('navigates back to dashboard when Back to Dashboard is clicked', () => {
    render(<ViralScratchOffGeneratorPage />);

    const backBtn = screen.getByText('Back to Dashboard');
    fireEvent.click(backBtn);

    expect(mockPush).toHaveBeenCalledWith('/dashboard');
  });

  it('allows changing the prizes text input', () => {
    render(<ViralScratchOffGeneratorPage />);

    const input = screen.getByPlaceholderText('10%, 20%, Free Shipping') as HTMLInputElement;
    fireEvent.change(input, { target: { value: '10%, 20%, 30%' } });

    expect(input.value).toBe('10%, 20%, 30%');
  });
});
