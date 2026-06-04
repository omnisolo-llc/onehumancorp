import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import '@testing-library/jest-dom';
import UnifiedAgentFeed from './page';

describe('UnifiedAgentFeed', () => {
  it('renders the feed header and proposals', () => {
    render(<UnifiedAgentFeed />);
    expect(screen.getByText('Unified Agent Feed')).toBeInTheDocument();
    expect(screen.getByText('New Social Media Campaign')).toBeInTheDocument();
  });

  it('allows approving a proposal', () => {
    render(<UnifiedAgentFeed />);
    const approveButtons = screen.getAllByText('Approve');
    fireEvent.click(approveButtons[0]);
    expect(screen.getByText('Approved')).toBeInTheDocument();
  });

  it('allows rejecting a proposal', () => {
    render(<UnifiedAgentFeed />);
    const rejectButtons = screen.getAllByText('Reject');
    fireEvent.click(rejectButtons[0]);
    expect(screen.getByText('Rejected')).toBeInTheDocument();
  });

  it('shows empty state when all are resolved', () => {
    render(<UnifiedAgentFeed />);
    const approveButtons = screen.getAllByText('Approve');
    fireEvent.click(approveButtons[0]);
    const rejectButtons = screen.getAllByText('Reject');
    fireEvent.click(rejectButtons[0]);
    const approveButtons2 = screen.getAllByText('Approve');
    fireEvent.click(approveButtons2[0]);
    expect(screen.getByText('No pending proposals. You\'re all caught up!')).toBeInTheDocument();
  });
});
