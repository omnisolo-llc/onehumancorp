import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { AgentActionCard } from './AgentActionCard';
import '@testing-library/jest-dom';

describe('AgentActionCard', () => {
  const defaultApproval = {
    id: 'msg_1',
    event_source: 'Test event',
    tenant_id: 'tenant_1',
    agent_id: 'agent_1',
    status: 'pending',
    created_at: new Date().toISOString(),
    proposed_action: null,
    context_payload: null
  };

  it('renders standard layout without error', () => {
    render(<AgentActionCard approval={defaultApproval} handleDecision={vi.fn()} loadingAction={null} queuedActionIds={new Set()} setEditingId={vi.fn()} editingId={null} setEditContent={vi.fn()} editContent="" />);
    expect(screen.getAllByText('Test event')[0]).toBeInTheDocument();
    expect(screen.getByTestId('feed-approve-btn')).toBeInTheDocument();
    expect(screen.getByTestId('feed-dismiss-btn')).toBeInTheDocument();
  });

  it('handles approve click', () => {
    const handleDecision = vi.fn();
    render(<AgentActionCard approval={defaultApproval} handleDecision={handleDecision} loadingAction={null} queuedActionIds={new Set()} setEditingId={vi.fn()} editingId={null} setEditContent={vi.fn()} editContent="" />);

    fireEvent.click(screen.getByTestId('feed-approve-btn'));
    expect(handleDecision).toHaveBeenCalledWith('msg_1', true, undefined, 'Test event');
  });

  it('handles dismiss click', () => {
    const handleDecision = vi.fn();
    render(<AgentActionCard approval={defaultApproval} handleDecision={handleDecision} loadingAction={null} queuedActionIds={new Set()} setEditingId={vi.fn()} editingId={null} setEditContent={vi.fn()} editContent="" />);

    fireEvent.click(screen.getByTestId('feed-dismiss-btn'));
    expect(handleDecision).toHaveBeenCalledWith('msg_1', false, undefined, 'Test event');
  });

  it('triggers edit flow', () => {
    const setEditingId = vi.fn();
    render(<AgentActionCard approval={defaultApproval} handleDecision={vi.fn()} loadingAction={null} queuedActionIds={new Set()} setEditingId={setEditingId} editingId={null} setEditContent={vi.fn()} editContent="" />);

    fireEvent.click(screen.getByTestId('edit-proposal'));
    expect(setEditingId).toHaveBeenCalledWith('msg_1');
  });

  it('renders editing state', () => {
    render(<AgentActionCard approval={defaultApproval} handleDecision={vi.fn()} loadingAction={null} queuedActionIds={new Set()} setEditingId={vi.fn()} editingId="msg_1" setEditContent={vi.fn()} editContent="Edit text" />);

    expect(screen.getByTestId('edit-proposal-textarea')).toBeInTheDocument();
    expect(screen.getByTestId('save-proposal')).toBeInTheDocument();
    expect(screen.getByTestId('cancel-edit-proposal')).toBeInTheDocument();
  });


  it('renders invoice_draft properly', () => {
    const approval = {
      id: 'inv-1',
      tenant_id: 'default',
      lifecycle_state: 'PENDING_APPROVAL',
      event_source: 'finance_agent',
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
      context_payload: {
        feature_type: 'invoice_draft',
        project_name: 'Q3 Website Redesign',
        milestone_name: 'Design Phase',
        amount_cents: 150000,
        customer_name: 'Acme Corp'
      }
    };
    const { getByText, getByTestId } = render(<AgentActionCard approval={approval as any} handleDecision={vi.fn()} loadingAction={null} queuedActionIds={new Set()} setEditingId={vi.fn()} editingId={null} setEditContent={vi.fn()} editContent="" wrapDecision={vi.fn()} isActionLoading={() => false} />);
    expect(getByText('Generated Invoice')).toBeInTheDocument();
    expect(getByText('Q3 Website Redesign')).toBeInTheDocument();
    expect(getByText('Design Phase')).toBeInTheDocument();
    expect(getByText('$1500.00')).toBeInTheDocument();
    expect(getByTestId('feed-approve-btn')).toBeInTheDocument();
  });

  it('renders invoice_followup properly', () => {
    const approval = {
      id: 'inv-2',
      tenant_id: 'default',
      lifecycle_state: 'PENDING_APPROVAL',
      event_source: 'customer_success_agent',
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
      context_payload: {
        feature_type: 'invoice_followup',
        original_message: 'Acme Corp invoice is 3 days overdue.',
        generated_response: 'Hi Acme Corp, just a polite reminder...',
        suggested_channel: 'email'
      }
    };
    const wrapDecision = vi.fn();
    const { getByText, getByTestId } = render(<AgentActionCard approval={approval as any} handleDecision={vi.fn()} loadingAction={null} queuedActionIds={new Set()} setEditingId={vi.fn()} editingId={null} setEditContent={vi.fn()} editContent="" wrapDecision={wrapDecision} isActionLoading={() => false} />);
    expect(getByText('Acme Corp invoice is 3 days overdue.')).toBeInTheDocument();
    expect(getByText('Hi Acme Corp, just a polite reminder...')).toBeInTheDocument();
    expect(getByTestId('feed-approve-btn')).toBeInTheDocument();
  });
});