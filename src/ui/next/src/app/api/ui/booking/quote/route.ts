import { NextResponse } from 'next/server';

export async function POST(request: Request) {
    const body = await request.json();

    // Simulate GRPC pass-through
    // In a real implementation this would use the gRPC client to call RequestQuoteWithImage
    await new Promise(resolve => setTimeout(resolve, 500));

    return NextResponse.json({
        quote_range: "$150 - $250",
        description: `Based on the image and description (${body.problem_description.substring(0, 20)}...), this appears to be a standard issue.`,
        deposit_stripe_link: `https://checkout.stripe.com/pay/cs_test_${Date.now()}`,
        available_slots: [
            { start_time: "Tomorrow, 10:00 AM", end_time: "Tomorrow, 11:00 AM" },
            { start_time: "Tomorrow, 2:00 PM", end_time: "Tomorrow, 3:00 PM" },
            { start_time: "Friday, 9:00 AM", end_time: "Friday, 10:00 AM" }
        ]
    });
}
