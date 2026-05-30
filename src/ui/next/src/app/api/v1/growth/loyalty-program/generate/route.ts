import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const { rewardGoal, rewardItem } = await req.json();

    if (!rewardGoal || !rewardItem) {
      return NextResponse.json({ error: 'Missing parameters' }, { status: 400 });
    }

    // Convert generic inputs to a friendly AI-like string without duplication.
    // e.g., if goal is "Buy 5 items", we can just prepend "Join our loyalty program!"
    const goalLower = rewardGoal.toLowerCase();
    let actionStr = rewardGoal;

    // Attempt to avoid double-verb if the user typed "Buy 5 items"
    if (!goalLower.startsWith("buy ") && !goalLower.startsWith("spend ") && !goalLower.startsWith("visit ")) {
        actionStr = `complete ${rewardGoal}`;
    }

    const aiGeneratedResponse = `Join our loyalty program! ${actionStr} and get a free ${rewardItem}.`;

    return NextResponse.json({ result: aiGeneratedResponse }, { status: 200 });
  } catch (error) {
    return NextResponse.json({ error: 'Failed to generate loyalty program' }, { status: 500 });
  }
}
