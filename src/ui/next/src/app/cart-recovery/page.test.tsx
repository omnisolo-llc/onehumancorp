import React from 'react';
import { act, render, screen, fireEvent } from '@testing-library/react';
import CartRecoveryPage from './page';
import { vi, describe, it, expect, beforeEach } from 'vitest';

// Mock Next.js router
vi.mock('next/navigation', () => ({
    useRouter: () => ({
        push: vi.fn(),
    }),
}));

// A boundary fixture, not a claimed live billing connection.
const fetchPlan = vi.fn<typeof fetch>(async () => Response.json({ current_plan: 'pro' }));

describe('CartRecoveryPage', () => {
    beforeEach(() => {
        vi.clearAllMocks();
        vi.stubGlobal('fetch', fetchPlan);
        localStorage.clear();
        localStorage.setItem('has_pro', 'true');
    });

    it('renders the Cart Recovery page correctly', async () => {
        await act(async () => { render(<CartRecoveryPage />); });
        expect(screen.getByText('Recover Abandoned Carts')).toBeInTheDocument();
        expect(screen.getByText('Generate AI Campaign')).toBeInTheDocument();
    });

    it('does not pretend automatic recovery works without a dispatcher', async () => {
        await act(async () => { render(<CartRecoveryPage />); });
        const toggle = screen.getByRole('button', { name: 'Auto recovery unavailable' });
        expect(toggle).toBeDisabled();
        expect(screen.getByText('Automatic recovery is unavailable until a real campaign dispatcher is connected.')).toBeVisible();
        fireEvent.click(toggle);
        expect(toggle).toBeDisabled();
        expect(fetchPlan).toHaveBeenCalledTimes(1);
        expect(fetchPlan).toHaveBeenCalledWith('/api/v1/billing/my-plan');
    });
});
