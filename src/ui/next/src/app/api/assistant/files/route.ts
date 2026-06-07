import { NextResponse } from 'next/server';
import { planFileOperation } from '../store';

export async function POST(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    const operation = planFileOperation(payload || {});
    return NextResponse.json({ operation }, { status: 202 });
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'file operation could not be planned' }, { status: 400 });
  }
}
