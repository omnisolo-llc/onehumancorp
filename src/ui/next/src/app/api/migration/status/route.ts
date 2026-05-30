import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const url = new URL(request.url);
  const jobId = url.searchParams.get("job_id");
  const tenantId = request.headers.get('x-tenant-id') || 'default';

  if (!jobId) {
     return NextResponse.json({ error: 'Missing job_id' }, { status: 400 });
  }

  try {
    const res = await fetch(`${backendUrl}/api/migration/status/${jobId}`, {
      headers: {
        'x-tenant-id': tenantId,
      }
    });

    if (res.ok) {
        const data = await res.json();
        return NextResponse.json(data);
    }

    return NextResponse.json({ error: 'Failed to get job status' }, { status: res.status });
  } catch (e) {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
