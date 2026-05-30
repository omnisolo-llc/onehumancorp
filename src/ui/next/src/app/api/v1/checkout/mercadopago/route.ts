import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const body = await req.json();

    // Simulate Mercado Pago API call to create preference
    await new Promise(resolve => setTimeout(resolve, 500));

    return NextResponse.json({
      success: true,
      init_point: 'https://www.mercadopago.com.br/checkout/v1/redirect?pref_id=mock_pref_123',
    });
  } catch (error) {
    return NextResponse.json({ error: 'Failed to create preference' }, { status: 500 });
  }
}
