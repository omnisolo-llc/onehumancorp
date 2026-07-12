/** @vitest-environment jsdom */
import React from 'react';
import { render, screen, waitFor, fireEvent, cleanup } from '@testing-library/react';
import { describe, it, expect, vi, afterEach, beforeEach } from 'vitest';
import { AgentFeed } from '../AgentFeed';

// Mock the child component
vi.mock('../AgentFeedCard', () => {
    return {
        AgentFeedCard: ({ draft, onApprove, onEdit }: any) => (
            <div data-testid={`feed-card-${draft.draft_id}`}>
                <div data-testid={`draft-name-${draft.draft_id}`}>{draft.customer_name}</div>
                <button onClick={() => onApprove(draft.draft_id)}>Approve {draft.draft_id}</button>
                <button onClick={() => onEdit(draft.draft_id)}>Edit {draft.draft_id}</button>
            </div>
        )
    };
});

describe('AgentFeed', () => {
    const mockDrafts = [
        {
            draft_id: 'draft-1',
            work_item_id: 'work-1',
            tenant_id: 'tenant-1',
            customer_id: 'cust-1',
            customer_name: 'Bob',
            source: 'Email',
            response: 'Hi Bob',
            status: 'PENDING_APPROVAL'
        },
        {
            draft_id: 'draft-2',
            work_item_id: 'work-2',
            tenant_id: 'tenant-1',
            customer_id: 'cust-2',
            customer_name: 'Alice',
            source: 'SMS',
            response: 'Hi Alice',
            status: 'PENDING_APPROVAL'
        }
    ];

    beforeEach(() => {
        // Reset fetch mock
        global.fetch = vi.fn();
    });

    afterEach(() => {
        cleanup();
        vi.resetAllMocks();
    });

    it('displays loading state initially', () => {
        (global.fetch as any).mockImplementation(() => new Promise(() => {})); // Never resolves

        render(<AgentFeed />);
        expect(document.querySelector('.animate-pulse')).toBeDefined();
    });

    it('displays error state if fetch fails', async () => {
        (global.fetch as any).mockRejectedValue(new Error('Network error'));

        render(<AgentFeed />);

        await waitFor(() => {
            expect(screen.getByText('Network error')).toBeDefined();
        });
    });

    it('displays "No pending actions!" if feed is empty', async () => {
        (global.fetch as any).mockResolvedValue({
            ok: true,
            json: async () => []
        });

        render(<AgentFeed />);

        await waitFor(() => {
            expect(screen.getByText('All caught up!')).toBeDefined();
            expect(screen.getByText('No pending actions right now.')).toBeDefined();
        });
    });

    it('renders list of drafts', async () => {
        (global.fetch as any).mockResolvedValue({
            ok: true,
            json: async () => mockDrafts
        });

        render(<AgentFeed />);

        await waitFor(() => {
            expect(screen.getByTestId('feed-card-draft-1')).toBeDefined();
            expect(screen.getByTestId('feed-card-draft-2')).toBeDefined();
        });
    });

    it('removes draft from UI optimistically on successful approve', async () => {
        // Mock initial fetch
        (global.fetch as any).mockResolvedValueOnce({
            ok: true,
            json: async () => mockDrafts
        });

        render(<AgentFeed />);

        // Wait for render
        await waitFor(() => {
            expect(screen.getByTestId('feed-card-draft-1')).toBeDefined();
            expect(screen.getByTestId('feed-card-draft-2')).toBeDefined();
        });

        // Mock approve fetch
        (global.fetch as any).mockResolvedValueOnce({
            ok: true
        });

        const approveButton = screen.getByText('Approve draft-1');
        fireEvent.click(approveButton);

        // Verify API was called correctly
        expect(global.fetch).toHaveBeenCalledWith('/api/inbox/action_required/draft-1/approve', {
            method: 'POST'
        });

        // Verify optimistic update
        await waitFor(() => {
            expect(screen.queryByTestId('feed-card-draft-1')).toBeNull();
            expect(screen.getByTestId('feed-card-draft-2')).toBeDefined(); // Still exists
        });
    });

    it('does not remove draft if approve fails', async () => {
        // Mock initial fetch
        (global.fetch as any).mockResolvedValueOnce({
            ok: true,
            json: async () => mockDrafts
        });

        render(<AgentFeed />);

        // Wait for render
        await waitFor(() => {
            expect(screen.getByTestId('feed-card-draft-1')).toBeDefined();
        });

        // Mock approve fetch failing
        (global.fetch as any).mockResolvedValueOnce({
            ok: false
        });

        // Mock console.error
        const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

        const approveButton = screen.getByText('Approve draft-1');
        fireEvent.click(approveButton);

        await waitFor(() => {
            expect(consoleSpy).toHaveBeenCalledWith("Failed to approve draft");
        });

        // Draft should still be in UI
        expect(screen.getByTestId('feed-card-draft-1')).toBeDefined();

        consoleSpy.mockRestore();
    });
});
