import { NextResponse, NextRequest } from 'next/server';

export async function GET(
  request: NextRequest,
  context: { params: Promise<{ page: string }> }
) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
  const page = (await context.params).page;

  try {
    const res = await fetch(`${backendUrl}/api/walkthrough/${page}`);
    if (res.ok) {
        const data = await res.json();

        // Map backend `target_id`, `selector`, `text` and `content` to frontend `targetId` and `content`
        const mappedData = data.map((step: any) => ({
          targetId: step.target_id || step.targetId || (step.selector ? step.selector.replace('#', '') : ''),
          title: step.title,
          content: step.content || step.text,
          position: step.position || 'bottom'
        }));

        if (mappedData && mappedData.length > 0) return NextResponse.json(mappedData);
    }
  } catch (error) {
    console.error(`Failed to fetch walkthrough for page ${page}:`, error);
  }

  // Fallback if backend is unavailable
  if (page === 'dashboard') {
    return NextResponse.json([
      { targetId: 'dashboard-walkthrough-btn', title: 'Welcome', content: 'Welcome to your dashboard! This is your control center.' },
      { targetId: 'dashboard-walkthrough-btn', title: 'Stats', content: 'Here you can see the time and effort your agents have saved you.' }
    ]);
  }

  if (page === 'store-setup') {
      return NextResponse.json([
        { targetId: 'bio-input-tooltip', title: 'Business Description', content: 'Enter your business description.' },
        { targetId: 'generate-btn-tooltip', title: 'Generate', content: 'Click to generate!' }
      ]);
  }

  return NextResponse.json([]);
}
