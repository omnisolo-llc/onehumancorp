import React from 'react';
import Link from 'next/link';

export default function ChangelogPage() {
  const sections = [
    {
      version: "Version 1.0 (Latest)",
      contentLines: [
        "### 🌟 New Features",
        "- **Interactive AI Store Builder:** You can now generate a complete storefront from just a short description of your business. AI will handle the layout and copy for you.",
        "- **Smart Tooltips:** We added helpful text bubbles to all major buttons to help you learn the system faster.",
        "- **Help Center Upgrade:** Find answers instantly with our new searchable Help Center.",
        "### 🛠️ Improvements",
        "- Faster loading times for product images.",
        "- Simplified checkout process for your customers."
      ]
    }
  ];

  return (
    <div className="min-h-screen bg-gray-50 flex justify-center font-inter">
      <div className="w-full max-w-[375px] bg-[#F5F5F7] min-h-screen shadow-xl relative flex flex-col">
        <header className="px-5 pt-10 pb-4 bg-white/70 backdrop-blur-[30px] saturate-[210%] sticky top-0 z-20 border-b border-gray-200">
          <div className="flex justify-between items-center mb-4">
            <Link href="/dashboard" className="text-gray-500 hover:text-gray-900 transition-colors flex items-center justify-center min-w-[44px] min-h-[44px]">
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
            </Link>
            <span className="text-xs font-bold text-blue-600 uppercase tracking-widest bg-blue-50 px-2.5 py-1 rounded-full">Changelog</span>
          </div>
          <h1 className="text-3xl font-extrabold font-outfit text-gray-900 tracking-tight">Release Notes</h1>
        </header>

        <main className="flex-1 p-5 overflow-y-auto pb-24 space-y-6">
          <div className="space-y-6">
            {sections.map((section, idx) => (
              <div key={idx} className="bg-white/80 backdrop-blur-[30px] saturate-[210%] p-5 rounded-2xl shadow-sm border border-white/60 transition-all">
                <h2 className="text-lg font-bold text-blue-600 mb-3 font-outfit">{section.version}</h2>
                <div className="space-y-2">
                  {section.contentLines.map((line, lidx) => {
                    if (line.startsWith('### ')) {
                      return <h3 key={lidx} className="text-base font-bold text-gray-900 mt-4 mb-2 font-outfit">{line.replace('### ', '')}</h3>;
                    }
                    if (line.startsWith('- ')) {
                      return <li key={lidx} className="text-gray-600 text-sm ml-4 list-disc leading-relaxed">{line.replace('- ', '')}</li>;
                    }
                    return <p key={lidx} className="text-gray-600 text-sm leading-relaxed">{line}</p>;
                  })}
                </div>
                {idx === 0 && (
                  <img src="/dashboard_with_charts.png" alt="Screenshot" className="rounded-xl mt-4 max-w-full shadow-md border border-gray-100" />
                )}
              </div>
            ))}

            <div className="pt-4 pb-8 text-center">
              <a href="https://onehumancorp.com/changelog" target="_blank" rel="noopener noreferrer" className="text-blue-600 text-sm font-bold hover:underline bg-blue-50 px-5 py-3 rounded-xl border border-blue-100 inline-block w-full">
                Read the full technical changelog →
              </a>
            </div>
          </div>
        </main>
      </div>
      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
