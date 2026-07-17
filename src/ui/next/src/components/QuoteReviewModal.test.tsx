import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { ProposalReviewModal } from './ProposalReviewModal';
import React from 'react';

describe('ProposalReviewModal', () => {
  const mockPayload = {
    feature_type: 'proposal_draft',
    service: 'Test Service',
    price: 100.0,
    customer_inquiry: 'I need a test service'
  };

  it('renders correctly when open', () => {
    render(
      <ProposalReviewModal
        isOpen={true}
        onClose={vi.fn()}
        onApprove={vi.fn()}
        initialPayload={mockPayload}
      />
    );
    expect(screen.getByText('Review Proposal')).toBeDefined();
    expect(screen.getByDisplayValue('Test Service')).toBeDefined();
    expect(screen.getByTestId('modal-proposal-total')).toHaveTextContent('$100.00');
  });

  it('updates total when price changes', () => {
    render(
      <ProposalReviewModal
        isOpen={true}
        onClose={vi.fn()}
        onApprove={vi.fn()}
        initialPayload={mockPayload}
      />
    );
    const priceInput = screen.getByDisplayValue('100.00');
    fireEvent.change(priceInput, { target: { value: '150.00' } });
    expect(screen.getByTestId('modal-proposal-total')).toHaveTextContent('$150.00');
  });

  it('calls onApprove with updated payload', () => {
    const onApprove = vi.fn();
    render(
      <ProposalReviewModal
        isOpen={true}
        onClose={vi.fn()}
        onApprove={onApprove}
        initialPayload={mockPayload}
      />
    );

    const approveBtn = screen.getByTestId('modal-approve-btn');
    fireEvent.click(approveBtn);

    expect(onApprove).toHaveBeenCalled();
    const calledPayload = onApprove.mock.calls[0][0];
    expect(calledPayload.suggested_price).toBe(100.0);
  });
});
