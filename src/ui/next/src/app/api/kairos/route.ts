import { NextResponse } from 'next/server';

export async function GET() {
  const activeTasks = [
    { id: "task-1", name: "Inventory Reorder Strategy", status: "In Progress", priority: "High" },
    { id: "task-2", name: "Customer Sentiment Analysis", status: "Queued", priority: "Medium" },
    { id: "task-3", name: "Social Media Campaign Draft", status: "Completed", priority: "Low" },
  ];

  const meshNodes = [
    { id: "node-1", type: "Brain", status: "Online", load: "12%" },
    { id: "node-2", type: "Nerve", status: "Online", load: "45%" },
    { id: "node-3", type: "Memory", status: "Online", load: "8%" },
  ];

  const autoDreamMemory = {
    knowledgeDensity: "842.5 MB",
    semanticClusters: "12 Active"
  };

  return NextResponse.json({ activeTasks, meshNodes, autoDreamMemory });
}
