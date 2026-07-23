/** @jsxImportSource react */
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { AmbassadorReplyCard } from './AmbassadorReplyCard';
import React from 'react';
import '@testing-library/jest-dom/vitest';

describe('AmbassadorReplyCard', () => {
  const defaultApproval = {
    id: 'item-123',
    lifecycle_state: 'PENDING_APPROVAL',
    payload: {
      source: 'instagram_dm',
      original_message: 'Do you have vegan option?',
      generated_response: 'Yes we do!',
      past_orders: '3 past orders',
      context_used: 'Customer requested vegan cake before'
    }
  };

  it('renders correctly with default props and customer context', () => {
    const onApprove = vi.fn();
    const onDismiss = vi.fn();
    const onEdit = vi.fn();

    render(
      <AmbassadorReplyCard
        approval={defaultApproval}
        onApprove={onApprove}
        onDismiss={onDismiss}
        onEdit={onEdit}
      />
    );

    // Verify header and source mapping
    expect(screen.getByText('1 New Message from instagram dm')).toBeInTheDocument();

    // Verify context section
    expect(screen.getByText('Customer Context')).toBeInTheDocument();
    expect(screen.getByText('3 past orders')).toBeInTheDocument();
    expect(screen.getByText('Customer requested vegan cake before')).toBeInTheDocument();

    // Verify inquiry and generated response
    expect(screen.getByText('"Do you have vegan option?"')).toBeInTheDocument();
    expect(screen.getByText('Yes we do!')).toBeInTheDocument();

    // Verify buttons are rendered
    const approveBtn = screen.getByTestId('feed-approve-btn');
    const editBtn = screen.getByTestId('feed-edit-btn');
    const dismissBtn = screen.getByTestId('feed-dismiss-btn');

    expect(approveBtn).toBeInTheDocument();
    expect(editBtn).toBeInTheDocument();
    expect(dismissBtn).toBeInTheDocument();

    // Simulate interactions
    fireEvent.click(approveBtn);
    expect(onApprove).toHaveBeenCalledTimes(1);

    fireEvent.click(editBtn);
    expect(onEdit).toHaveBeenCalledTimes(1);

    fireEvent.click(dismissBtn);
    expect(onDismiss).toHaveBeenCalledTimes(1);
  });

  it('renders properly when editing is active', () => {
    const onSaveEdit = vi.fn();
    const onCancelEdit = vi.fn();
    const setEditContent = vi.fn();

    render(
      <AmbassadorReplyCard
        approval={defaultApproval}
        isEditing={true}
        editContent="Updated reply content!"
        onSaveEdit={onSaveEdit}
        onCancelEdit={onCancelEdit}
        setEditContent={setEditContent}
      />
    );

    // Verify the textarea is in edit mode and shows editContent
    const textarea = screen.getByTestId('feed-edit-input') as HTMLTextAreaElement;
    expect(textarea).toBeInTheDocument();
    expect(textarea.value).toBe('Updated reply content!');

    // Simulate typing
    fireEvent.change(textarea, { target: { value: 'Something else' } });
    expect(setEditContent).toHaveBeenCalledWith('Something else');

    // Verify save and cancel buttons are present
    const saveBtn = screen.getByTestId('feed-save-edit-btn');
    const cancelBtn = screen.getByTestId('feed-cancel-edit-btn');

    expect(saveBtn).toBeInTheDocument();
    expect(cancelBtn).toBeInTheDocument();

    fireEvent.click(saveBtn);
    expect(onSaveEdit).toHaveBeenCalledTimes(1);

    fireEvent.click(cancelBtn);
    expect(onCancelEdit).toHaveBeenCalledTimes(1);
  });

  it('handles empty/fallback fields gracefully', () => {
    const minimalApproval = {
      id: 'item-minimal',
      proposed_action: {
        source: undefined,
        original_message: undefined,
        generated_response: undefined
      }
    };

    render(<AmbassadorReplyCard approval={minimalApproval} />);

    expect(screen.getByText('1 New Message from unknown')).toBeInTheDocument();
    expect(screen.getByText('"Customer message"')).toBeInTheDocument();
    expect(screen.getByText('Ready to send.')).toBeInTheDocument();
  });
});
