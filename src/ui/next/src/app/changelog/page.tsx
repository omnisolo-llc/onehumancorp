import React from 'react';
import fs from 'fs';
import path from 'path';
import { marked } from 'marked';

export default async function ChangelogPage() {
  const filePath = path.join(process.cwd(), '..', '..', '..', 'CHANGELOG.md');
  let changelogHtml = '';

  try {
    const fileContents = fs.readFileSync(filePath, 'utf8');
    changelogHtml = marked.parse(fileContents) as string;
  } catch (error) {
    changelogHtml = '<p>Could not load changelog.</p>';
  }

  return (
    <div className="min-h-screen bg-gray-50 flex justify-center py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-4xl w-full bg-white rounded-2xl shadow-xl overflow-hidden">
        <div className="px-8 py-6 border-b border-gray-100 bg-gray-50 flex items-center justify-between">
          <div>
            <h1 className="text-3xl font-bold text-gray-900 font-outfit">Release Notes</h1>
            <p className="mt-1 text-sm text-gray-500">What's new in One Human Corp</p>
          </div>
          <a href="/dashboard" className="text-sm font-semibold text-blue-600 hover:text-blue-800 transition-colors">
            ← Back to Dashboard
          </a>
        </div>
        <div className="p-8 prose prose-blue max-w-none">
          <div dangerouslySetInnerHTML={{ __html: changelogHtml }} />
        </div>
      </div>
    </div>
  );
}
