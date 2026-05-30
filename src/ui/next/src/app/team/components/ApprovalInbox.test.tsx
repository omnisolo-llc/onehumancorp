import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import ApprovalInbox from './ApprovalInbox';

describe('ApprovalInbox', () => {
  it('renders safely when description has no payload', () => {
    const mockApproval = {
      id: '1',
      tenant_id: 't1',
      department: 'sales',
      description: 'Test description without payload',
      status: 'pending',
      action_risk: 'low'
    };
    render(
      <ApprovalInbox
        departmentId="sales"
        departmentName="Sales"
        approvals={[mockApproval]}
        onBack={() => {}}
        onApprove={() => {}}
        onReject={() => {}}
      />
    );
    expect(screen.getByText('Test description without payload')).toBeInTheDocument();
  });
});
