import { NextResponse } from 'next/server';

// In a real app this might use Redis, DB, or backend proxy
// For this harness UI, we mock the persistence of the chosen backend
let currentBackend = 'local';

export async function GET() {
  return NextResponse.json({ backend: currentBackend });
}

export async function POST(request: Request) {
  try {
    const { backend } = await request.json();
    if (backend === 'local' || backend === 'docker') {
      currentBackend = backend;
      return NextResponse.json({ success: true, backend: currentBackend });
    }
    return NextResponse.json({ error: 'Invalid backend' }, { status: 400 });
  } catch (err: any) {
    return NextResponse.json({ error: err.message }, { status: 500 });
  }
}
