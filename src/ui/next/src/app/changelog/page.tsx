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
        "- Simplified checkout process for your customers.",
        "This is a plain paragraph test line."
      ]
    }
  ];

  return (
    <div className="min-h-screen bg-[#F5F5F7]/80 py-12 px-4 sm:px-6 lg:px-8 font-inter backdrop-blur-[20px] saturate-200">
      <div className="max-w-3xl mx-auto">
        <h1 className="text-3xl font-bold text-gray-900 mb-8 font-outfit text-center tracking-tight">Release Notes & Changelog</h1>
        <div className="space-y-8">
          {sections.map((section, idx) => (
            <div key={idx} className="bg-white/70 backdrop-blur-[24px] saturate-200 p-8 rounded-3xl shadow-[0_12px_40px_rgba(0,0,0,0.08)] border border-white/60 hover:bg-white/80 hover:border-blue-300 hover:shadow-[0_20px_60px_rgba(37,99,235,0.1)] transition-all duration-300 ease-out hover:-translate-y-1 ring-1 ring-black/5 group">
              <h2 className="text-xl font-bold text-blue-600 mb-4 font-outfit">{section.version}</h2>
              <div className="space-y-2">
                {section.contentLines.map((line, lidx) => {
                  if (line.startsWith('### ')) {
                    return <h3 key={lidx} className="text-lg font-semibold text-gray-800 mt-4 mb-2">{line.replace('### ', '')}</h3>;
                  }
                  if (line.startsWith('- ')) {
                    return <li key={lidx} className="text-gray-600 ml-4 list-disc">{line.replace('- ', '')}</li>;
                  }
                  return <p key={lidx} className="text-gray-600">{line}</p>;
                })}
              </div>
              {idx === 0 && (
                <img src="/dashboard_with_charts.png" alt="Screenshot" className="rounded-xl mt-4 max-w-full shadow-lg border border-gray-200/50" />
              )}
            </div>
          ))}

          <div className="mt-8 text-center">
            <a href="https://onehumancorp.com/changelog" target="_blank" rel="noopener noreferrer" className="text-blue-600 font-bold hover:underline bg-blue-50/80 backdrop-blur-md px-6 py-3 rounded-full border border-blue-100 inline-block shadow-sm">
              Read the full technical changelog on our website →
            </a>
          </div>
        </div>
      </div>
    </div>
  );
}
