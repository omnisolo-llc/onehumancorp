import { NextResponse } from 'next/server';
import fs from 'fs';
import path from 'path';

export async function GET() {
  try {
    let content = '';

    // In NextJS dev mode, process.cwd() is src/ui/next
    // From /app/src/ui/next to /app/CHANGELOG.md is ../../../CHANGELOG.md
    const rootPath = path.join(process.cwd(), '..', '..', '..', 'CHANGELOG.md');

    if (fs.existsSync(rootPath)) {
        content = fs.readFileSync(rootPath, 'utf8');
    } else {
        const devPath = path.join(process.cwd(), 'CHANGELOG.md');
        if (fs.existsSync(devPath)) {
            content = fs.readFileSync(devPath, 'utf8');
        } else {
             content = "# Changelog\n\nNo recent updates to show.";
        }
    }
    return NextResponse.json({ content });
  } catch (e) {
    return NextResponse.json({ content: "# Changelog\n\nCould not load updates at this time." });
  }
}
