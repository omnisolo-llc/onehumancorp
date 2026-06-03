import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import SalesAcquisitionPage from './page';

// Mock useRouter
vi.mock('next/navigation', () => ({
  useRouter() {
    return {
      push: vi.fn(),
      replace: vi.fn(),
      prefetch: vi.fn(),
    };
  },
}));

describe('Sales & Acquisition Page', () => {
  it('renders correctly', () => {
    render(<SalesAcquisitionPage />);
    expect(screen.getByText('Sales & Acquisition')).toBeInTheDocument();
    expect(screen.getByText('Autonomous Quoting')).toBeInTheDocument();
  });

  it('can toggle autonomous quoting and enter pricing rules', () => {
    render(<SalesAcquisitionPage />);

    // Initially not showing rules
    expect(screen.queryByPlaceholderText('e.g. $50/hr base, plus materials')).not.toBeInTheDocument();

    // Toggle on
    const toggle = screen.getByRole('checkbox');
    fireEvent.click(toggle);

    // Should now show rules text area
    const rulesInput = screen.getByPlaceholderText('e.g. $50/hr base, plus materials');
    expect(rulesInput).toBeInTheDocument();

    fireEvent.change(rulesInput, { target: { value: '$60/hr' } });
    expect(rulesInput).toHaveValue('$60/hr');
  });
});
