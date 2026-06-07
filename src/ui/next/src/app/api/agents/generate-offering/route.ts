import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const { intent } = await request.json();

    // In a real implementation, this would call the AI backend
    // For this implementation, we return mock data based on the intent

    let title = "Custom Offering";
    let type = "Service";
    let price = "50.00";

    if (intent.toLowerCase().includes('guitar')) {
        title = "Beginner Guitar Lesson (1 Hour)";
    } else if (intent.toLowerCase().includes('cake') || intent.toLowerCase().includes('cupcake') || intent.toLowerCase().includes('cookie') || intent.toLowerCase().includes('baking')) {
        title = "Custom Baked Goods";
        type = "Product";
        price = "35.00";
    }

    return NextResponse.json({
      title: title,
      description: `Automatically generated description for: ${intent}. This covers all the essentials needed.`,
      type: type,
      price: price
    });
  } catch (error) {
    return NextResponse.json({ error: 'Failed to generate offering' }, { status: 500 });
  }
}
