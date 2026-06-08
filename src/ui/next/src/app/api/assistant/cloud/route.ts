import { NextResponse } from 'next/server';
import { createCloudSession, listCloudSessions, mutateCloudSession } from '../store';

export async function GET() {
  return NextResponse.json(listCloudSessions());
}

export async function POST(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json(createCloudSession(payload || {}), { status: 201 });
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'cloud session could not be started' }, { status: 400 });
  }
}

export async function PATCH(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json(mutateCloudSession(payload || {}));
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'cloud session could not be updated' }, { status: 400 });
  }
}
