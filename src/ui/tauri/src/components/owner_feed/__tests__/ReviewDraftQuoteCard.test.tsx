import React from 'react';
import { render, screen, fireEvent, cleanup } from '@testing-library/react';
import { describe, it, expect, vi, afterEach } from 'vitest';
import { ReviewDraftQuoteCard } from '../ReviewDraftQuoteCard';

describe('ReviewDraftQuoteCard', () => {
  afterEach(() => {
    cleanup();
  });

  it('renders correctly and formats currencies properly', () => {
    const mockOnApprove = vi.fn();
    const mockOnEdit = vi.fn();

    render(
      <ReviewDraftQuoteCard
        customerName="Alice"
        projectDescription="Website Redesign"
        totalCost={300000} // $3000.00
        onApprove={mockOnApprove}
        onEdit={mockOnEdit}
      />
    );

    // Assert text rendering
    expect(screen.getByText('Draft Quote Ready')).toBeDefined();
    expect(screen.getByText('Action Required')).toBeDefined();
    expect(screen.getByText('Website Redesign for Alice')).toBeDefined();

    // Assert cost formatting
    expect(screen.getByText('Total Cost:')).toBeDefined();
    expect(screen.getByText('$3000.00')).toBeDefined();

    // Assert deposit formatting (33% of 3000.00 is 1000.00)
    expect(screen.getByText('Deposit Required (33%):')).toBeDefined();
    expect(screen.getByText('$1000.00')).toBeDefined();

    // Verify buttons exist
    const approveButton = screen.getByRole('button', { name: 'Approve & Send' });
    const editButton = screen.getByRole('button', { name: 'Edit' });

    expect(approveButton).toBeDefined();
    expect(editButton).toBeDefined();
  });

  it('calls onApprove and onEdit when buttons are clicked', () => {
    const mockOnApprove = vi.fn();
    const mockOnEdit = vi.fn();

    render(
      <ReviewDraftQuoteCard
        customerName="Alice"
        projectDescription="Website Redesign"
        totalCost={300000}
        onApprove={mockOnApprove}
        onEdit={mockOnEdit}
      />
    );

    const approveButton = screen.getByRole('button', { name: 'Approve & Send' });
    fireEvent.click(approveButton);
    expect(mockOnApprove).toHaveBeenCalledTimes(1);

    const editButton = screen.getByRole('button', { name: 'Edit' });
    fireEvent.click(editButton);
    expect(mockOnEdit).toHaveBeenCalledTimes(1);
  });
});
