import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  // This endpoint simulates the Operations Agent route optimization.
  // It receives the current list of appointments and the staff's current location,
  // and returns an optimized order.

  try {
    const body = await request.json();
    const { appointments, currentLocationLat, currentLocationLng } = body;

    if (!appointments || !Array.isArray(appointments)) {
      return NextResponse.json({ error: 'Missing or invalid appointments data' }, { status: 400 });
    }

    // In a real implementation:
    // 1. Call Google Maps Distance Matrix API or open-source equivalent (OSRM)
    // 2. Solve Travelling Salesperson Problem (TSP) / Vehicle Routing Problem (VRP)
    // 3. Output the new optimal sequence and updated estimated start times

    // Simulate simple reordering (just reversing for demonstration)
    // A real algorithm would re-sort by distance from previous node
    const optimized = [...appointments].sort((a, b) => {
       // Mock: push completed/cancelled to end
       if (['Completed', 'Cancelled'].includes(a.status)) return 1;
       if (['Completed', 'Cancelled'].includes(b.status)) return -1;
       return 0; // maintain relative order of pending for now
    });

    // Simulate a scenario where completing a job early triggers an agent suggestion
    let agentSuggestion = null;
    if (appointments.some(a => a.status === 'Completed' && new Date(a.actual_end_time) < new Date(a.scheduled_end_time))) {
        agentSuggestion = "You finished early! Should I text the next client to see if we can arrive early?";
    }

    return NextResponse.json({
        success: true,
        optimizedRoute: optimized,
        agentSuggestion
    });
  } catch (error) {
    return NextResponse.json({ error: 'Invalid request' }, { status: 400 });
  }
}
