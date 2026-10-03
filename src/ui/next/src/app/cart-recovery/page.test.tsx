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
const fetchPlan = vi.fn<typeof fetch>(async url => Response.json(url === '/api/v1/auth/session-identity' ? { userId: 'cart-owner', tenantId: 'cart-tenant', expiresAt: Date.now() + 60000 } : { current_plan: 'Pro' }));

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
        expect(fetchPlan.mock.calls.map(([url]) => url)).toEqual(['/api/v1/auth/session-identity', '/api/v1/billing/my-plan']);
        expect(fetchPlan).toHaveBeenCalledWith('/api/v1/billing/my-plan', expect.objectContaining({ credentials: 'same-origin' }));
        expect(fetchPlan.mock.calls.some(([, options]) => options?.method === 'POST')).toBe(false);
    });
});
