import { NextResponse } from 'next/server';
import { listPreviews, mutatePreview } from '../store';

export async function GET() {
  return NextResponse.json(listPreviews());
}

export async function PATCH(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json({ preview: mutatePreview(payload || {}) });
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'preview could not be updated' }, { status: 400 });
  }
}
