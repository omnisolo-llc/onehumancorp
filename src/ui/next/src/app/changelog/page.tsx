import React from 'react';
import fs from 'fs';
import path from 'path';

export default function ChangelogPage() {
  const changelogPath = path.join(process.cwd(), '..', '..', '..', 'CHANGELOG.md');
  let changelogContent = '';
  try {
    changelogContent = fs.readFileSync(changelogPath, 'utf8');
  } catch (err) {
    changelogContent = "";
  }

  let sections = changelogContent.split('## ').filter(Boolean).map(section => {
    const lines = section.split('\n');
    const version = lines[0].trim();
    const contentLines = lines.slice(1).filter(l => l.trim().length > 0);
    return { version, contentLines };
  });

  if (sections.length === 0 || sections[0].version === "No changelog found.") {
    sections = [
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
  }

  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-3xl mx-auto">
        <div className="mb-8 flex items-center justify-between">
           <a href="/dashboard" className="text-gray-500 hover:text-gray-900 transition-colors flex items-center gap-2 text-sm font-semibold">
             <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
             Back to Dashboard
           </a>
        </div>
        <h1 className="text-4xl font-bold text-gray-900 mb-8 font-outfit tracking-tight">Release Notes & Changelog</h1>

        <div className="space-y-8">
          {sections.map((section, idx) => (
            <div key={idx} className="bg-white p-8 rounded-2xl shadow-lg border border-gray-100" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)' }}>
              <div className="flex items-center gap-4 mb-6">
                <div className="w-12 h-12 bg-blue-100 text-blue-600 rounded-xl flex items-center justify-center font-bold font-outfit text-lg shadow-inner">
                  v1.0
                </div>
                <div>
                   <h2 className="text-2xl font-bold text-gray-900 font-outfit">{section.version}</h2>
                   <p className="text-sm text-gray-500 font-medium">Just shipped!</p>
                </div>
              </div>

              <div className="space-y-4">
                {section.contentLines.map((line, lidx) => {
                  if (line.startsWith('### ')) {
                    return <h3 key={lidx} className="text-lg font-bold text-gray-800 mt-6 mb-3 font-outfit tracking-wide">{line.replace('### ', '')}</h3>;
                  }
                  if (line.startsWith('- ')) {
                    const htmlLine = line.replace('- ', '').replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>');
                    return (
                       <div key={lidx} className="flex items-start gap-3 text-gray-600 mb-2">
                          <div className="w-1.5 h-1.5 rounded-full bg-blue-500 mt-2 shrink-0"></div>
                          <p dangerouslySetInnerHTML={{__html: htmlLine}} className="leading-relaxed"></p>
                       </div>
                    );
                  }
                  return <p key={lidx} className="text-gray-600 leading-relaxed">{line}</p>;
                })}
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
