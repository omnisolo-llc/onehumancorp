import React from "react";

export default function ChangelogPage() {
  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>What&apos;s New</h1>
        <div className="flex items-center gap-3">
          <a href="/dashboard" className="text-sm font-medium text-blue-600 hover:underline">Back to Dashboard</a>
        </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full flex flex-col gap-8">
        <section>
          <h2 className="text-xl font-semibold mb-6 font-outfit" style={{ color: '#1D1D1F' }}>Recent Updates</h2>

          <div className="flex flex-col gap-6">
            <div className="p-6 shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
              <div className="flex items-center justify-between mb-3">
                <h3 className="text-lg font-bold font-outfit" style={{ color: '#1D1D1F' }}>v0.4.41</h3>
                <span className="text-xs font-medium px-2 py-1 bg-blue-50 text-blue-600 rounded-full">Latest</span>
              </div>
              <p className="text-sm text-gray-700 leading-relaxed mb-4">
                <strong>Help & Guides:</strong> Added an In-App Help Center and Contextual Tooltips. You can now get step-by-step guides, walkthroughs, and plain-language assistance directly within the app without ever leaving your workspace.
              </p>
              <div className="w-full h-48 bg-gray-200 rounded-xl border border-gray-300 flex items-center justify-center overflow-hidden">
                <img src="https://picsum.photos/seed/changelog1/800/400" alt="Help Center feature screenshot" className="w-full h-full object-cover opacity-80" />
              </div>
            </div>

            <div className="p-6 shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
              <h3 className="text-lg font-bold font-outfit mb-3" style={{ color: '#1D1D1F' }}>v0.4.32</h3>
              <p className="text-sm text-gray-700 leading-relaxed mb-4">
                <strong>Reliability:</strong> Health system improvements to ensure your data stays in sync securely, whether you&apos;re online or offline.
              </p>
              <div className="w-full h-48 bg-gray-200 rounded-xl border border-gray-300 flex items-center justify-center overflow-hidden">
                <img src="https://picsum.photos/seed/changelog2/800/400" alt="Reliability feature screenshot" className="w-full h-full object-cover opacity-80" />
              </div>
            </div>

            <div className="p-6 shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
              <h3 className="text-lg font-bold font-outfit mb-3" style={{ color: '#1D1D1F' }}>v0.4.30</h3>
              <p className="text-sm text-gray-700 leading-relaxed">
                <strong>Design:</strong> Updated the look and feel of the referral widget to give your store a more premium, modern aesthetic.
              </p>
            </div>

            <div className="p-6 shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
              <h3 className="text-lg font-bold font-outfit mb-3" style={{ color: '#1D1D1F' }}>v0.4.29</h3>
              <p className="text-sm text-gray-700 leading-relaxed">
                <strong>Team Communication:</strong> Improved how your AI team communicates, ensuring they can still work together smoothly even if your internet connection drops.
              </p>
            </div>

            <div className="p-6 shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
              <h3 className="text-lg font-bold font-outfit mb-3" style={{ color: '#1D1D1F' }}>v0.4.28</h3>
              <p className="text-sm text-gray-700 leading-relaxed">
                <strong>Help Center:</strong> Scaled the Help Center to work better across all environments, and ensured you can still access help guides when offline.
              </p>
            </div>

            <div className="p-6 shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
              <h3 className="text-lg font-bold font-outfit mb-3" style={{ color: '#1D1D1F' }}>v0.3.6</h3>
              <p className="text-sm text-gray-700 leading-relaxed">
                <strong>Storage:</strong> Made storage more efficient, reducing the space the app takes up on your local device.
              </p>
            </div>

            <div className="p-6 shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
              <h3 className="text-lg font-bold font-outfit mb-3" style={{ color: '#1D1D1F' }}>v0.3.5</h3>
              <p className="text-sm text-gray-700 leading-relaxed">
                <strong>Performance:</strong> Enhanced the underlying communication systems to make your AI team faster and more reliable.
              </p>
            </div>

            <div className="p-6 shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
              <h3 className="text-lg font-bold font-outfit mb-3" style={{ color: '#1D1D1F' }}>v0.3.4</h3>
              <p className="text-sm text-gray-700 leading-relaxed mb-4">
                <strong>Offline Mode:</strong> Implemented a fully offline-capable engine, ensuring your store operations continue running safely with automatic fallbacks.
              </p>
              <div className="w-full h-48 bg-gray-200 rounded-xl border border-gray-300 flex items-center justify-center overflow-hidden">
                <img src="https://picsum.photos/seed/changelog3/800/400" alt="Offline mode feature screenshot" className="w-full h-full object-cover opacity-80" />
              </div>
            </div>
          </div>

          <div className="mt-8 text-center">
            <a href="https://onehumancorp.com/changelog" target="_blank" rel="noopener noreferrer" className="inline-block px-6 py-3 bg-white border border-gray-300 rounded-xl text-sm font-semibold text-gray-700 hover:bg-gray-50 transition-colors shadow-sm">
              View full changelog on website →
            </a>
          </div>
        </section>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
