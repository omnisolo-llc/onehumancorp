import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  try {
    const { searchParams } = new URL(request.url);
    const tenant_id = searchParams.get('tenant_id') || 'DEFAULT';
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const backendRes = await fetch(`${backendUrl}/api/v1/growth/referrals/metrics?tenant_id=${tenant_id}`);
    if (backendRes.ok) {
      return NextResponse.json(await backendRes.json());
    }
    return NextResponse.json({ active_referrals: 0 });
  } catch (error) {
    return NextResponse.json({ active_referrals: 0 });
  }
}
