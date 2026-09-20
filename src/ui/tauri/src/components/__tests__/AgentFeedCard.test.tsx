/** @vitest-environment jsdom */
import React from 'react';
import { render, screen, fireEvent, cleanup, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, afterEach } from 'vitest';
import { AgentFeedCard, ActionRequiredDraft } from '../AgentFeedCard';

describe('AgentFeedCard', () => {
    afterEach(() => {
        cleanup();
    });

    const mockDraft: ActionRequiredDraft = {
        draft_id: 'draft-123',
        work_item_id: 'work-456',
        tenant_id: 'tenant-789',
        customer_id: 'cust-101',
        customer_name: 'Alice Smith',
        source: 'Instagram',
        response: 'Hello Alice! We can deliver the cake on Friday.',
        status: 'PENDING_APPROVAL'
    };

    it('renders customer name and source correctly', () => {
        const mockApprove = vi.fn();
        const mockEdit = vi.fn();

        render(
            <AgentFeedCard
                draft={mockDraft}
                onApprove={mockApprove}
                onEdit={mockEdit}
            />
        );

        expect(screen.getByText('Message from Alice Smith')).toBeDefined();
        expect(screen.getByText('Instagram')).toBeDefined();
        expect(screen.getByText('Hello Alice! We can deliver the cake on Friday.')).toBeDefined();
    });

    it('calls onApprove with draft_id when Approve button is clicked', () => {
        const mockApprove = vi.fn();
        const mockEdit = vi.fn();

        render(
            <AgentFeedCard
                draft={mockDraft}
                onApprove={mockApprove}
                onEdit={mockEdit}
            />
        );

        const approveButton = screen.getByRole('button', { name: 'Approve & Send' });
        fireEvent.click(approveButton);

        expect(mockApprove).toHaveBeenCalledWith('draft-123');
        expect(mockApprove).toHaveBeenCalledTimes(1);
    });

    it('saves the edited response with the draft identity only after Save', async () => {
        const mockApprove = vi.fn();
        const mockEdit = vi.fn();

        render(
            <AgentFeedCard
                draft={mockDraft}
                onApprove={mockApprove}
                onEdit={mockEdit}
            />
        );

        const editButton = screen.getByRole('button', { name: 'Edit Draft' });
        fireEvent.click(editButton);

        expect(mockEdit).not.toHaveBeenCalled();
        const editor = screen.getByRole('textbox', { name: 'Draft response' });
        fireEvent.change(editor, { target: { value: 'The owner-approved revised response.' } });
        fireEvent.click(screen.getByRole('button', { name: 'Save' }));
        await waitFor(() => {
            expect(mockEdit).toHaveBeenCalledWith('draft-123', 'The owner-approved revised response.');
            expect(mockEdit).toHaveBeenCalledTimes(1);
            expect(screen.queryByRole('textbox')).toBeNull();
        });
    });

    it('keeps unsaved text visible when persistence fails and never approves it', async () => {
        const onEdit = vi.fn().mockRejectedValue(new Error('Unavailable'));
        const onApprove = vi.fn();
        render(<AgentFeedCard draft={mockDraft} onApprove={onApprove} onEdit={onEdit} />);
        fireEvent.click(screen.getByRole('button', { name: 'Edit Draft' }));
        fireEvent.change(screen.getByRole('textbox', { name: 'Draft response' }), {
            target: { value: 'Keep this unsaved response.' },
        });
        fireEvent.click(screen.getByRole('button', { name: 'Save' }));
        await waitFor(() => expect(screen.getByRole('alert').textContent).toContain('not saved'));
        expect((screen.getByRole('textbox') as HTMLTextAreaElement).value).toBe('Keep this unsaved response.');
        expect(onApprove).not.toHaveBeenCalled();
        expect(onEdit).toHaveBeenCalledTimes(1);
        fireEvent.click(screen.getByRole('button', { name: 'Cancel' }));
        expect(screen.queryByRole('textbox')).toBeNull();
        expect(screen.getByText(mockDraft.response)).toBeDefined();
    });

    it('renders "Unknown User" if customer_name is not provided', () => {
        const draftWithoutName = { ...mockDraft, customer_name: undefined };

        render(
            <AgentFeedCard
                draft={draftWithoutName}
                onApprove={vi.fn()}
                onEdit={vi.fn()}
            />
        );

        expect(screen.getByText('Message from Unknown User')).toBeDefined();
    });
});
