import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  try {
    const { searchParams } = new URL(request.url);
    const team_id = searchParams.get('team_id') || 'DEFAULT';
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const backendRes = await fetch(`${backendUrl}/api/v1/growth/team-invites/metrics?team_id=${team_id}`);
    if (backendRes.ok) {
      return NextResponse.json(await backendRes.json());
    }
    return NextResponse.json({ total_invites: 0 });
  } catch (error) {
    return NextResponse.json({ total_invites: 0 });
  }
}
