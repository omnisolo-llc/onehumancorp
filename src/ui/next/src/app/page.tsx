'use client';
import { useEffect, Suspense } from 'react';
import { useRouter, useSearchParams } from 'next/navigation';

function HomeContent() {
  const router = useRouter();
  const searchParams = useSearchParams();

  useEffect(() => {
    if (searchParams.get('dashboard') === '1') {
      router.push('/dashboard');
      return;
    }

    const hasOnboarded = localStorage.getItem('has_onboarded');

    const checkOnboardingStatus = async () => {
      try {
        const tenantId = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront';
        const userId = localStorage.getItem('user_id') || 'test-user';

        const res = await fetch('/api/onboarding/status', {
          headers: {
            'X-Tenant-ID': tenantId,
            'X-User-ID': userId,
          }
        });

        if (res.ok) {
          const data = await res.json();
          if (data.has_onboarded) {
            localStorage.setItem('has_onboarded', 'true');
            router.push('/dashboard');
            return;
          }
        }
      } catch (err) {
        console.error('Failed to fetch onboarding status', err);
      }

      // Fallback or not onboarded
      if (hasOnboarded) {
         router.push('/dashboard');
      } else {
         router.push('/onboarding');
      }
    };

    if (hasOnboarded) {
      router.push('/dashboard');
    } else {
      checkOnboardingStatus();
    }
  }, [router, searchParams]);

  return null;
}

export default function Home() {
  return (
    <Suspense fallback={null}>
      <HomeContent />
    </Suspense>
  );
}
