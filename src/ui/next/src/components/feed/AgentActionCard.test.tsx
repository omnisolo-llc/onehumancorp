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
    lifecycle_state: 'PENDING_APPROVAL',
    created_at: new Date().toISOString(),
    proposed_action: null,
    context_payload: null
  };

  it('renders standard layout without error', () => {
    render(<AgentActionCard approval={defaultApproval} handleDecision={vi.fn()} queuedActionIds={new Set()} setEditingId={vi.fn()} editingId={null} setEditContent={vi.fn()} editContent="" editQuotePrice="" editQuoteScope="" setEditQuotePrice={vi.fn()} setEditQuoteScope={vi.fn()} />);
    expect(screen.getAllByText('Test event')[0]).toBeInTheDocument();
    expect(screen.getByTestId('feed-approve-btn')).toBeInTheDocument();
    expect(screen.getByTestId('feed-dismiss-btn')).toBeInTheDocument();
  });

  it('handles approve click', () => {
    const handleDecision = vi.fn();
    render(<AgentActionCard approval={defaultApproval} handleDecision={handleDecision} queuedActionIds={new Set()} setEditingId={vi.fn()} editingId={null} setEditContent={vi.fn()} editContent="" editQuotePrice="" editQuoteScope="" setEditQuotePrice={vi.fn()} setEditQuoteScope={vi.fn()} />);

    fireEvent.click(screen.getByTestId('feed-approve-btn'));
    expect(handleDecision).toHaveBeenCalledWith('msg_1', true, undefined, 'Test event');
  });

  it('handles dismiss click', () => {
    const handleDecision = vi.fn();
    render(<AgentActionCard approval={defaultApproval} handleDecision={handleDecision} queuedActionIds={new Set()} setEditingId={vi.fn()} editingId={null} setEditContent={vi.fn()} editContent="" editQuotePrice="" editQuoteScope="" setEditQuotePrice={vi.fn()} setEditQuoteScope={vi.fn()} />);

    fireEvent.click(screen.getByTestId('feed-dismiss-btn'));
    expect(handleDecision).toHaveBeenCalledWith('msg_1', false, undefined, 'Test event');
  });

  it('triggers edit flow', () => {
    const setEditingId = vi.fn();
    render(<AgentActionCard approval={defaultApproval} handleDecision={vi.fn()} queuedActionIds={new Set()} setEditingId={setEditingId} editingId={null} setEditContent={vi.fn()} editContent="" editQuotePrice="" editQuoteScope="" setEditQuotePrice={vi.fn()} setEditQuoteScope={vi.fn()} />);

    fireEvent.click(screen.getByTestId('edit-proposal'));
    expect(setEditingId).toHaveBeenCalledWith('msg_1');
  });

  it('renders editing state', () => {
    render(<AgentActionCard approval={defaultApproval} handleDecision={vi.fn()} queuedActionIds={new Set()} setEditingId={vi.fn()} editingId="msg_1" setEditContent={vi.fn()} editContent="Edit text" editQuotePrice="" editQuoteScope="" setEditQuotePrice={vi.fn()} setEditQuoteScope={vi.fn()} />);

    expect(screen.getByTestId('edit-proposal-textarea')).toBeInTheDocument();
    expect(screen.getByTestId('save-proposal')).toBeInTheDocument();
    expect(screen.getByTestId('cancel-edit-proposal')).toBeInTheDocument();
  });

  it('renders dynamic pricing recommendation layout', () => {
    const dynamicPricingApproval = {
      ...defaultApproval,
      proposed_action: {
        type: 'dynamic_pricing_recommendation',
        rule_config: {
          name: 'Clearance: Summer Hats'
        }
      }
    };
    render(<AgentActionCard approval={dynamicPricingApproval} handleDecision={vi.fn()} queuedActionIds={new Set()} setEditingId={vi.fn()} editingId={null} setEditContent={vi.fn()} editContent="" editQuotePrice="" editQuoteScope="" setEditQuotePrice={vi.fn()} setEditQuoteScope={vi.fn()} />);

    expect(screen.getByText('Apply Dynamic Rule')).toBeInTheDocument();
    expect(screen.getByText('Clearance: Summer Hats')).toBeInTheDocument();
    expect(screen.getByTestId('approve-dynamic-pricing')).toBeInTheDocument();
    expect(screen.getByTestId('adjust-dynamic-pricing')).toBeInTheDocument();
  });

  it('renders yield management recommendation layout', () => {
    const yieldManagementApproval = {
      ...defaultApproval,
      proposed_action: {
        type: 'yield_management_recommendation',
        rule_config: {
          name: 'Peak Surge: Guitar Lesson'
        }
      }
    };
    render(<AgentActionCard approval={yieldManagementApproval} handleDecision={vi.fn()} queuedActionIds={new Set()} setEditingId={vi.fn()} editingId={null} setEditContent={vi.fn()} editContent="" editQuotePrice="" editQuoteScope="" setEditQuotePrice={vi.fn()} setEditQuoteScope={vi.fn()} />);

    expect(screen.getByText('Apply Dynamic Rule')).toBeInTheDocument();
    expect(screen.getByText('Peak Surge: Guitar Lesson')).toBeInTheDocument();
    expect(screen.getByTestId('approve-dynamic-pricing')).toBeInTheDocument();
    expect(screen.getByTestId('adjust-dynamic-pricing')).toBeInTheDocument();
  });
});
