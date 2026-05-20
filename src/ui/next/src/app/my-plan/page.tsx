"use client";
import React, { useState, useEffect } from 'react';

export default function MyPlanPage() {
  const [showCostDetails, setShowCostDetails] = useState(false);
  const [myPlan, setMyPlan] = useState<any>(null);
  const [costDashboard, setCostDashboard] = useState<any>(null);

  useEffect(() => {
    fetch('http://localhost:18789/my-plan')
      .then(res => res.json())
      .then(data => setMyPlan(data))
      .catch(e => console.error(e));
  }, []);

  const handleViewCostDetails = () => {
    setShowCostDetails(true);
    fetch('http://localhost:18789/cost-dashboard')
      .then(res => res.json())
      .then(data => setCostDashboard(data))
      .catch(e => console.error(e));
  };

  return (
    <div className="p-8">
      <h1>My Current Plan</h1>
      <p>Plan: {myPlan?.current_plan || 'Free'}</p>

      <button onClick={handleViewCostDetails}>View Cost Details</button>

      {showCostDetails && (
        <div className="mt-8">
          <h2>Cost & AI Usage</h2>
          <p>Actions used: {myPlan?.ai_actions_used || 0}</p>
        </div>
      )}
    </div>
  );
}
