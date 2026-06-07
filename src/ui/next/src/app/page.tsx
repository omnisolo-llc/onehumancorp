'use client';
import { useEffect, Suspense } from 'react';
import { useRouter, useSearchParams } from 'next/navigation';

<<<<<<< HEAD
function HomeContent() {
=======
import { Suspense } from 'react';

function HomeContent() {

>>>>>>> 42ebd8a4 (Enhance help center and walkthrough components with premium glassmorphism and fix Next.js build errors)
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

export default function Home() {
  return (
<<<<<<< HEAD
    <Suspense fallback={null}>
=======
    <Suspense>
>>>>>>> 42ebd8a4 (Enhance help center and walkthrough components with premium glassmorphism and fix Next.js build errors)
      <HomeContent />
    </Suspense>
  );
}
