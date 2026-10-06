import React from 'react';
import { render, screen, fireEvent, act, waitFor } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { AgentActionCard, type AgentActionCardProps } from './AgentActionCard';
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

  const defaultProps = {
    approval: defaultApproval,
    queuedActionIds: new Set<string>(),
    editingId: null,
    editContent: '',
    editQuotePrice: '',
    editQuoteScope: '',
    setEditingId: vi.fn(),
    setEditContent: vi.fn(),
    setEditQuotePrice: vi.fn(),
    setEditQuoteScope: vi.fn(),
    handleDecision: vi.fn(),
  };

  function renderEditingCard(handleDecision: AgentActionCardProps['handleDecision']) {
    function EditingCard() {
      const [editingId, setEditingId] = React.useState<string | null>('msg_1');
      const [editContent, setEditContent] = React.useState('Edited proposal');
      return <AgentActionCard {...defaultProps} {...{ editingId, setEditingId, editContent, setEditContent, handleDecision }} />;
    }
    return render(<EditingCard />);
  }

  it.each(['draft_reply', 'generated_response'] as const)('shows a generic %s before editing and initializes the editor with it', (field) => {
    const setEditingId = vi.fn();
    const setEditContent = vi.fn();
    const approval = {
      ...defaultApproval,
      event_source: 'sales',
      context_payload: { description: 'Customer inquiry' },
      proposed_action: { [field]: 'Proposed response\nSecond paragraph' },
    };
    render(<AgentActionCard {...defaultProps} {...{ approval, setEditingId, setEditContent }} />);

    expect(screen.getByRole('heading', { name: 'Customer inquiry' })).toBeVisible();
    expect(screen.getByText('Proposed response Second paragraph')).toBeVisible();
    expect(screen.queryByRole('textbox')).not.toBeInTheDocument();
    fireEvent.click(screen.getByTestId('edit-proposal'));
    expect(setEditingId).toHaveBeenCalledWith('msg_1');
    expect(setEditContent).toHaveBeenCalledWith('Proposed response\nSecond paragraph');
  });

  it('preserves the specialized social draft without adding a duplicate generic draft', () => {
    const approval = {
      ...defaultApproval,
      proposed_action: { feature_type: 'social_post_draft', instagram: 'Specialized social draft', draft_reply: 'Generic fallback' },
    };
    render(<AgentActionCard {...defaultProps} approval={approval} />);

    expect(screen.getByText(/Specialized social draft/)).toBeVisible();
    expect(screen.queryByText('Generic fallback')).not.toBeInTheDocument();
  });

  it.each(['Additional customer context', { description: 'Additional customer context' }])('shows a generic draft alongside proposed-action context %j', (context) => {
    const approval = {
      ...defaultApproval,
      event_source: 'sales',
      context_payload: { description: 'Customer inquiry' },
      proposed_action: { context, draft_reply: 'Reply with context' },
    };
    render(<AgentActionCard {...defaultProps} approval={approval} />);

    expect(screen.getByRole('heading', { name: 'Customer inquiry' })).toBeVisible();
    expect(screen.getByText('Reply with context')).toBeVisible();
  });

  it('shows the generic draft for an unrecognized feature with context', () => {
    const approval = {
      ...defaultApproval,
      proposed_action: { feature_type: 'lead_reply', context: 'Inquiry context', draft_reply: 'Reply for a new feature' },
    };
    render(<AgentActionCard {...defaultProps} approval={approval} />);

    expect(screen.getByText('Reply for a new feature')).toBeVisible();
  });

  it.each([42, { invalid: true }, ['Invalid draft'], '   '])('uses draft_reply when generated_response is not usable text: %j', (generated_response) => {
    const setEditContent = vi.fn();
    const approval = {
      ...defaultApproval,
      proposed_action: { generated_response, draft_reply: 'Usable fallback reply' } as unknown as AgentActionCardProps['approval']['proposed_action'],
    };
    render(<AgentActionCard {...defaultProps} {...{ approval, setEditContent }} />);

    expect(screen.getByText('Usable fallback reply')).toBeVisible();
    fireEvent.click(screen.getByTestId('edit-proposal'));
    expect(setEditContent).toHaveBeenCalledWith('Usable fallback reply');
  });

  it.each(['CustomerSuccessAgent', 'customer_success_agent'])('names the %s dismissal accurately and preserves its callback', async (event_source) => {
    const handleDecision = vi.fn().mockResolvedValue(true);
    const approval = {
      ...defaultApproval,
      event_source,
      context_payload: { description: 'Customer needs help' },
      proposed_action: { draft_reply: 'Proposed support reply' },
    };
    render(<AgentActionCard {...defaultProps} {...{ approval, handleDecision }} />);

    expect(screen.getByText('Proposed support reply')).toBeVisible();
    expect(screen.queryByRole('button', { name: /Edit/ })).not.toBeInTheDocument();
    await act(async () => fireEvent.click(screen.getByRole('button', { name: 'Dismiss' })));
    expect(handleDecision).toHaveBeenCalledExactlyOnceWith('msg_1', false, undefined, event_source);
    expect(screen.getByTestId('triage-card-msg_1')).not.toHaveClass('scale-95');
  });

  it('shows approval styling only after the decision is acknowledged', async () => {
    let acknowledge!: (accepted: boolean) => void;
    const handleDecision = vi.fn(() => new Promise<boolean>((resolve) => { acknowledge = resolve; }));
    render(<AgentActionCard {...defaultProps} handleDecision={handleDecision} />);

    fireEvent.click(screen.getByTestId('feed-approve-btn'));
    expect(handleDecision).toHaveBeenCalledExactlyOnceWith('msg_1', true, undefined, 'Test event');
    expect(screen.getByTestId('feed-approve-btn')).toBeDisabled();
    expect(screen.getByTestId('triage-card-msg_1')).not.toHaveClass('scale-95');

    await act(async () => acknowledge(true));
    expect(screen.getByTestId('triage-card-msg_1')).toHaveClass('scale-95', '!border-green-500');
  });

  describe.each(['save-proposal', 'feed-approve-btn'])('%s editing submission', (buttonId) => {
    it.each([false, undefined])('retains the draft and editor when acknowledgment is %s', async (accepted) => {
      let acknowledge!: (accepted: boolean | undefined) => void;
      const handleDecision = vi.fn(() => new Promise<boolean | undefined>((resolve) => { acknowledge = resolve; }));
      renderEditingCard(handleDecision);

      fireEvent.click(screen.getByTestId(buttonId));
      expect(handleDecision).toHaveBeenCalledExactlyOnceWith('msg_1', true, 'Edited proposal', 'Test event');
      expect(screen.getByRole('textbox')).toHaveValue('Edited proposal');
      expect(screen.getByTestId('triage-card-msg_1')).not.toHaveClass('scale-95');

      await act(async () => acknowledge(accepted));
      expect(screen.getByRole('textbox')).toHaveValue('Edited proposal');
      expect(screen.getByTestId(buttonId)).toBeEnabled();
      expect(screen.getByTestId('triage-card-msg_1')).not.toHaveClass('scale-95');
    });

    it('closes the editor after an acknowledged edited decision', async () => {
      const handleDecision = vi.fn().mockResolvedValue(true);
      renderEditingCard(handleDecision);

      await act(async () => fireEvent.click(screen.getByTestId(buttonId)));
      expect(handleDecision).toHaveBeenCalledExactlyOnceWith('msg_1', true, 'Edited proposal', 'Test event');
      expect(screen.queryByRole('textbox')).not.toBeInTheDocument();
      expect(screen.getByTestId('triage-card-msg_1')).toHaveClass('scale-95');
    });
  });

  it('retains the edited draft after the decision callback throws', async () => {
    const error = new Error('Acknowledgment unavailable');
    const handleDecision = vi.fn().mockRejectedValue(error);
    const consoleError = vi.spyOn(console, 'error').mockImplementation(() => {});
    try {
      renderEditingCard(handleDecision);
      fireEvent.click(screen.getByTestId('save-proposal'));
      await waitFor(() => expect(screen.getByTestId('save-proposal')).toBeEnabled());

      expect(screen.getByRole('textbox')).toHaveValue('Edited proposal');
      expect(screen.getByTestId('triage-card-msg_1')).not.toHaveClass('scale-95');
      expect(consoleError).toHaveBeenCalledWith('Decision failed', error);
    } finally {
      consoleError.mockRestore();
    }
  });

  it('renders standard layout without error', () => {
    render(<AgentActionCard approval={defaultApproval} handleDecision={vi.fn()} queuedActionIds={new Set()} setEditingId={vi.fn()} editingId={null} setEditContent={vi.fn()} editContent="" editQuotePrice="" editQuoteScope="" setEditQuotePrice={vi.fn()} setEditQuoteScope={vi.fn()} />);
    expect(screen.getAllByText('Test event')[0]).toBeInTheDocument();
    expect(screen.getByTestId('feed-approve-btn')).toBeInTheDocument();
    expect(screen.getByTestId('feed-dismiss-btn')).toHaveClass('bg-red-100', 'text-red-700');
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
  it('renders the daily prep task controls once instead of unreachable generic actions', () => {
    const approval = { ...defaultApproval, event_source: 'Operations Agent', context_payload: { feature_type: 'daily_prep_checklist', description: 'Daily Prep Checklist' }, proposed_action: { action_type: 'Daily Prep Checklist', message: 'Review Daily Prep Checklist' } };
    render(<AgentActionCard approval={approval} handleDecision={vi.fn()} queuedActionIds={new Set()} setEditingId={vi.fn()} editingId={null} setEditContent={vi.fn()} editContent="" editQuotePrice="" editQuoteScope="" setEditQuotePrice={vi.fn()} setEditQuoteScope={vi.fn()} />);
    expect(screen.getByRole('button', { name: 'Mark Complete' })).toBeVisible();
    expect(screen.getByRole('button', { name: 'Assign to Staff' })).toBeVisible();
    expect(screen.getByRole('button', { name: 'Dismiss task' })).toBeVisible();
    expect(screen.getAllByTestId('feed-approve-btn')).toHaveLength(1);
    expect(screen.getAllByTestId('feed-dismiss-btn')).toHaveLength(1);
  });

});
