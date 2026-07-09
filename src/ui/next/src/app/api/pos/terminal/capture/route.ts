import { NextRequest, NextResponse } from 'next/server';

export async function POST(req: NextRequest) {
  const STRIPE_SECRET_KEY = process.env.STRIPE_SECRET_KEY;

  if (!STRIPE_SECRET_KEY) {
    return NextResponse.json(
      { error: 'Stripe secret key is not configured.' },
      { status: 500 }
    );
  }

  try {
    const { paymentIntentId } = await req.json();

    if (!paymentIntentId) {
      return NextResponse.json(
        { error: 'Missing required parameter: paymentIntentId.' },
        { status: 400 }
      );
    }

    const response = await fetch(`https://api.stripe.com/v1/payment_intents/${paymentIntentId}/capture`, {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${STRIPE_SECRET_KEY}`,
        'Content-Type': 'application/x-www-form-urlencoded',
      },
    });

    if (!response.ok) {
      const errorData = await response.json();
      console.error('Failed to capture payment intent with Stripe:', errorData);
      return NextResponse.json(
        { error: 'Failed to capture Stripe payment intent.' },
        { status: response.status }
      );
    }

    const data = await response.json();

    // We would notify the event bus / operations agent here.
    // For now we assume optimistic UI + webhook fallback handling.

    return NextResponse.json(data);
  } catch (error) {
    console.error('Error capturing Stripe payment intent:', error);
    return NextResponse.json(
      { error: 'Internal server error while capturing payment intent.' },
      { status: 500 }
    );
  }
}
