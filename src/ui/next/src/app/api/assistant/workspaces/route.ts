import { NextResponse } from 'next/server';
import { listWorkspaces, mutateWorkspace } from '../store';

export async function GET() {
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
