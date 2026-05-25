import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import '@testing-library/jest-dom';
import ApprovalCard from './ApprovalCard';
import { ApprovalRequest } from '../page';

const mockReq: ApprovalRequest = {
  id: '123',
  tenant_id: 'tenant-1',
  department: 'sales',
  description: 'Test description',
  status: 'Pending',
  action_risk: 'High',
};

describe('ApprovalCard', () => {
  it('renders correctly', () => {
    const onApprove = vi.fn();
    const onReject = vi.fn();
    const onEdit = vi.fn();

    render(<ApprovalCard req={mockReq} onApprove={onApprove} onReject={onReject} onEdit={onEdit} />);

    expect(screen.getByText('High Risk')).toBeInTheDocument();
    expect(screen.getByText('Pending')).toBeInTheDocument();
    expect(screen.getByText('Test description')).toBeInTheDocument();
  });

  it('calls onApprove when Approve is clicked', () => {
    const onApprove = vi.fn();
    const onReject = vi.fn();
    const onEdit = vi.fn();

    render(<ApprovalCard req={mockReq} onApprove={onApprove} onReject={onReject} onEdit={onEdit} />);

    fireEvent.click(screen.getByText('Approve'));
    expect(onApprove).toHaveBeenCalledWith('123');
  });

  it('calls onReject when Reject is clicked', () => {
    const onApprove = vi.fn();
    const onReject = vi.fn();
    const onEdit = vi.fn();

    render(<ApprovalCard req={mockReq} onApprove={onApprove} onReject={onReject} onEdit={onEdit} />);

    fireEvent.click(screen.getByText('Reject'));
    expect(onReject).toHaveBeenCalledWith('123');
  });

  it('calls onEdit when Edit is clicked', () => {
    const onApprove = vi.fn();
    const onReject = vi.fn();
    const onEdit = vi.fn();

    render(<ApprovalCard req={mockReq} onApprove={onApprove} onReject={onReject} onEdit={onEdit} />);

    fireEvent.click(screen.getByText('Edit'));
    expect(onEdit).toHaveBeenCalledWith('123');
  });
});
