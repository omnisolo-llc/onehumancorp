import { NextResponse } from 'next/server';
import { createExportArtifact } from '../store';

export async function POST(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    const artifact = createExportArtifact(payload || {});
    return NextResponse.json({ artifact }, { status: 201 });
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'artifact could not be exported' }, { status: 400 });
  }
}
