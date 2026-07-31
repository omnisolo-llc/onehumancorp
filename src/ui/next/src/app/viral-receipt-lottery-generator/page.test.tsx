import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ViralReceiptLotteryGeneratorPage from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: mockPush }),
}));

vi.mock('../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc" />,
}));

describe('ViralReceiptLotteryGeneratorPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders correctly', () => {
    render(<ViralReceiptLotteryGeneratorPage />);
    expect(screen.getByText('Viral Receipt Lottery 🎟')).toBeDefined();
  });

  it('generates a link', async () => {
    render(<ViralReceiptLotteryGeneratorPage />);
    const generateBtn = screen.getByText('Generate Link');
    fireEvent.click(generateBtn);
    expect(generateBtn.textContent).toBe('Generating...');

    await waitFor(() => {
        expect(screen.getByText('Share this link:')).toBeDefined();
    }, { timeout: 2000 });
  });
});
