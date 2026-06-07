import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ReferralRedirectPage from './page';
import { useRouter, useParams, useSearchParams } from 'next/navigation';

vi.mock('next/navigation', () => ({
    useRouter: vi.fn(),
    useParams: vi.fn(),
    useSearchParams: vi.fn()
}));

describe('ReferralRedirectPage', () => {
    const mockPush = vi.fn();

    beforeEach(() => {
        vi.clearAllMocks();
        global.fetch = vi.fn().mockResolvedValue({ ok: true });

        Object.defineProperty(window, 'localStorage', {
            value: {
                setItem: vi.fn(),
                getItem: vi.fn()
            },
            writable: true
        });

        (useRouter as any).mockReturnValue({ push: mockPush });
        (useSearchParams as any).mockReturnValue({ get: (key: string) => key === 'offer' ? 'test_offer' : null });
    });

    it('redirects to root if no referrer id', async () => {
        (useParams as any).mockReturnValue({});

        render(<ReferralRedirectPage />);

        expect(mockPush).toHaveBeenCalledWith('/');
    });

    it('tracks click and redirects to onboarding if referrer id exists', async () => {
        (useParams as any).mockReturnValue({ referrer_id: 'user123' });

        render(<ReferralRedirectPage />);

        expect(window.localStorage.setItem).toHaveBeenCalledWith('referred_by', 'user123');
        expect(window.localStorage.setItem).toHaveBeenCalledWith('referral_offer', 'test_offer');

        expect(global.fetch).toHaveBeenCalledWith('/api/v1/growth/referrals/track', expect.objectContaining({
            method: 'POST',
            body: JSON.stringify({
                action: 'click',
                referrer_id: 'user123',
                offer: 'test_offer'
            })
        }));

        await waitFor(() => {
            expect(mockPush).toHaveBeenCalledWith('/onboarding');
        });
    });
});
