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
    const { amount, currency, orderId } = await req.json();

    if (!amount || !currency) {
      return NextResponse.json(
        { error: 'Missing required parameters: amount or currency.' },
        { status: 400 }
      );
    }

    const params = new URLSearchParams();
    params.append('amount', amount.toString());
    params.append('currency', currency);
    params.append('payment_method_types[]', 'card_present');
    params.append('capture_method', 'manual');
    if (orderId) {
      params.append('metadata[orderId]', orderId);
    }

    const response = await fetch('https://api.stripe.com/v1/payment_intents', {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${STRIPE_SECRET_KEY}`,
        'Content-Type': 'application/x-www-form-urlencoded',
      },
      body: params,
    });

    if (!response.ok) {
      const errorData = await response.json();
      console.error('Failed to create payment intent from Stripe:', errorData);
      return NextResponse.json(
        { error: 'Failed to create Stripe payment intent.' },
        { status: response.status }
      );
    }

    const data = await response.json();
    return NextResponse.json({
      client_secret: data.client_secret,
      id: data.id,
    });
  } catch (error) {
    console.error('Error creating Stripe payment intent:', error);
    return NextResponse.json(
      { error: 'Internal server error while creating payment intent.' },
      { status: 500 }
    );
  }
}
