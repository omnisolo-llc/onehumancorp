import Link from 'next/link';

export default function BusinessSetupCompatibilityPage() {
  return (
    <main id="business-setup-screen" className="min-h-screen flex items-center justify-center bg-[#F5F5F7] p-6">
      <section className="w-full max-w-xl rounded-[16px] bg-white p-8 shadow-sm border border-gray-100">
        <h1 className="text-3xl font-bold mb-3">OneHuman</h1>
        <h2 className="text-2xl font-semibold mb-3">Your business, live in minutes.</h2>
        <p className="text-gray-600 mb-6">
          Start the setup wizard to launch a database-backed OHC storefront and operations workspace.
        </p>
        <Link href="/onboarding" className="inline-flex items-center justify-center rounded-[8px] bg-[#0066FF] px-4 py-3 font-semibold text-white">
          Start Business Setup
        </Link>
      </section>
    </main>
  );
}
