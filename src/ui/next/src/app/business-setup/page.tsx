import Link from 'next/link';

export default function BusinessSetupCompatibilityPage() {
  return (
    <main id="business-setup-screen" className="min-h-screen flex items-center justify-center bg-gradient-to-br from-[#f8f9fa] to-[#e9ecef] dark:from-[#000000] dark:to-[#1a1a1a] p-6">
      <section className="w-full max-w-xl glassmorphism p-8 shadow-sm border border-white/20">
        <h1 className="text-3xl font-bold mb-3 text-[#1D1D1F] dark:text-[#F5F5F7]">OneHuman</h1>
        <h2 className="text-2xl font-semibold mb-3 text-[#1D1D1F] dark:text-[#F5F5F7]">Your business, live in minutes.</h2>
        <p className="text-gray-600 dark:text-[#A1A1A6] mb-6">
          Start the setup wizard to launch a database-backed OHC storefront and operations workspace.
        </p>
        <Link href="/onboarding" className="inline-flex items-center justify-center rounded-[8px] bg-[#0066FF] px-4 py-3 font-semibold text-white">
          Start Business Setup
        </Link>
      </section>
    </main>
  );
}
