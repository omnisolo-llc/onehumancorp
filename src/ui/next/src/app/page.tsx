'use client';
import { useEffect } from 'react';
import { useRouter, useSearchParams } from 'next/navigation';

export default function Home() {
  const router = useRouter();
  const searchParams = useSearchParams();

  useEffect(() => {
    if (searchParams.get('dashboard') === '1') {
      router.push('/dashboard');
      return;
    }

    const hasOnboarded = localStorage.getItem('has_onboarded');
    if (hasOnboarded) {
      router.push('/dashboard');
    } else {
      router.push('/onboarding');
    }
  }, [router, searchParams]);

  return null;
}
