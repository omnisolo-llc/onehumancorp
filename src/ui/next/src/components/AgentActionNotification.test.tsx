import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { AgentActionNotification } from './AgentActionNotification';

describe('AgentActionNotification', () => {
  const mockProps = {
    id: "evt-123",
    summary: "New plumbing inquiry from Carlos",
    draftResponse: "Hi Carlos, I can help with that. Estimated price is $150.",
    actionSummary: "Send quote for $150 and propose Tuesday",
    onApprove: vi.fn(),
    onEdit: vi.fn(),
    onDecline: vi.fn(),
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders correctly with provided props', () => {
    render(<AgentActionNotification {...mockProps} />);
    expect(screen.getByText('New plumbing inquiry from Carlos')).toBeInTheDocument();
    expect(screen.getByText('Send quote for $150 and propose Tuesday')).toBeInTheDocument();
    expect(screen.getByText('"Hi Carlos, I can help with that. Estimated price is $150."')).toBeInTheDocument();
  });

  it('calls onApprove when Approve button is clicked', () => {
    render(<AgentActionNotification {...mockProps} />);
    fireEvent.click(screen.getByText('Approve & Send'));
    expect(mockProps.onApprove).toHaveBeenCalledWith("evt-123");
  });

  it('calls onEdit when Edit button is clicked', () => {
    render(<AgentActionNotification {...mockProps} />);
    fireEvent.click(screen.getByText('Edit'));
    expect(mockProps.onEdit).toHaveBeenCalledWith("evt-123");
  });

  it('calls onDecline when Decline button is clicked', () => {
    render(<AgentActionNotification {...mockProps} />);
    fireEvent.click(screen.getByText('Decline'));
    expect(mockProps.onDecline).toHaveBeenCalledWith("evt-123");
  });
});
