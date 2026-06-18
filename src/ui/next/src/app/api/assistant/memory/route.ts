import { NextResponse } from 'next/server';
import { listMemories, mutateMemory } from '../store';

export async function GET() {
  return NextResponse.json({ memories: listMemories() });
}

export async function PATCH(request: Request) {
  const body = await request.json();
  const updated = mutateMemory(body);
  return NextResponse.json({ memories: updated });
}
