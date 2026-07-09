import { NextResponse } from 'next/server';

export async function POST() {
  const STRIPE_SECRET_KEY = process.env.STRIPE_SECRET_KEY;

  if (!STRIPE_SECRET_KEY) {
    return NextResponse.json(
      { error: 'Stripe secret key is not configured.' },
      { status: 500 }
    );
  }

  try {
    const response = await fetch('https://api.stripe.com/v1/terminal/connection_tokens', {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${STRIPE_SECRET_KEY}`,
        'Content-Type': 'application/x-www-form-urlencoded',
      },
    });

    if (!response.ok) {
      const errorData = await response.json();
      console.error('Failed to fetch connection token from Stripe:', errorData);
      return NextResponse.json(
        { error: 'Failed to create Stripe terminal connection token.' },
        { status: response.status }
      );
    }

    const data = await response.json();
    return NextResponse.json({ secret: data.secret });
  } catch (error) {
    console.error('Error creating Stripe terminal connection token:', error);
    return NextResponse.json(
      { error: 'Internal server error while creating connection token.' },
      { status: 500 }
    );
  }
}
