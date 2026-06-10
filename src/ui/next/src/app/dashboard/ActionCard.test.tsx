import { render, screen, fireEvent, act } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { ActionCard, AgentFeedItem } from './ActionCard';

describe('ActionCard', () => {
  const item: AgentFeedItem = {
    id: 'test-1',
    tenant_id: 'default',
    event_source: 'inventory_agent',
    lifecycle_state: 'PENDING_APPROVAL',
    proposed_action: { message: 'Restock needed' },
    context_payload: { product: 'Bread', quantity: 5 },
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
  };

  it('renders card title and agent type', () => {
    render(<ActionCard item={item} onApprove={vi.fn()} onEdit={vi.fn()} onDiscard={vi.fn()} />);
    expect(screen.getByText('Restock needed')).toBeDefined();
    expect(screen.getByText('Operations')).toBeDefined();
    expect(screen.getByText('📦')).toBeDefined();
  });

  it('calls onApprove when approve button is clicked', async () => {
    const onApprove = vi.fn().mockResolvedValue(undefined);
    render(<ActionCard item={item} onApprove={onApprove} onEdit={vi.fn()} onDiscard={vi.fn()} />);
    const btn = screen.getByLabelText('Approve');
    await act(async () => { fireEvent.click(btn); });
    expect(onApprove).toHaveBeenCalledWith('test-1');
  });

  it('expands context when Edit is clicked', async () => {
    render(<ActionCard item={item} onApprove={vi.fn()} onEdit={vi.fn()} onDiscard={vi.fn()} />);
    expect(screen.queryByText('Context & Details')).toBeNull();
    const editBtn = screen.getByLabelText('Edit');
    await act(async () => { fireEvent.click(editBtn); });
    expect(screen.getByText('Context & Details')).toBeDefined();
    expect(screen.getByText('Bread')).toBeDefined();
  });
});
