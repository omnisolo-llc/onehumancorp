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
    if (hasOnboarded) {
      router.push('/unified-feed');
    } else {
      router.push('/onboarding');
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
