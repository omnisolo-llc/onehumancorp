import { NextResponse } from 'next/server';

export async function POST(req: Request, { params }: { params: { id: string } }) {
  // In a real app we'd update DB state
  return NextResponse.json({ success: true });
}
