import { NextResponse } from 'next/server';
import { listExploreTemplates, mutateExplore, remixExploreTemplate } from '../store';

export async function GET() {
  return NextResponse.json(listExploreTemplates());
}

export async function POST(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json(remixExploreTemplate(payload || {}), { status: 201 });
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'explore template could not be remixed' }, { status: 400 });
  }
}

export async function PATCH(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json(mutateExplore(payload || {}));
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'explore remix could not be updated' }, { status: 400 });
  }
}
