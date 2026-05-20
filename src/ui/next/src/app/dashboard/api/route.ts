import { NextResponse } from 'next/server';

export async function GET() {
  // Simulate backend aggregation of daily metrics and agent tasks
  const dailyBriefing = {
    title: "Good Morning, Carlos",
    date: new Date().toLocaleDateString('en-US', { weekday: 'long', month: 'short', day: 'numeric' }),
    summary: [
      "You had 8 new inquiries this week.",
      "Vegan cake requests doubled in the last 3 days.",
      "Consider adding a vegan chocolate option to your catalog!"
    ]
  };

  const agentTasks = [
    {
      id: "t_1",
      agentName: "The Ambassador",
      agentRole: "Customer Success",
      actionType: "Draft Reply",
      description: "Drafted a reply to a vegan cake inquiry from Maria.",
      previewText: "Hi Maria! Thanks for reaching out. Yes, we can absolutely make a vegan version of our classic chocolate cake. Would you like to proceed with an order for this Saturday?",
      status: "pending"
    },
    {
      id: "t_2",
      agentName: "The Generative Promoter",
      agentRole: "Marketing",
      actionType: "Social Post",
      description: "Generated an Instagram post for the new Summer Berry Tart.",
      previewText: "Summer is here, and so is our new Berry Tart! 🍓 Made with local strawberries and a buttery crust. #SummerDesserts #LocalBakery",
      status: "pending"
    },
    {
      id: "t_3",
      agentName: "The Vigilant Manager",
      agentRole: "Operations",
      actionType: "Inventory Alert",
      description: "Low stock alert: Almond Flour (2 bags left).",
      previewText: "Would you like me to place an order for 10 bags of Almond Flour from your primary supplier?",
      status: "pending"
    }
  ];

  return NextResponse.json({ dailyBriefing, agentTasks });
}
