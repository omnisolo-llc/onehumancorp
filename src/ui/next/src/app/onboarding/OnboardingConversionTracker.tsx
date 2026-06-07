'use client';

import { useEffect } from 'react';

export function OnboardingConversionTracker() {
    useEffect(() => {
        const referredBy = localStorage.getItem('referred_by');
        const offer = localStorage.getItem('referral_offer');

        if (referredBy) {
            // Track conversion
            fetch('/api/v1/growth/referrals/track', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    action: 'conversion',
                    referrer_id: referredBy,
                    offer: offer || 'default'
                })
            }).catch(err => console.error('Failed to track conversion', err))
            .finally(() => {
                // Clear the state so we don't double track
                localStorage.removeItem('referred_by');
                localStorage.removeItem('referral_offer');
            });
        }
    }, []);

    return null;
}
