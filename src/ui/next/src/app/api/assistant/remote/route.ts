import { NextResponse } from 'next/server';
import { createRemoteTask, listRemotePlatforms } from '../store';

export async function GET() {
  return NextResponse.json(listRemotePlatforms());
}

export async function POST(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    const result = createRemoteTask(payload || {});
    return NextResponse.json(result, { status: 202 });
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'remote task could not be created' }, { status: 400 });
  }
}
