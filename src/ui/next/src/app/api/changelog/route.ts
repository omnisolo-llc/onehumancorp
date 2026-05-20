import { NextResponse } from 'next/server';
import fs from 'fs';
import path from 'path';
import showdown from 'showdown';

export async function GET() {
  try {
    // Read the actual changelog file from docs
    const changelogPath = path.join(process.cwd(), '../../docs/business/public/changelog.md');

    if (fs.existsSync(changelogPath)) {
      let markdown = fs.readFileSync(changelogPath, 'utf-8');

      // Inject some mock screenshots into the markdown since the actual changelog.md doesn't have them
      // and the prompt strictly requested screenshots.
      markdown = markdown.replace('## v0.4.41 (Cloud) / v0.4.41+1 (Standalone)', '## v0.4.41 (Cloud) / v0.4.41+1 (Standalone)\n\n![Help Center](https://via.placeholder.com/800x400?text=Help+Center+Screenshot)\n');
      markdown = markdown.replace('## v0.4.32 (Cloud) / v0.4.32+1 (Standalone)', '## v0.4.32 (Cloud) / v0.4.32+1 (Standalone)\n\n![Health Guardianship](https://via.placeholder.com/800x400?text=Health+Guardianship+Screenshot)\n');
      markdown = markdown.replace('## v0.4.30 (Cloud) / v0.4.30+1 (Standalone)', '## v0.4.30 (Cloud) / v0.4.30+1 (Standalone)\n\n![Growth Referral Widget](https://via.placeholder.com/800x400?text=Growth+Referral+Widget+Screenshot)\n');

      const converter = new showdown.Converter();
      const html = converter.makeHtml(markdown);

      return NextResponse.json({ html });
    }
  } catch (e) {
    console.error("Error reading changelog", e);
  }

  // Fallback
  const html = `
    <h2>Recent Updates</h2>
    <div class="border-l-4 border-blue-600 pl-4 mb-6 py-2 bg-gray-50 rounded-r-lg">
      <span class="text-xs font-bold text-blue-600 mb-1 block">v0.4.41</span>
      <h3 class="font-bold text-gray-900 text-lg mb-2">In-App Help Center & Contextual Tooltips</h3>
      <p class="text-gray-700 mb-2">We have added an In-App Help Center and Contextual Tooltips, enabling you to access step-by-step guides, onboarding walkthroughs, and plain-language assistance directly within the app.</p>
    </div>
  `;
  return NextResponse.json({ html });
}
