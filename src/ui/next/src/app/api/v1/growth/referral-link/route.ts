import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  try {
    const { searchParams } = new URL(request.url);
    const tenantRaw = searchParams.get('tenant') || 'my-store';
    // Use an API integration with the backend if needed, or simply return the formatted link here
    // based on our audit that prefers dynamic links.

    // Fallback if backend is not available
    const link = `https://ohc.app/invite/${tenantRaw}`;
    return NextResponse.json({ link });
  } catch (error) {
    console.error("Error generating referral link:", error);
    // Fallback if fetch fails completely
    const link = `https://ohc.app/invite/team-default`;
    return NextResponse.json({ link });
  }
}
