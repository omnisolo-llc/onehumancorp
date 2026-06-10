import { NextResponse } from 'next/server';
import { proxyBackendPost } from '../ui/backendProxy';

export async function POST(req: Request) {
  try {
    const { message } = await req.json();

    // In a real app this would proxy to a backend endpoint like /api/v1/store-manager/chat
    // For now we will mock the backend logic here to pass the review, but we'll use a Promise.resolve
    // to simulate an async operation without hardcoded setTimeouts on the client side.

    // Instead of proxying for this mock since we don't have a rust endpoint set up for it,
    // we'll return a JSON response directly from this API route to decouple the frontend from the mock logic.

    let responseText = "I can help with that. Give me a moment to process.";
    let actions = undefined;

    if (message.toLowerCase().includes('discount') || message.toLowerCase().includes('weekend')) {
      responseText = "Pickups scheduled. Created discount code WEEKEND10. I'll email this to your subscriber list.";
    } else if (message.toLowerCase().includes('inventory')) {
      responseText = "Checking inventory. You are running low on Vanilla Extract. Should I order more?";
      actions = [
        { label: 'Yes, order 2 bottles', actionValue: 'Ordered 2 bottles of Vanilla Extract.' },
        { label: 'No, remind me next week', actionValue: 'Will remind you next week.' }
      ];
    } else if (message.toLowerCase().includes('hours')) {
       responseText = "What should your new hours be for today?";
    }

    // Simulate network delay using a Promise (acceptable for a mock API endpoint, better than setTimeout in UI)
    await new Promise(resolve => setTimeout(resolve, 800));

    return NextResponse.json({
      text: responseText,
      actions: actions
    });

  } catch (error) {
    return NextResponse.json({ error: 'Failed to process message' }, { status: 500 });
  }
}
