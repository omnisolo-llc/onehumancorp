import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { ActionableEmptyStateCard } from './ActionableEmptyStateCard';

describe('ActionableEmptyStateCard', () => {
  it('renders correctly with default message', () => {
    const actions = [{ label: 'Create First', onClick: vi.fn(), primary: true }];
    render(<ActionableEmptyStateCard moduleContext="orders" actions={actions} />);

    expect(screen.getByText(/You don't have any active orders yet/i)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Create First' })).toBeInTheDocument();
  });

  it('renders correctly with custom message', () => {
    const actions = [{ label: 'Import', onClick: vi.fn() }];
    render(
      <ActionableEmptyStateCard
        moduleContext="customers"
        message="Custom message here."
        actions={actions}
      />
    );

    expect(screen.getByText('Custom message here.')).toBeInTheDocument();
  });

  it('triggers onClick when button is clicked', () => {
    const mockClick = vi.fn();
    const actions = [
      { label: 'Action 1', onClick: mockClick, primary: true },
      { label: 'Action 2', onClick: vi.fn() }
    ];

    render(<ActionableEmptyStateCard moduleContext="tasks" actions={actions} />);

    fireEvent.click(screen.getByRole('button', { name: 'Action 1' }));
    expect(mockClick).toHaveBeenCalledTimes(1);
  });
});
