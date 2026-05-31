'use client';
import { useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function Home() {
  const router = useRouter();

  useEffect(() => {
    const hasOnboarded = localStorage.getItem('has_onboarded');
    if (hasOnboarded) {
      router.push('/dashboard');
    } else {
      router.push('/onboarding');
    }
  }, [router]);

  return null;
}
