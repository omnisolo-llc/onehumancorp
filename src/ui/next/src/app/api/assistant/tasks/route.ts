import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  try {
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const tenantId = request.headers.get('x-tenant-id') || 'storefront';
    const headers: Record<string, string> = {
      'x-tenant-id': tenantId,
      'Content-Type': 'application/json',
    };
    const authHeader = request.headers.get('Authorization');
    if (authHeader) {
      headers['Authorization'] = authHeader;
    }

    const res = await fetch(`${backendUrl}/api/assistant/tasks`, { headers });
    if (res.ok) {
      const data = await res.json();
      return NextResponse.json({ tasks: data, capabilities: {} }); // We can mock capabilities here or proxy from backend later
    } else {
        return NextResponse.json({ error: 'Failed to fetch tasks' }, { status: res.status });
    }
  } catch (error) {
    console.error('Failed to fetch tasks from backend:', error);
    return NextResponse.json({ error: 'Backend unavailable' }, { status: 502 });
  }
}

export async function POST(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const tenantId = request.headers.get('x-tenant-id') || 'storefront';
    const headers: Record<string, string> = {
      'x-tenant-id': tenantId,
      'Content-Type': 'application/json',
    };
    const authHeader = request.headers.get('Authorization');
    if (authHeader) {
      headers['Authorization'] = authHeader;
    }

    const res = await fetch(`${backendUrl}/api/assistant/tasks`, {
      method: 'POST',
      headers,
      body: JSON.stringify(payload),
    });

    if (res.ok) {
      const createdTask = await res.json();
      return NextResponse.json({ task: createdTask }, { status: 201 });
    } else {
        return NextResponse.json({ error: 'Failed to create task' }, { status: res.status });
    }
  } catch (error) {
    console.error('Failed to create task in backend:', error);
  }

  return NextResponse.json({ error: 'Backend unavailable' }, { status: 502 });
}
