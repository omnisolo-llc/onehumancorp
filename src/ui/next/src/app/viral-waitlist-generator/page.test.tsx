import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ViralWaitlistGeneratorPage from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: mockPush }),
}));

vi.mock('../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc" />,
}));

describe('ViralWaitlistGeneratorPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    localStorage.clear();
  });

  it('renders correctly', () => {
    render(<ViralWaitlistGeneratorPage />);
    expect(screen.getByText('Viral Waitlist Generator 🚀')).toBeDefined();
    expect(screen.getByText('Product/Service Name')).toBeDefined();
  });

  it('generates embed code when button is clicked', async () => {
    render(<ViralWaitlistGeneratorPage />);

    const generateBtn = screen.getByText('Generate Widget');
    fireEvent.click(generateBtn);

    await waitFor(() => {
      expect(screen.getByText('Embed Waitlist')).toBeDefined();
    });
  });

  it('shows soft paywall when toggling branding off without pro', async () => {
    render(<ViralWaitlistGeneratorPage />);

    const toggle = screen.getByRole('checkbox');
    fireEvent.click(toggle);

    await waitFor(() => {
      expect(screen.getAllByText('Upgrade to Pro').length).toBeGreaterThan(0);
    });
  });

  it('navigates back to dashboard when Back to Dashboard is clicked', () => {
    render(<ViralWaitlistGeneratorPage />);

    const backBtn = screen.getByText('← Back to Dashboard');
    fireEvent.click(backBtn);

    expect(mockPush).toHaveBeenCalledWith('/dashboard');
  });
});
