import { NextResponse } from 'next/server';
import { createApproval, listApprovals, mutateApproval } from '../store';

export async function GET() {
  return NextResponse.json(listApprovals());
}

export async function POST(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json({ approval: createApproval(payload || {}) }, { status: 201 });
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'approval could not be created' }, { status: 400 });
  }
}

export async function PATCH(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json({ approval: mutateApproval(payload || {}) });
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'approval could not be updated' }, { status: 400 });
  }
}
