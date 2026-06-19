import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  // This endpoint simulates the Operations Agent route optimization.
  // It receives the current list of appointments and the staff's current location,
  // and returns an optimized order.

  try {
    const body = await request.json();
    const { appointments, currentLocationLat, currentLocationLng, delayMinutes, delayedJobId } = body;

    if (!appointments || !Array.isArray(appointments)) {
      return NextResponse.json({ error: 'Missing or invalid appointments data' }, { status: 400 });
    }

    // In a real implementation:
    // 1. Call Google Maps Distance Matrix API or open-source equivalent (OSRM)
    // 2. Solve Travelling Salesperson Problem (TSP) / Vehicle Routing Problem (VRP)
    // 3. Output the new optimal sequence and updated estimated start times

    let agentSuggestion = null;

    // Create a mutable copy of appointments to apply delays
    let optimized = [...appointments].sort((a, b) => {
       // Mock: push completed/cancelled to end
       if (['Completed', 'Cancelled'].includes(a.status)) return 1;
       if (['Completed', 'Cancelled'].includes(b.status)) return -1;

       // Sort by scheduled start time to keep them chronological
       const aTime = new Date(a.scheduled_start_time).getTime();
       const bTime = new Date(b.scheduled_start_time).getTime();
       return aTime - bTime;
    });

    // Apply delays if requested
    if (delayMinutes && delayedJobId) {
        let applyDelay = false;
        let delayedCount = 0;

        optimized = optimized.map(job => {
            if (job.id === delayedJobId) {
                applyDelay = true;
                return job; // Original job keeps its time, it's just running late
            } else if (applyDelay && !['Completed', 'Cancelled'].includes(job.status)) {
                // Apply cascading delay to subsequent pending jobs
                const newStart = new Date(new Date(job.scheduled_start_time).getTime() + delayMinutes * 60000);
                const newEnd = new Date(new Date(job.scheduled_end_time).getTime() + delayMinutes * 60000);
                delayedCount++;
                return {
                    ...job,
                    scheduled_start_time: newStart.toISOString(),
                    scheduled_end_time: newEnd.toISOString()
                };
            }
            return job;
        });

        if (delayedCount > 0) {
            agentSuggestion = `Drafting delay notifications for the next ${delayedCount} client${delayedCount > 1 ? 's' : ''}. Approve?`;
        }
    }

    // Calculate dynamic travel time blocks (simulated)
    for (let i = 0; i < optimized.length; i++) {
        // If there's a next job that isn't cancelled/completed
        if (i < optimized.length - 1 && !['Completed', 'Cancelled'].includes(optimized[i].status) && !['Completed', 'Cancelled'].includes(optimized[i+1].status)) {
            // Simplistic assumption: 15 minutes travel time
            optimized[i].travelBlock = "Travel Time: 15 mins";
        }
    }

    // Simulate a scenario where completing a job early triggers an agent suggestion
    if (!agentSuggestion && appointments.some(a => a.status === 'Completed' && new Date(a.actual_end_time) < new Date(a.scheduled_end_time))) {
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
