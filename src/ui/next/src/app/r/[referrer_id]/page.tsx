'use client';

import { useEffect, useState } from 'react';
import { useRouter, useParams, useSearchParams } from 'next/navigation';

export default function ReferralRedirectPage() {
    const router = useRouter();
    const params = useParams();
    const searchParams = useSearchParams();
    const [tracking, setTracking] = useState(true);

    useEffect(() => {
        const referrerId = params.referrer_id as string;
        const offer = searchParams.get('offer') || 'default';

        if (!referrerId) {
            router.push('/');
            return;
        }

        // Store the referral info in localStorage for conversion tracking later
        if (typeof window !== 'undefined') {
            localStorage.setItem('referred_by', referrerId);
            localStorage.setItem('referral_offer', offer);
        }

        // Track the click
        fetch('/api/v1/growth/referrals/track', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                action: 'click',
                referrer_id: referrerId,
                offer: offer,
            })
        }).catch(err => console.error('Tracking failed', err))
          .finally(() => {
              // Redirect to onboarding or sign up
              setTracking(false);
              router.push('/onboarding');
          });

    }, [params, searchParams, router]);

    return (
        <div className="min-h-screen flex items-center justify-center bg-gray-50 font-outfit">
            <div className="text-center">
                <div className="w-12 h-12 border-4 border-indigo-200 border-t-indigo-600 rounded-full animate-spin mx-auto mb-4"></div>
                <p className="text-gray-600 text-sm font-medium">Applying your special offer...</p>
            </div>
        </div>
    );
}
