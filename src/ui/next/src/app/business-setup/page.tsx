import Link from 'next/link';

export default function BusinessSetupCompatibilityPage() {
  return (
    <main id="business-setup-screen" className="min-h-screen flex items-center justify-center bg-[#F5F5F7] dark:bg-[#111111] p-4 sm:p-6 overflow-hidden relative">
      <div className="absolute top-[-10%] left-[-10%] w-[40%] h-[40%] rounded-full bg-[#0066FF] opacity-20 blur-[100px] pointer-events-none"></div>
      <div className="absolute bottom-[-10%] right-[-10%] w-[40%] h-[40%] rounded-full bg-[#34C759] opacity-20 blur-[100px] pointer-events-none"></div>

      <section className="w-full max-w-xl rounded-[16px] p-6 sm:p-8 shadow-lg border border-white/40 dark:border-white/10 z-10 transition-all"
               style={{
                 background: 'var(--glass-bg, rgba(255, 255, 255, 0.65))',
                 backdropFilter: 'blur(30px) saturate(210%)',
                 WebkitBackdropFilter: 'blur(30px) saturate(210%)',
               }}>
        <h1 className="text-3xl font-bold mb-3 font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">OneHumanCorp</h1>
        <h2 className="text-2xl font-semibold mb-3 text-[#1D1D1F] dark:text-[#F5F5F7]">Your business, live in minutes.</h2>
        <p className="text-gray-600 dark:text-gray-300 mb-6 text-sm sm:text-base leading-relaxed">
          Start the setup wizard to launch a database-backed OHC storefront, AI team, and operations workspace with zero technical knowledge.
        </p>
        <Link href="/onboarding" className="inline-flex items-center justify-center rounded-[8px] bg-[#0066FF] px-5 py-3 font-semibold text-white shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] hover:shadow-[0_6px_20px_rgba(0,102,255,0.23)] active:scale-[0.98] transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)] w-full sm:w-auto">
          Start Business Setup
        </Link>
      </section>

      <style dangerouslySetInnerHTML={{__html: `
        @media (prefers-color-scheme: dark) {
          section {
            --glass-bg: rgba(22, 22, 26, 0.7) !important;
          }
        }
      `}} />
    </main>
  );
}
