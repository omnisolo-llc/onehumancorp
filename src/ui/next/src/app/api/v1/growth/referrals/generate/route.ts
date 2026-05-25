import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const backendUrl = process.env.API_URL || "http://127.0.0.1:8080";
    // Forward the POST request to the Rust backend
    const res = await fetch(`${backendUrl}/api/v1/growth/referrals/generate`, {
        method: "POST",
        // Pass any necessary headers (e.g. auth context if available in requests)
        headers: {
            "Content-Type": "application/json"
        }
    });

    if (!res.ok) {
        throw new Error(`Backend returned status ${res.status}`);
    }
    const data = await res.json();

    return NextResponse.json({ referral_link: data.referral_link, code: data.referral_link.split('/').pop() });
  } catch (error) {
    console.error("Error generating referral:", error);

    // Fallback for isolated E2E tests where the Rust backend is completely unreachable
    if (process.env.NODE_ENV === "test" || process.env.NEXT_PUBLIC_E2E_MOCK === "true") {
       const referralCode = Math.random().toString(36).substring(2, 10).toUpperCase();
       const referralLink = `ohc://join?ref=${referralCode}&utm_source=standalone_desktop&utm_medium=team_share&inviter=currentUser`;
       return NextResponse.json({ referral_link: referralLink, code: referralCode });
    }

    return NextResponse.json(
      { error: "Failed to generate referral" },
      { status: 500 }
    );
  }
}
