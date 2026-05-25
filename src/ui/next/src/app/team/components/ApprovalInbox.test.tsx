import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import ApprovalInbox from './ApprovalInbox';
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
];

describe('ApprovalInbox', () => {
  it('renders correctly', () => {
    const onBack = vi.fn();
    const onApprove = vi.fn();
    const onReject = vi.fn();
    const onEdit = vi.fn();

    render(
      <ApprovalInbox
        departmentId="sales"
        departmentName="Sales"
        approvals={mockReqs}
        onBack={onBack}
        onApprove={onApprove}
        onReject={onReject}
        onEdit={onEdit}
      />
    );

    expect(screen.getByText('Sales')).toBeInTheDocument();
    expect(screen.getByText('Approval Inbox')).toBeInTheDocument();
    expect(screen.getByText('Test description 1')).toBeInTheDocument();
  });
});
