import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { GroupedAgentActionCard } from './GroupedAgentActionCard';
import '@testing-library/jest-dom';

describe('GroupedAgentActionCard', () => {
  const mockItems = [
    {
      id: 'msg_1',
      event_source: 'Source 1',
      tenant_id: 't1',
      agent_id: 'a1',
      status: 'pending',
      created_at: new Date().toISOString(),
      proposed_action: null,
      context_payload: null
    },
    {
      id: 'msg_2',
      event_source: 'Source 1',
      tenant_id: 't1',
      agent_id: 'a1',
      status: 'pending',
      created_at: new Date().toISOString(),
      proposed_action: null,
      context_payload: null
    }
  ] as any[];

  const defaultGroup = {
    id: 'group_1',
    items: mockItems
  };

  it('renders standard layout without error', () => {
    render(<GroupedAgentActionCard
      items={defaultGroup.items} groupKey={defaultGroup.id} title="items"
      handleDecision={vi.fn()}
      loadingAction={null}
      queuedActionIds={new Set()}
      setEditingId={vi.fn()}
      editingId={null}
      setEditContent={vi.fn()}
      editContent=""
    />);
    expect(screen.getByText('2 items')).toBeInTheDocument();
    expect(screen.getByTestId('approve-all-group_1')).toBeInTheDocument();
  });

  it('handles approve all click', () => {
    const handleDecision = vi.fn();
    render(<GroupedAgentActionCard
      items={defaultGroup.items} groupKey={defaultGroup.id} title="items"
      handleDecision={handleDecision}
      loadingAction={null}
      queuedActionIds={new Set()}
      setEditingId={vi.fn()}
      editingId={null}
      setEditContent={vi.fn()}
      editContent=""
    />);

    fireEvent.click(screen.getByTestId('approve-all-group_1'));
    expect(handleDecision).toHaveBeenCalledWith('msg_1', true, undefined, 'Source 1');
    expect(handleDecision).toHaveBeenCalledWith('msg_2', true, undefined, 'Source 1');
  });

  it('handles expand/collapse', () => {
    render(<GroupedAgentActionCard
      items={defaultGroup.items} groupKey={defaultGroup.id} title="items"
      handleDecision={vi.fn()}
      loadingAction={null}
      queuedActionIds={new Set()}
      setEditingId={vi.fn()}
      editingId={null}
      setEditContent={vi.fn()}
      editContent=""
    />);

    // Initially not expanded
    expect(screen.queryByTestId('expanded-items-group_1')).not.toBeInTheDocument();

    // Click expand
    fireEvent.click(screen.getByText('Review Individually'));
    expect(screen.getByTestId('expanded-items-group_1')).toBeInTheDocument();

    // Click collapse
    fireEvent.click(screen.getByText('Collapse'));
    expect(screen.queryByTestId('expanded-items-group_1')).not.toBeInTheDocument();
  });

  it('renders invoice_draft properly in group', () => {
    const mockGroup = {
      id: 'inv_group',
      items: [
        {
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
        }
      ] as any[]
    };
    const wrapDecision = vi.fn();
    render(
      <GroupedAgentActionCard
        items={mockGroup.items} groupKey={mockGroup.id} title="items"
        handleDecision={vi.fn()}
        loadingAction={null}
        queuedActionIds={new Set()}
        setEditingId={vi.fn()}
        editingId={null}
        setEditContent={vi.fn()}
        editContent=""
        wrapDecision={wrapDecision}
        isActionLoading={() => false}
      />
    );

    fireEvent.click(screen.getByText('Review Individually'));
    expect(screen.getByText('Q3 Website Redesign')).toBeInTheDocument();
    expect(screen.getByText('Design Phase')).toBeInTheDocument();
    expect(screen.getByText('$1500.00')).toBeInTheDocument();
  });

  it('renders invoice_followup properly in group', () => {
    const mockGroup = {
      id: 'inv_followup_group',
      items: [
        {
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
        }
      ] as any[]
    };
    const wrapDecision = vi.fn();
    render(
      <GroupedAgentActionCard
        items={mockGroup.items} groupKey={mockGroup.id} title="items"
        handleDecision={vi.fn()}
        loadingAction={null}
        queuedActionIds={new Set()}
        setEditingId={vi.fn()}
        editingId={null}
        setEditContent={vi.fn()}
        editContent=""
        wrapDecision={wrapDecision}
        isActionLoading={() => false}
      />
    );

    fireEvent.click(screen.getByText('Review Individually'));
    expect(screen.getByText('Acme Corp invoice is 3 days overdue.')).toBeInTheDocument();
  });
});
