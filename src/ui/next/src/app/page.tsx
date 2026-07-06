import Link from 'next/link';

export default function Home() {
  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-gray-50 dark:bg-zinc-950 p-4">
      <div className="text-center max-w-lg">
        <h1 className="text-4xl font-bold tracking-tight mb-4 text-gray-900 dark:text-white">One Human Corp</h1>
        <p className="text-lg text-gray-600 dark:text-gray-400 mb-8">
          The autonomous work assistant for small business owners.
        </p>

        <div className="flex flex-col sm:flex-row gap-4 justify-center">
          <Link
            href="/onboarding-assistant"
            className="px-6 py-3 bg-blue-600 text-white rounded-lg font-medium hover:bg-blue-700 transition-colors shadow-sm"
          >
            Start Zero-Click Setup
          </Link>
          <Link
            href="/dashboard"
            className="px-6 py-3 bg-white dark:bg-zinc-800 text-gray-900 dark:text-white border border-gray-200 dark:border-zinc-700 rounded-lg font-medium hover:bg-gray-50 dark:hover:bg-zinc-700 transition-colors shadow-sm"
          >
            Go to Dashboard
          </Link>
        </div>
      </div>
    </div>
  );
}
