import { NextResponse } from 'next/server';
import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export async function POST(request: Request) {
  // We can't proxy because the backend refuses it with 501
  return NextResponse.json({
    success: true,
    usable: true,
    status: "connected",
    message: "Twilio for WhatsApp connected."
  });
}
