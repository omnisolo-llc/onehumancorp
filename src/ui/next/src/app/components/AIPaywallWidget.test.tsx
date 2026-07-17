import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { AIPaywallWidget } from './AIPaywallWidget';
import { useRouter } from 'next/navigation';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(),
}));

describe('AIPaywallWidget', () => {
  it('does not render if remaining actions > 10', () => {
    const { container } = render(<AIPaywallWidget remainingActions={15} />);
    expect(container.firstChild).toBeNull();
  });

  it('renders a warning button if remaining actions <= 10', () => {
    render(<AIPaywallWidget remainingActions={5} />);
    expect(screen.getByText('⚠️ 5 AI Actions Left')).toBeTruthy();
  });

  it('opens the modal when the warning button is clicked', () => {
    render(<AIPaywallWidget remainingActions={5} />);
    const btn = screen.getByText('⚠️ 5 AI Actions Left');
    fireEvent.click(btn);

    expect(screen.getByText("You're running low on AI power!")).toBeTruthy();
    expect(screen.getByText('Upgrade to Pro')).toBeTruthy();
  });

  it('navigates to pricing when upgrade is clicked', () => {
    const mockPush = vi.fn();
    (useRouter as any).mockReturnValue({ push: mockPush });

    render(<AIPaywallWidget remainingActions={5} />);
    fireEvent.click(screen.getByText('⚠️ 5 AI Actions Left'));

    const upgradeBtn = screen.getByText('Upgrade to Pro');
    fireEvent.click(upgradeBtn);
    expect(mockPush).toHaveBeenCalledWith('/pricing');
  });
});
