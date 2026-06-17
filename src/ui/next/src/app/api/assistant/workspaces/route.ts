import { NextResponse } from 'next/server';
import { listWorkspaces, mutateWorkspace } from '../store';

export async function GET(request?: Request) {
  try {
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const tenantId = request?.headers?.get('x-tenant-id') || 'storefront';
    
    const headers: Record<string, string> = {
      'x-tenant-id': tenantId,
    };
    const authHeader = request?.headers?.get('Authorization');
    if (authHeader) {
      headers['Authorization'] = authHeader;
    }

    const res = await fetch(`${backendUrl}/api/assistant/workspaces`, {
      headers,
    });

    if (res.ok) {
      const data = await res.json();
      const workspaces = data.map((ws: any) => ({
        id: ws.id,
        name: ws.name,
        collapsed: false,
        pinned: false,
        archived: false,
        sortOrder: 0,
        memoryFile: 'MEMORY.md',
      }));
      return NextResponse.json({ workspaces, deleted: [] });
    }
  } catch (error) {
    console.error('Failed to fetch workspaces from backend:', error);
  }

  return NextResponse.json(listWorkspaces());
}

export async function PATCH(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json(mutateWorkspace(payload || {}));
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'workspace could not be updated' }, { status: 400 });
  }
}
