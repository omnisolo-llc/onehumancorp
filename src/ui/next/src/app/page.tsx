'use client';
import { useEffect } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';

export default function Home() {
  const router = useRouter();

  useEffect(() => {
    const hasOnboarded = localStorage.getItem('has_onboarded');
    if (hasOnboarded) {
      router.push('/dashboard');
    }
  }, [router]);

  return (
    <div className="flex flex-col min-h-screen p-6 bg-gray-50">
      <nav className="flex items-center gap-4 mb-8">
         <Link href="/dashboard" className="px-4 py-2 bg-indigo-100 text-indigo-700 rounded-md font-medium hover:bg-indigo-200">Dashboard</Link>
         <Link href="/agents" className="px-4 py-2 bg-indigo-100 text-indigo-700 rounded-md font-medium hover:bg-indigo-200">Agents</Link>
         <Link href="/login" className="px-4 py-2 bg-gray-200 text-gray-800 rounded-md font-medium hover:bg-gray-300 ml-auto">Login</Link>
      </nav>
      <h1 className="text-4xl font-bold font-outfit text-gray-900 mb-4">Welcome to OHC</h1>
      <p className="text-gray-600 text-lg max-w-2xl mb-8">
        The Small Business App for Everyone. Launch, run, and grow your business entirely on your own.
      </p>
      <div className="flex gap-4">
        <button
            onClick={() => router.push('/onboarding')}
            className="px-6 py-3 bg-indigo-600 text-white font-bold rounded-xl hover:bg-indigo-700 transition-colors shadow-md"
        >
            Get Started
        </button>
      </div>
    </div>
  );
}
