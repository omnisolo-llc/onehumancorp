import { NextResponse } from 'next/server';

function haversineDistance(coords1, coords2) {
  function toRad(x) {
    return x * Math.PI / 180;
  }

  var lon1 = coords1.lng;
  var lat1 = coords1.lat;

  var lon2 = coords2.lng;
  var lat2 = coords2.lat;

  var R = 6371; // km

  var x1 = lat2 - lat1;
  var dLat = toRad(x1);
  var x2 = lon2 - lon1;
  var dLon = toRad(x2)
  var a = Math.sin(dLat / 2) * Math.sin(dLat / 2) +
    Math.cos(toRad(lat1)) * Math.cos(toRad(lat2)) *
    Math.sin(dLon / 2) * Math.sin(dLon / 2);
  var c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
  var d = R * c;

  return d;
}

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const { appointments, currentLocationLat, currentLocationLng } = body;

    if (!appointments || !Array.isArray(appointments)) {
      return NextResponse.json({ error: 'Missing or invalid appointments data' }, { status: 400 });
    }

    // Filter pending vs completed
    const completed = appointments.filter(a => ['Completed', 'Cancelled'].includes(a.status));
    let pending = appointments.filter(a => !['Completed', 'Cancelled'].includes(a.status));

    let currentLat = currentLocationLat || 0;
    let currentLng = currentLocationLng || 0;

    // Simulate Operations Agent Route Optimization using Nearest Neighbor
    const optimizedPending = [];
    while (pending.length > 0) {
       // Find nearest
       let nearestIndex = 0;
       let minDistance = Infinity;
       for (let i=0; i<pending.length; i++) {
          const apptLat = pending[i].location_lat || 0;
          const apptLng = pending[i].location_lng || 0;
          const dist = haversineDistance({lat: currentLat, lng: currentLng}, {lat: apptLat, lng: apptLng});
          if (dist < minDistance) {
              minDistance = dist;
              nearestIndex = i;
          }
       }

       const nextJob = pending.splice(nearestIndex, 1)[0];
       // Estimate travel time: roughly 2 mins per km, plus 5 min buffer
       const travelTimeMins = Math.round(minDistance * 2) + 5;

       // Update scheduled start time if we are optimizing
       // In a real app we'd shift the schedule based on travel time from NOW or previous job completion
       // For mock purposes we just append travel time info to notes
       if (minDistance > 0) {
           nextJob.notes = (nextJob.notes ? nextJob.notes + '\n' : '') + `[Travel: ~${travelTimeMins} mins]`;
       }

       optimizedPending.push(nextJob);
       currentLat = nextJob.location_lat || 0;
       currentLng = nextJob.location_lng || 0;
    }

    const optimized = [...completed, ...optimizedPending];

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
