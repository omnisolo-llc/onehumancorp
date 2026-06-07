import { NextResponse } from 'next/server';
import { listMemories, mutateMemory } from '../store';

export async function GET() {
  return NextResponse.json({ memories: listMemories() });
}

export async function PATCH(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    const memories = mutateMemory(payload || {});
    return NextResponse.json({ memories });
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'memory could not be updated' }, { status: 400 });
  }
}
