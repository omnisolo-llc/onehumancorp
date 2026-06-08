import { NextResponse } from 'next/server';
import { createAssistantTask, getAssistantCapabilities, listAssistantTasks } from '../store';

export async function GET() {
  return NextResponse.json({ tasks: listAssistantTasks(), capabilities: getAssistantCapabilities() });
}

export async function POST(request: Request) {
  const payload = await request.json().catch(() => null);
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
  try {
    const res = await fetch(`${backendUrl}/api/assistant/task`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            ...(request.headers.get('Authorization') ? { 'Authorization': request.headers.get('Authorization') as string } : {})
        },
        body: JSON.stringify(payload || {}),
    });

    if (!res.ok) {
        // Fallback to store if backend is not available
        const task = createAssistantTask(payload || {});
        return NextResponse.json({ task }, { status: 201 });
    }

    const data = await res.json();
    return NextResponse.json({ task: data }, { status: 201 });
  } catch (error: any) {
    const task = createAssistantTask(payload || {});
    return NextResponse.json({ task }, { status: 201 });
  }
}
