'use client';

import { useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function ZeroClickBuilderPage() {
  const router = useRouter();

  useEffect(() => {
    // Redirect the old page to the new one
    router.replace('/onboarding/zero-click');
  }, [router]);

  return (
    <div className="min-h-screen flex items-center justify-center p-4 bg-[#F5F5F7]">
       <div className="w-16 h-16 border-4 border-[#0066FF]/30 border-t-[#0066FF] rounded-full animate-spin"></div>
    </div>
  );
}
