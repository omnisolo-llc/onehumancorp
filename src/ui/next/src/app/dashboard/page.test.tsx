import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import Dashboard from './page';
import { vi, describe, it, expect, beforeEach } from 'vitest';
import { TooltipProvider } from '../../components/TooltipRegistry';

// Mock the fetch call
global.fetch = vi.fn();

describe('Dashboard Component', () => {
    beforeEach(() => {
        vi.resetAllMocks();
        (global.fetch as any).mockResolvedValue({
            ok: true,
            json: async () => ({
                pending_approvals: [
                    {
                        id: 'test-approval-1',
                        description: 'Draft reply for Google Review from Maya',
                        payload: {
                            feature_type: 'google_review_reply',
                            original_message: 'Best vegan cakes ever!',
                            generated_response: 'Thanks Maya!',
                            star_rating: '5',
                            reviewer_name: 'Maya'
                        }
                    }
                ]
            }),
        });
    });

    it('renders the Local Visibility card with pending reviews', async () => {
        render(
            <TooltipProvider>
                <Dashboard />
            </TooltipProvider>
        );

        await waitFor(() => {
            expect(screen.getByText('Local Visibility')).toBeInTheDocument();
            expect(screen.getByText('Google Business Profile')).toBeInTheDocument();
            expect(screen.getByText('1 New')).toBeInTheDocument();
            expect(screen.getByText('Maya')).toBeInTheDocument();
            expect(screen.getByText('"Best vegan cakes ever!"')).toBeInTheDocument();
            expect(screen.getByText('Thanks Maya!')).toBeInTheDocument();
        });
    });

    it('approves a Google Review reply', async () => {
        render(
            <TooltipProvider>
                <Dashboard />
            </TooltipProvider>
        );

        await waitFor(() => {
            expect(screen.getByText('Approve & Reply')).toBeInTheDocument();
        });

        fireEvent.click(screen.getByText('Approve & Reply'));

        await waitFor(() => {
            expect(global.fetch).toHaveBeenCalledWith('/api/agents/approvals/test-approval-1', expect.objectContaining({
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ approved: true })
            }));
        });
    });
});
