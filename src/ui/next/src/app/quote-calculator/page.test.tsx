import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ProposalCalculatorPage from './page';

const mockUseSearchParams = vi.fn();
vi.mock('next/navigation', () => ({
  useSearchParams: () => mockUseSearchParams(),
}));

describe('ProposalCalculatorPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockUseSearchParams.mockReturnValue(new URLSearchParams('?tenant=test-biz&service=Test%20Service&basePrice=100&unitName=Items&pricePerUnit=20&theme=light'));
  });

  it('renders correctly with URL parameters', () => {
    render(<ProposalCalculatorPage />);
    expect(screen.getByText('Test Service Proposal')).toBeDefined();
    expect(screen.getByText('Number of Items')).toBeDefined();
    expect(screen.getByText('$100.00')).toBeDefined(); // base price
    expect(screen.getByText('$20.00 per items')).toBeDefined(); // unit price
  });

  it('calculates total correctly when quantity changes', () => {
    render(<ProposalCalculatorPage />);

    // Initial total: 100 + (1 * 20) = 120
    expect(screen.getByText('$120.00')).toBeDefined();

    const slider = screen.getByRole('slider');
    fireEvent.change(slider, { target: { value: '5' } });

    // New total: 100 + (5 * 20) = 200
    expect(screen.getByText('$200.00')).toBeDefined();
  });
});
