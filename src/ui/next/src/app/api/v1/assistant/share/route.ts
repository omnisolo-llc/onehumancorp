import { NextResponse } from 'next/server';
import { createShare, listShares, mutateShare } from '../store';

export async function GET() {
  return NextResponse.json(listShares());
}

export async function POST(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json({ share: createShare(payload || {}) }, { status: 201 });
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'share could not be created' }, { status: 400 });
  }
}

export async function PATCH(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json({ share: mutateShare(payload || {}) });
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'share action failed' }, { status: 400 });
  }
}
