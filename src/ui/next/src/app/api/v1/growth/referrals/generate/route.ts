import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    return NextResponse.json({
        referral_link: "ohc://join?ref=DEFAULT"
    });
  } catch (error) {
    console.error("Error generating referral code:", error);
    return NextResponse.json(
      { error: "Failed to generate referral code" },
      { status: 500 }
    );
  }
}
