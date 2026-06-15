"use client";

import { useState, useEffect } from "react";
import Head from "next/head";

export default function Dashboard() {
  const [data, setData] = useState<any>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Zero mock data requirement: fetch from actual API endpoint.
    fetch('/api/ui/dashboard/metrics')
      .then(res => res.json())
      .then(data => {
        setData(data);
        setLoading(false);
      })
      .catch(err => {
        console.error("Failed to fetch dashboard metrics", err);
        setLoading(false);
      });
  }, []);

  if (loading) {
    return <div className="p-8 text-gray-500">Loading Dashboard...</div>;
  }

  if (!data) {
    return <div className="p-8 text-gray-500">No dashboard data available.</div>;
  }

  return (
    <div className="p-8">
      <Head>
        <title>Dashboard | OHC</title>
      </Head>
      <h1 className="text-2xl font-bold mb-6">Dashboard</h1>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="glass-panel p-6">
          <h2 className="text-lg font-semibold text-gray-700">Total Revenue</h2>
          <p className="text-3xl font-bold mt-2">${(data.total_revenue_cents / 100).toFixed(2)}</p>
        </div>

        <div className="glass-panel p-6">
          <h2 className="text-lg font-semibold text-gray-700">Active Customers</h2>
          <p className="text-3xl font-bold mt-2">{data.active_customers}</p>
        </div>

        <div className="glass-panel p-6">
          <h2 className="text-lg font-semibold text-gray-700">Pending Orders</h2>
          <p className="text-3xl font-bold mt-2">{data.pending_orders}</p>
        </div>
      </div>
    </div>
  );
}
