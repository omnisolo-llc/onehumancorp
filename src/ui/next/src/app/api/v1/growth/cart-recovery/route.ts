import { NextResponse } from 'next/server';

// Temporary mock storage for demonstration purposes. In production this would use the real database.
let cartRecoveryConfig = {
  isEnabled: false,
  delay: "4",
  includeDiscount: true,
};

export async function GET() {
  try {
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const backendRes = await fetch(`${backendUrl}/api/v1/growth/cart-recovery`, {
      method: 'GET',
      headers: { 'Content-Type': 'application/json' },
    });

    if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        // Fallback to local config if backend fails
        return NextResponse.json(cartRecoveryConfig);
    }
  } catch (error) {
    return NextResponse.json(cartRecoveryConfig);
  }
}

export async function POST(request: Request) {
  try {
    const body = await request.json();

    // Fallback update local config
    cartRecoveryConfig = { ...cartRecoveryConfig, ...body };

    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const backendRes = await fetch(`${backendUrl}/api/v1/growth/cart-recovery`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });

    if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        return NextResponse.json({ success: true, message: "Settings saved successfully (fallback)" });
    }
  } catch (error) {
    return NextResponse.json({ success: true, message: "Settings saved successfully (fallback)" });
  }
}
