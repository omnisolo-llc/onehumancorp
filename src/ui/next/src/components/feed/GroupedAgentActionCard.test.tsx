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
      agent_id: 'agent_1',
      status: 'pending',
      created_at: new Date().toISOString(),
      proposed_action: null,
      context_payload: null
    },
    {
      id: 'msg_2',
      event_source: 'Source 2',
      tenant_id: 't1',
      agent_id: 'agent_1',
      status: 'pending',
      created_at: new Date().toISOString(),
      proposed_action: null,
      context_payload: null
    }
  ];

  it('renders standard layout without error', () => {
    render(<GroupedAgentActionCard
      groupKey="group_1"
      title="Tasks"
      items={mockItems}
      handleDecision={vi.fn()}
      loadingAction={null}
      queuedActionIds={new Set()}
      setEditingId={vi.fn()}
      editingId={null}
      setEditContent={vi.fn()}
      editContent=""
    />);
    expect(screen.getByText('2 new Tasks')).toBeInTheDocument();
    expect(screen.getByText('2 items')).toBeInTheDocument();

    // Check if the bulk action buttons are present
    expect(screen.getByText('Approve All')).toBeInTheDocument();
  });

  it('handles approve all click', () => {
    const handleDecision = vi.fn();
    render(<GroupedAgentActionCard
      groupKey="group_1"
      title="Tasks"
      items={mockItems}
      handleDecision={handleDecision}
      loadingAction={null}
      queuedActionIds={new Set()}
      setEditingId={vi.fn()}
      editingId={null}
      setEditContent={vi.fn()}
      editContent=""
    />);

    fireEvent.click(screen.getByText('Approve All'));
    // It should call handleDecision for each item
    expect(handleDecision).toHaveBeenCalledTimes(2);
    expect(handleDecision).toHaveBeenCalledWith('msg_1', true, undefined, 'Source 1');
    expect(handleDecision).toHaveBeenCalledWith('msg_2', true, undefined, 'Source 2');
  });

  it('handles expand/collapse', () => {
    render(<GroupedAgentActionCard
      groupKey="group_1"
      title="Tasks"
      items={mockItems}
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
});
