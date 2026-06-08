import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { ActionCard } from './ActionCard';
import { describe, it, expect, beforeEach, vi } from 'vitest';

describe('ActionCard', () => {
  const defaultProps = {
    id: 'test-card-1',
    department: 'Sales',
    description: 'Create a lead for John Doe',
    status: 'pending' as const,
    onApprove: vi.fn(),
    onEdit: vi.fn(),
    onDiscard: vi.fn(),
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders generic pending action correctly', () => {
    render(<ActionCard {...defaultProps} />);

    expect(screen.getByTestId('action-card')).toBeInTheDocument();
    expect(screen.getByText('Needs Approval')).toBeInTheDocument();
    expect(screen.getByText('Sales')).toBeInTheDocument();
    expect(screen.getByText('Create a lead for John Doe')).toBeInTheDocument();

    expect(screen.getByTestId('approve-action-btn')).toHaveTextContent('Approve & Execute');
    expect(screen.getByText('Edit Details')).toBeInTheDocument();
  });

  it('renders quote draft correctly', () => {
    render(
      <ActionCard
        {...defaultProps}
        featureType="quote_draft"
        scope="Replace roof"
        suggestedPrice={5000}
      />
    );

    expect(screen.getByTestId('draft-quote-card')).toBeInTheDocument();
    expect(screen.getByText('Draft Quote: Sales for Customer')).toBeInTheDocument();
    expect(screen.getByText('Scope of Work: Replace roof')).toBeInTheDocument();
    expect(screen.getByText('Calculated Total: $5000')).toBeInTheDocument();
    expect(screen.getByTestId('approve-action-btn')).toHaveTextContent('Approve & Send');
    expect(screen.getByText('Discard')).toBeInTheDocument();
  });

  it('handles approve click', () => {
    render(<ActionCard {...defaultProps} />);

    fireEvent.click(screen.getByTestId('approve-action-btn'));
    expect(defaultProps.onApprove).toHaveBeenCalledWith('test-card-1');
  });

  it('handles edit click', () => {
    render(<ActionCard {...defaultProps} />);

    fireEvent.click(screen.getByText('Edit Details'));
    expect(defaultProps.onEdit).toHaveBeenCalledWith('Create a lead for John Doe');
  });

  it('handles discard click on quote draft', () => {
    render(<ActionCard {...defaultProps} featureType="quote_draft" />);

    fireEvent.click(screen.getByText('Discard'));
    expect(defaultProps.onDiscard).toHaveBeenCalledWith('test-card-1');
  });

  it('renders approved status correctly', () => {
    render(<ActionCard {...defaultProps} status="approved" />);
    expect(screen.getByText('Approved')).toBeInTheDocument();
    expect(screen.queryByTestId('approve-action-btn')).not.toBeInTheDocument();
  });

  it('renders rejected status correctly', () => {
    render(<ActionCard {...defaultProps} status="rejected" />);
    expect(screen.getByText('Rejected')).toBeInTheDocument();
    expect(screen.queryByTestId('approve-action-btn')).not.toBeInTheDocument();
  });
});
