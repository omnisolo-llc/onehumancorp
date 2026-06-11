import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import IncidentRoomPage from './page';

describe('IncidentRoomPage', () => {
    beforeEach(() => {
        vi.clearAllMocks();
        global.fetch = vi.fn();
    });

    it('renders and fetches incidents', async () => {
        (global.fetch as any).mockResolvedValueOnce({
            ok: true,
            json: async () => [
                {
                    id: 'inc-1',
                    title: 'Espresso machine down',
                    description: 'No espresso available',
                    status: 'proposed',
                    resolution_plan: { summary: 'Plan 1', actions: [] },
                    created_at: '2023-01-01T00:00:00Z',
                }
            ],
        });

        render(<IncidentRoomPage />);
        expect(screen.getByText('Loading incidents...')).toBeInTheDocument();

        await waitFor(() => {
            expect(screen.getByText('Espresso machine down')).toBeInTheDocument();
        });
    });

    it('handles approval flow', async () => {
        (global.fetch as any)
            .mockResolvedValueOnce({
                ok: true,
                json: async () => [
                    {
                        id: 'inc-1',
                        title: 'Espresso machine down',
                        description: 'No espresso available',
                        status: 'proposed',
                        resolution_plan: { summary: 'Plan 1', actions: [] },
                        created_at: '2023-01-01T00:00:00Z',
                    }
                ],
            })
            .mockResolvedValueOnce({
                ok: true,
                json: async () => ({ status: 'approved_and_executed' }),
            })
            .mockResolvedValueOnce({
                ok: true,
                json: async () => [], // Empty after approval
            });

        render(<IncidentRoomPage />);

        await waitFor(() => {
            expect(screen.getByText('Espresso machine down')).toBeInTheDocument();
        });

        fireEvent.click(screen.getByTestId('incident-card-inc-1'));

        await waitFor(() => {
            expect(screen.getByText('Resolution Plan')).toBeInTheDocument();
        });

        fireEvent.click(screen.getByTestId('execute-plan-btn'));

        await waitFor(() => {
            expect(screen.getByText('Plan executed successfully.')).toBeInTheDocument();
        });
    });

    it('renders urgent priority correctly', async () => {
         (global.fetch as any).mockResolvedValueOnce({
            ok: true,
            json: async () => [
                {
                    id: 'inc-1',
                    title: 'Urgent task',
                    description: 'No espresso available',
                    status: 'proposed',
                    resolution_plan: { summary: 'Plan 1', actions: [] },
                    created_at: '2023-01-01T00:00:00Z',
                }
            ],
        });
        render(<IncidentRoomPage />);
        await waitFor(() => {
            expect(screen.getAllByText('Urgent')[1]).toBeInTheDocument(); // first is the title
        });
    });

    it('handles cancel properly', async () => {
        (global.fetch as any).mockResolvedValueOnce({
            ok: true,
            json: async () => [
                {
                    id: 'inc-1',
                    title: 'Espresso machine down',
                    description: 'No espresso available',
                    status: 'proposed',
                    resolution_plan: { summary: 'Plan 1', actions: [] },
                    created_at: '2023-01-01T00:00:00Z',
                }
            ],
        });
        render(<IncidentRoomPage />);
        await waitFor(() => {
            expect(screen.getByText('Espresso machine down')).toBeInTheDocument();
        });
        fireEvent.click(screen.getByTestId('incident-card-inc-1'));
        await waitFor(() => {
            expect(screen.getByText('Resolution Plan')).toBeInTheDocument();
        });
        fireEvent.click(screen.getByText('Cancel'));
        await waitFor(() => {
            expect(screen.queryByText('Resolution Plan')).not.toBeInTheDocument();
        });
    });
});
