import { NextResponse } from "next/server";

export async function POST() {
  const tenant = "my-store"; // Ideally from auth context
  return NextResponse.json({
    referral_link: `https://ohc.store/join?ref=${tenant}&dynamic=true`
  });
}
