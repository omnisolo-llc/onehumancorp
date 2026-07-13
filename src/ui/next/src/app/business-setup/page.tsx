'use client';
import { useRouter } from 'next/navigation';
import { Suspense } from 'react';

function BusinessSetupContent() {
  const router = useRouter();

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#16161a] font-inter flex flex-col justify-center px-4 py-8 sm:px-6 lg:px-8">
      <div className="w-full sm:max-w-md lg:max-w-lg xl:max-w-2xl mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative overflow-hidden text-center p-8 justify-center glassmorphism">
        <h1 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#f5f5f7] mb-2">Your business, live in minutes.</h1>
        <p className="text-gray-500 dark:text-[#a1a1a6] mb-6 text-sm">Create your AI-powered storefront and start selling today.</p>

        <button
          className="w-full bg-[#0066FF] text-white font-bold p-4 active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] hover:bg-[#005bb5] rounded-lg"
          onClick={() => router.push('/onboarding')}
        >
          Start Business Setup
        </button>
      </div>
    </div>
  );
}

export default function BusinessSetup() {
  return (
    <Suspense fallback={null}>
      <BusinessSetupContent />
    </Suspense>
  );
}
