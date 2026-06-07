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
    <div className="min-h-screen w-full max-w-[100vw] overflow-x-hidden bg-[#F5F5F7]/80 py-12 px-4 sm:px-6 lg:px-8 font-inter backdrop-blur-[20px] saturate-200">
      <div className="max-w-3xl mx-auto w-full px-2 sm:px-0">
        <h1 className="text-3xl font-bold text-gray-900 mb-8 font-outfit text-center tracking-tight">Release Notes & Changelog</h1>
        <div className="space-y-8 w-full">
          {sections.map((section, idx) => (
            <div key={idx} className="bg-white/70 backdrop-blur-[20px] saturate-200 p-5 sm:p-6 rounded-2xl shadow-[0_8px_32px_rgba(0,0,0,0.05)] border border-white/60 transition-all w-full overflow-hidden break-words">
              <h2 className="text-xl font-bold text-blue-600 mb-4 font-outfit">{section.version}</h2>
              <div className="space-y-2">
                {section.contentLines.map((line, lidx) => {
                  if (line.startsWith('### ')) {
                    return <h3 key={lidx} className="text-lg font-semibold text-gray-800 mt-4 mb-2 leading-snug">{line.replace('### ', '')}</h3>;
                  }
                  if (line.startsWith('- ')) {
                    return <li key={lidx} className="text-gray-600 ml-4 list-disc text-sm sm:text-base leading-relaxed break-words">{line.replace('- ', '')}</li>;
                  }
                  return <p key={lidx} className="text-gray-600 text-sm sm:text-base leading-relaxed break-words">{line}</p>;
                })}
              </div>
              {idx === 0 && (
                <img src="/dashboard_with_charts.png" alt="Screenshot" className="rounded-xl mt-4 max-w-full h-auto object-cover shadow-lg border border-gray-200/50" />
              )}
            </div>
          ))}

          <div className="mt-8 text-center px-2">
            <a href="https://onehumancorp.com/changelog" target="_blank" rel="noopener noreferrer" className="text-blue-600 font-bold hover:underline bg-blue-50/80 backdrop-blur-md px-4 sm:px-6 py-3 rounded-full border border-blue-100 inline-block shadow-sm text-sm sm:text-base break-words">
              Read the full technical changelog on our website →
            </a>
          </div>
        </div>
      </div>
    </div>
  );
}
