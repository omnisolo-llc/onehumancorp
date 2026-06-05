import { render, screen, fireEvent } from '@testing-library/react';
import { ProposalCard, AgentProposal } from './ProposalCard';
import { describe, it, expect, vi } from 'vitest';
import React from 'react';

describe('ProposalCard', () => {
  const mockProposal: AgentProposal = {
    id: '1',
    agent_id: 'marketing-agent',
    title: 'New Social Post',
    description: 'Shall I post this?',
    status: 'pending',
    created_at: new Date().toISOString(),
  };

  it('renders proposal details', () => {
    render(<ProposalCard proposal={mockProposal} onApprove={() => {}} onDecline={() => {}} />);
    expect(screen.getByText('New Social Post')).toBeDefined();
    expect(screen.getByText('Shall I post this?')).toBeDefined();
  });

  it('calls onApprove when approve button is clicked', () => {
    const onApprove = vi.fn();
    render(<ProposalCard proposal={mockProposal} onApprove={onApprove} onDecline={() => {}} />);
    fireEvent.click(screen.getByText('Approve'));
    expect(onApprove).toHaveBeenCalledWith('1');
  });

  it('shows approved status', () => {
    const approvedProposal = { ...mockProposal, status: 'approved' as const };
    render(<ProposalCard proposal={approvedProposal} onApprove={() => {}} onDecline={() => {}} />);
    expect(screen.getByText('Approved')).toBeDefined();
    expect(screen.queryByText('Approve')).toBeNull();
  });
});
