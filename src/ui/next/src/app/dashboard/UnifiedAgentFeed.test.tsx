import React from 'react';
import { render, screen, waitFor, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { UnifiedAgentFeed } from './UnifiedAgentFeed';

// Mock fetch globally
const mockFetch = vi.fn();
global.fetch = mockFetch;

describe('UnifiedAgentFeed Component', () => {
    beforeEach(() => {
        mockFetch.mockReset();
        // The component makes 2 parallel fetch calls
        mockFetch.mockImplementation(async (url) => {
            if (url.includes('/api/agents/approvals/activity')) {
                return {
                    ok: true,
                    json: async () => ({ pending_approvals: [] })
                };
            }
            if (url.includes('/api/agents/approvals')) {
                return {
                    ok: true,
                    json: async () => ({
                        pending_approvals: [
                            { id: '1', title: 'Test Proposal', description: 'This is a test proposal.', status: 'pending', department: 'Sales', payload: {} }
                        ]
                    })
                };
            }
            return { ok: true, json: async () => ({ pending_approvals: [] }) };
        });
    });

    it('renders the feed header', async () => {
        await act(async () => {
            render(<UnifiedAgentFeed />);
        });
        expect(screen.getAllByText(/Proposals/i)[0]).toBeInTheDocument();
        expect(screen.getByText(/Activity Feed/i)).toBeInTheDocument();
    });

    it('verifies the Approve button has a minimum height of 44px', async () => {
        await act(async () => {
            render(<UnifiedAgentFeed />);
        });

        // Wait for the approvals to load
        await waitFor(() => {
            expect(screen.getByText(/This is a test proposal./i)).toBeInTheDocument();
        });

        const buttons = await screen.findAllByRole('button', { name: /Approve/i });
        expect(buttons.length).toBeGreaterThan(0);

        // Check if the button has the Tailwind class for min-height 44px
        const approveButton = buttons[0];
        expect(approveButton).toHaveClass('min-h-[44px]');
    });
});
