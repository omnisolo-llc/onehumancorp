import React from 'react';
import fs from 'fs';
import path from 'path';

export default function ChangelogPage() {
  // Fix the path to point to root correctly
  const changelogPath = path.join(process.cwd(), '..', '..', '..', 'CHANGELOG.md');
  let changelogContent = '';
  try {
    changelogContent = fs.readFileSync(changelogPath, 'utf8');
  } catch (err) {
    changelogContent = "";
  }

  // Very basic markdown parsing for display
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
          "- **Dashboard Analytics:** A brand new interactive walkthrough and Help Center article explaining how to read your store metrics.",
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
        <h1 className="text-3xl font-bold text-gray-900 mb-8 font-outfit">Release Notes & Changelog</h1>
        <div className="space-y-8">
          {sections.map((section, idx) => (
            <div key={idx} className="bg-white p-6 rounded-xl shadow-sm border border-gray-100">
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
                <img src="/dashboard_with_charts.png" alt="Screenshot" className="rounded-xl mt-4 max-w-full" />
              )}
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
