import { NextRequest, NextResponse } from "next/server";

export async function POST(req: NextRequest) {
  try {
    const backendUrl = process.env.BACKEND_URL || "http://localhost:8080";
    const body = await req.json();

    const response = await fetch(`${backendUrl}/api/onboarding/launch`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(body),
    });

    if (!response.ok) {
      const errorText = await response.text();
      console.error("Backend launch returned error:", response.status, errorText);
      return NextResponse.json({ error: "Failed to launch store" }, { status: response.status });
    }

    const data = await response.json();
    return NextResponse.json(data);
  } catch (error) {
    console.error("Error in onboarding launch API:", error);
    return NextResponse.json({ error: "Internal Server Error" }, { status: 500 });
  }
}
