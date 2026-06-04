import { render, screen, fireEvent } from '@testing-library/react';
import { ProposalCard, AgentProposal } from './ProposalCard';
import { vi, describe, it, expect } from 'vitest';

describe('ProposalCard', () => {
  const mockProposal: AgentProposal = {
    id: '1',
    department: 'OPERATIONS',
    description: 'Fix the leaky faucet',
    actionRisk: 'HIGH',
    status: 'PendingApproval',
    payload: {
      context: {
        urgency: 'Immediate',
        estimated_cost: 50.0
      }
    }
  };

  const mockApprove = vi.fn();
  const mockDecline = vi.fn();

  it('renders proposal details correctly', () => {
    render(<ProposalCard proposal={mockProposal} onApprove={mockApprove} onDecline={mockDecline} />);

    expect(screen.getByText('Fix the leaky faucet')).toBeInTheDocument();
    expect(screen.getByText('OPERATIONS')).toBeInTheDocument();
    expect(screen.getByText(/Requires Review/i)).toBeInTheDocument();
    expect(screen.getByText(/urgency/i)).toBeInTheDocument();
    expect(screen.getByText('Immediate')).toBeInTheDocument();
    expect(screen.getByText('50')).toBeInTheDocument();
  });

  it('calls onApprove when approve button is clicked', () => {
    render(<ProposalCard proposal={mockProposal} onApprove={mockApprove} onDecline={mockDecline} />);

    fireEvent.click(screen.getByRole('button', { name: /Approve proposal/i }));
    expect(mockApprove).toHaveBeenCalledWith('1');
  });

  it('calls onDecline when decline button is clicked', () => {
    render(<ProposalCard proposal={mockProposal} onApprove={mockApprove} onDecline={mockDecline} />);

    fireEvent.click(screen.getByRole('button', { name: /Decline proposal/i }));
    expect(mockDecline).toHaveBeenCalledWith('1');
  });

  it('does not render "Requires Review" for LOW risk', () => {
    const lowRiskProposal: AgentProposal = { ...mockProposal, actionRisk: 'LOW' };
    render(<ProposalCard proposal={lowRiskProposal} onApprove={mockApprove} onDecline={mockDecline} />);

    expect(screen.queryByText(/Requires Review/i)).not.toBeInTheDocument();
  });
});
