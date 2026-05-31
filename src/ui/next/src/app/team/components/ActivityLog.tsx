"use client";

import React, { useState, useEffect } from 'react';
import { ApprovalRequest } from '../page';

export default function ActivityLog() {
  const [activities, setActivities] = useState<ApprovalRequest[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchActivities = async () => {
      try {
        const response = await fetch('/api/agents/approvals/activity');
        if (response.ok) {
          const data = await response.json();
          setActivities(data.activities || []);
        }
      } catch (error) {
        console.error("Failed to fetch activities", error);
      } finally {
        setLoading(false);
      }
    };
    fetchActivities();
  }, []);

  if (loading) {
     return <div className="flex justify-center py-10">
       <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
     </div>;
  }

  return (
    <div className="flex flex-col space-y-4">
      <h2 className="text-xl font-bold font-outfit text-gray-900 tracking-tight">Recent Activity</h2>
      {activities.length === 0 ? (
         <div className="text-gray-500 text-sm">No recent activity.</div>
      ) : (
        activities.map(activity => (
          <div key={activity.id} className="bg-white p-4 rounded-xl shadow-sm border border-gray-100 flex flex-col">
             <div className="flex justify-between items-center mb-2">
                 <span className="text-xs font-bold uppercase tracking-wider text-gray-500">{activity.department}</span>
                 <span className={`text-xs font-medium px-2 py-1 rounded-md ${activity.status === 'APPROVED' ? 'bg-green-100 text-green-700' : 'bg-red-100 text-red-700'}`}>{activity.status}</span>
             </div>
             <p className="text-sm text-gray-800">{activity.description}</p>
          </div>
        ))
      )}
    </div>
  );
}
