import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import CapitalPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('CapitalPage', () => {
  it('renders offers after loading', async () => {
    render(<CapitalPage />);
    await waitFor(() => {
      expect(screen.getByText('Capital Offers')).toBeInTheDocument();
    });
    expect(screen.getByText('Advance: $2000.00')).toBeInTheDocument();
    expect(screen.getByText('One-time fee: $150.00')).toBeInTheDocument();
    expect(screen.getByText(/Repayment: 10% of daily sales until repaid/i)).toBeInTheDocument();
  });

  it('allows accepting an offer', async () => {
    const alertMock = vi.spyOn(window, 'alert').mockImplementation(() => {});
    render(<CapitalPage />);

    await waitFor(() => {
      expect(screen.getByText('Accept Offer')).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText('Accept Offer'));

    await waitFor(() => {
      expect(screen.getByText('Accepted')).toBeInTheDocument();
    });

    expect(alertMock).toHaveBeenCalledWith('Offer accepted! Funds deposited to your wallet.');
    alertMock.mockRestore();
  });
});
