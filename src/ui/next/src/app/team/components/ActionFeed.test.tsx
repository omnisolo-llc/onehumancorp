import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import ActionFeed from './ActionFeed';
import { ApprovalRequest } from '../page';

const mockReqs: ApprovalRequest[] = [
  {
    id: '123',
    tenant_id: 'tenant-1',
    department: 'sales',
    description: 'Test description 1',
    status: 'Pending',
    action_risk: 'High',
  },
  {
    id: '456',
    tenant_id: 'tenant-1',
    department: 'marketing',
    description: 'Test description 2',
    status: 'Pending',
    action_risk: 'Low',
  },
];

describe('ActionFeed', () => {
  it('renders "All Caught Up!" when there are no approvals', () => {
    const onApprove = vi.fn();
    const onReject = vi.fn();
    const onEdit = vi.fn();

    render(<ActionFeed approvals={[]} onApprove={onApprove} onReject={onReject} onEdit={onEdit} />);

    expect(screen.getByText('All Caught Up!')).toBeInTheDocument();
    expect(screen.getByText('There are no pending actions requiring your review.')).toBeInTheDocument();
  });

  it('renders approvals when they exist', () => {
    const onApprove = vi.fn();
    const onReject = vi.fn();
    const onEdit = vi.fn();

    render(<ActionFeed approvals={mockReqs} onApprove={onApprove} onReject={onReject} onEdit={onEdit} />);

    expect(screen.getByText('Test description 1')).toBeInTheDocument();
    expect(screen.getByText('Test description 2')).toBeInTheDocument();
  });
});
