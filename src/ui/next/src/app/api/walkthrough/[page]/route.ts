import { NextResponse, NextRequest } from 'next/server';

export async function GET(
  request: NextRequest,
  { params }: { params: { page: string } }
) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
  const page = params.page;

  try {
    const res = await fetch(`${backendUrl}/api/walkthrough/${page}`);
    if (!res.ok) {
      throw new Error(`Backend responded with status: ${res.status}`);
    }
    const data = await res.json();

    // Map backend `selector` and `text` to frontend `targetId` and `content`
    const mappedData = data.map((step: any) => ({
      targetId: step.selector ? step.selector.replace('#', '') : step.targetId,
      title: step.title,
      content: step.text || step.content,
      position: step.position || 'bottom'
    }));

    return NextResponse.json(mappedData);
  } catch (error) {
    console.error(`Failed to fetch walkthrough for page ${page}:`, error);
    return NextResponse.json([], { status: 200 }); // Graceful fallback
  }
}
