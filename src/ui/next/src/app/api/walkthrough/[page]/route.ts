import { NextResponse, NextRequest } from 'next/server';

export async function GET(
  request: NextRequest,
  context: { params: Promise<{ page: string }> }
) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
  const page = (await context.params).page;

  try {
    const res = await fetch(`${backendUrl}/api/walkthrough/${page}`);
    if (!res.ok) {
      throw new Error(`Backend responded with status: ${res.status}`);
    }
    const data = await res.json();

    // Map backend `target_id`, `selector`, `text` and `content` to frontend `targetId` and `content`
    const mappedData = data.map((step: any) => ({
      targetId: step.target_id || step.targetId || (step.selector ? step.selector.replace('#', '') : ''),
      title: step.title,
      content: step.content || step.text,
      position: step.position || 'bottom'
    }));

    return NextResponse.json(mappedData);
  } catch (error) {
    console.error(`Failed to fetch walkthrough for page ${page}:`, error);
    return NextResponse.json({ error: 'Backend walkthrough service unavailable' }, { status: 502 });
  }
}
