import { NextResponse } from 'next/server';
import { createUpload, listUploads } from '../store';

export async function GET() {
  return NextResponse.json(listUploads());
}

export async function POST(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json({ upload: createUpload(payload || {}) }, { status: 201 });
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'upload could not be created' }, { status: 400 });
  }
}
