/** @vitest-environment jsdom */
import React from 'react';
import { render, screen, fireEvent, cleanup } from '@testing-library/react';
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

    it('opens edit mode, allows editing, and calls onEdit with draft_id and edited response when Save button is clicked', () => {
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

        // Edit text area should be visible
        const textarea = screen.getByTestId('feed-edit-textarea') as HTMLTextAreaElement;
        expect(textarea).toBeDefined();
        expect(textarea.value).toBe(mockDraft.response);

        // Change response and save
        fireEvent.change(textarea, { target: { value: 'New edited response!' } });
        const saveButton = screen.getByTestId('feed-save-btn');
        fireEvent.click(saveButton);

        expect(mockEdit).toHaveBeenCalledWith('draft-123', 'New edited response!');
        expect(mockEdit).toHaveBeenCalledTimes(1);
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
