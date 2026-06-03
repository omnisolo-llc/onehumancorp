"use client";

<<<<<<< HEAD
import { useEffect, useState } from "react";
import Link from "next/link";
import { AppShell } from "../components/AppShell";

type Order = {
  id: string;
  customer_name?: string;
  total_amount?: number;
  status?: string;
  created_at?: string;
};

function tenantId() {
  if (typeof window === "undefined") return "default";
  return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
}

function money(value: number | undefined) {
  return new Intl.NumberFormat("en-US", { style: "currency", currency: "USD" }).format(value || 0);
}

function badgeTone(status?: string) {
  const normalized = (status || "").toLowerCase();
  if (["paid", "completed", "shipped", "delivered"].includes(normalized)) return "good";
  if (["pending", "unfulfilled", "open"].includes(normalized)) return "warn";
  if (["failed", "cancelled", "canceled"].includes(normalized)) return "bad";
  return "";
}

export default function OrdersPage() {
  const [orders, setOrders] = useState<Order[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  useEffect(() => {
    async function loadOrders() {
      setLoading(true);
      setError("");
      try {
        const res = await fetch(`/api/ui/orders?tenant_id=${encodeURIComponent(tenantId())}`);
        if (!res.ok) throw new Error("Failed to load orders from the database");
        const data = await res.json();
        setOrders(Array.isArray(data) ? data : []);
      } catch (e: any) {
        setError(e?.message || "Failed to load orders");
      } finally {
        setLoading(false);
      }
    }
    loadOrders();
  }, []);

  return (
    <AppShell
      title="Orders"
      subtitle="Database-backed order queue with fulfillment status."
      statusItems={[
        { label: "Rows", value: String(orders.length), tone: orders.length > 0 ? "good" : "neutral" },
        { label: "Pending", value: String(orders.filter((order) => (order.status || "").toLowerCase() === "pending").length), tone: "warn" },
      ]}
      actions={[{ label: "Dashboard", href: "/dashboard" }]}
    >
      <div className="app-panel">
        <div className="app-panel-header">
          <div>
            <div className="app-panel-title">Order List</div>
            <div className="app-list-subtitle">Loaded from `/api/ui/orders`.</div>
          </div>
        </div>
        {error && <div className="app-empty">{error}</div>}
        {!error && orders.length === 0 ? (
          <div className="app-empty">{loading ? "Loading orders from the database..." : "No order rows found for this tenant."}</div>
        ) : (
          <div className="app-table-wrap">
            <table className="app-table">
              <thead>
                <tr>
                  <th>Order ID</th>
                  <th>Created</th>
                  <th>Customer</th>
                  <th>Total</th>
                  <th>Status</th>
                  <th>Action</th>
                </tr>
              </thead>
              <tbody>
                {orders.map((order) => (
                  <tr key={order.id}>
                    <td className="font-semibold">{order.id}</td>
                    <td>{order.created_at || "Unknown"}</td>
                    <td>{order.customer_name || "Unknown"}</td>
                    <td>{money(order.total_amount)}</td>
                    <td><span className={`app-badge ${badgeTone(order.status)}`}>{order.status || "Unknown"}</span></td>
                    <td><Link href={`/orders/${order.id}`} className="app-button">View</Link></td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>
    </AppShell>
=======
import { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function OrdersPage() {
  const router = useRouter();

  // Mock orders
  const [orders] = useState([
    {
      id: 'ORD-7829',
      customerName: 'Alice Johnson',
      items: '2x Vegan Chocolate Cake',
      total: '$45.00',
      status: 'unfulfilled',
      date: 'Oct 12, 2023',
    },
    {
      id: 'ORD-7830',
      customerName: 'Bob Smith',
      items: '1x Custom Birthday Cake',
      total: '$85.00',
      status: 'shipped',
      date: 'Oct 11, 2023',
    }
  ]);

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit text-gray-900">Orders</h1>
        <button onClick={() => router.push('/dashboard')} className="text-sm font-medium text-blue-600 hover:text-blue-800">
          Back to Dashboard
        </button>
      </header>

      <main className="p-6 max-w-4xl mx-auto w-full flex flex-col gap-6">
        <div className="bg-white rounded-2xl shadow-sm border border-gray-100 overflow-hidden">
          <table className="w-full text-left border-collapse">
            <thead>
              <tr className="bg-gray-50 border-b border-gray-100 text-sm text-gray-500 uppercase tracking-wide font-semibold">
                <th className="p-4">Order ID</th>
                <th className="p-4">Date</th>
                <th className="p-4">Customer</th>
                <th className="p-4">Items</th>
                <th className="p-4">Total</th>
                <th className="p-4">Status</th>
                <th className="p-4 text-right">Action</th>
              </tr>
            </thead>
            <tbody>
              {orders.map(order => (
                <tr key={order.id} className="border-b border-gray-50 hover:bg-gray-50 transition-colors">
                  <td className="p-4 font-medium text-gray-900">{order.id}</td>
                  <td className="p-4 text-sm text-gray-600">{order.date}</td>
                  <td className="p-4 text-sm text-gray-900">{order.customerName}</td>
                  <td className="p-4 text-sm text-gray-600 truncate max-w-[200px]">{order.items}</td>
                  <td className="p-4 text-sm font-medium text-gray-900">{order.total}</td>
                  <td className="p-4">
                    <span className={`px-2 py-1 text-xs rounded-full font-medium ${
                      order.status === 'unfulfilled' ? 'bg-yellow-100 text-yellow-800' : 'bg-green-100 text-green-800'
                    }`}>
                      {order.status === 'unfulfilled' ? 'Unfulfilled' : 'Shipped'}
                    </span>
                  </td>
                  <td className="p-4 text-right">
                    <button
                      onClick={() => router.push(`/orders/${order.id}`)}
                      className="px-3 py-1 bg-blue-50 text-blue-600 hover:bg-blue-100 rounded-lg text-sm font-medium transition-colors"
                    >
                      View
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </main>
    </div>
>>>>>>> e123d49a (feat: [architecture] Unified Multimodal Autonomous Customer Support Engine Research Report (#23362))
  );
}
