import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const { product_id, enable_subscribe_and_save, frequency_days, discount_percentage } = body;

    if (!product_id) {
      return NextResponse.json({ error: 'product_id is required' }, { status: 400 });
    }

    // Since this is a growth feature, we would typically save this configuration
    // to a database table like `subscription_configurations` linked to the tenant and product.
    // For now, we simulate a successful save.
    console.log(`Saved subscription config for ${product_id}: enabled=${enable_subscribe_and_save}, freq=${frequency_days}, discount=${discount_percentage}`);

    return NextResponse.json({
        success: true,
        message: 'Subscription configuration saved successfully.',
        data: {
            product_id,
            enable_subscribe_and_save,
            frequency_days,
            discount_percentage
        }
    });
  } catch (error) {
    console.error('Error saving subscription configuration:', error);
    return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });
  }
}