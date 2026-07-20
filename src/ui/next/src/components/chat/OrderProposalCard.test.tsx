import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { OrderProposalCard } from './OrderProposalCard';

describe('OrderProposalCard', () => {
  it('renders correctly and handles actions', () => {
    const handleApprove = vi.fn();
    const handleEdit = vi.fn();
    const handleDiscard = vi.fn();

    render(
      <OrderProposalCard
        id="test-id"
        customerName="Maya Baker"
        customerEmail="maya@example.com"
        scope="Custom cake for next Friday"
        suggestedPrice={150.00}
        depositRequired={75.00}
        status="pending"
        onApprove={handleApprove}
        onEdit={handleEdit}
        onDiscard={handleDiscard}
      />
    );

    expect(screen.getByText('Order Proposal Ready')).toBeInTheDocument();
    expect(screen.getByText('For: Maya Baker (maya@example.com)')).toBeInTheDocument();
    expect(screen.getByText('Custom cake for next Friday')).toBeInTheDocument();
    expect(screen.getByText('$150.00')).toBeInTheDocument();
    expect(screen.getByText('$75.00')).toBeInTheDocument();

    fireEvent.click(screen.getByTestId('approve-proposal-btn'));
    expect(handleApprove).toHaveBeenCalledWith('test-id');

    fireEvent.click(screen.getByTestId('edit-proposal-btn'));
    expect(handleEdit).toHaveBeenCalledWith('test-id');

    fireEvent.click(screen.getByTestId('discard-proposal-btn'));
    expect(handleDiscard).toHaveBeenCalledWith('test-id');
  });

  it('hides buttons when status is approved', () => {
    const handleApprove = vi.fn();
    const handleEdit = vi.fn();

    render(
      <OrderProposalCard
        id="test-id"
        customerName="Maya Baker"
        scope="Custom cake for next Friday"
        suggestedPrice={150.00}
        depositRequired={75.00}
        status="approved"
        onApprove={handleApprove}
        onEdit={handleEdit}
      />
    );

    expect(screen.queryByTestId('approve-proposal-btn')).not.toBeInTheDocument();
    expect(screen.queryByTestId('edit-proposal-btn')).not.toBeInTheDocument();
    expect(screen.getByText('Approved')).toBeInTheDocument();
  });
});
