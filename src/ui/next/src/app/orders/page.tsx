"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { AppShell } from "../components/AppShell";
import { Card, CardHeader, CardTitle, CardContent } from "../../components/ui/card";
import { Button } from "../../components/ui/button";

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
  if (["paid", "completed", "shipped", "delivered"].includes(normalized)) return "bg-green-500/20 text-green-700 dark:text-green-400";
  if (["pending", "unfulfilled", "open"].includes(normalized)) return "bg-amber-500/20 text-amber-700 dark:text-amber-400";
  if (["failed", "cancelled", "canceled"].includes(normalized)) return "bg-red-500/20 text-red-700 dark:text-red-400";
  return "bg-gray-500/20 text-gray-700 dark:text-gray-400";
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
      <Card className="app-panel">
        <CardHeader className="app-panel-header">
          <div>
            <CardTitle className="app-panel-title">Order List</CardTitle>
            <div className="app-list-subtitle text-sm text-gray-500 dark:text-[#A1A1A6]">Live orders for the current tenant.</div>
          </div>
        </CardHeader>
        <CardContent>
          {error && <div className="p-8 text-center text-red-600 dark:text-red-400 bg-red-50 dark:bg-red-900/10 rounded-xl">{error}</div>}
          {!error && orders.length === 0 ? (
            <div className="p-12 flex flex-col items-center justify-center text-center">
              <div className="w-16 h-16 bg-gray-100 dark:bg-white/5 rounded-full flex items-center justify-center mb-4 text-gray-400">
                <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" /></svg>
              </div>
              <h3 className="text-lg font-medium text-gray-900 dark:text-gray-100 mb-1">
                {loading ? "Loading Orders" : "No Orders Found"}
              </h3>
              <p className="text-sm text-gray-500 dark:text-[#A1A1A6] max-w-sm">
                {loading ? "Syncing the latest data from the database..." : "You don't have any orders yet. Once customers start buying, they will appear here."}
              </p>
            </div>
          ) : (
            <div className="overflow-x-auto">
              <table className="w-full text-sm text-left">
                <thead>
                  <tr className="border-b border-gray-200 dark:border-white/10 text-gray-500 dark:text-[#A1A1A6]">
                    <th className="pb-3 pt-4 px-4 font-medium">Order ID</th>
                    <th className="pb-3 pt-4 px-4 font-medium">Created</th>
                    <th className="pb-3 pt-4 px-4 font-medium">Customer</th>
                    <th className="pb-3 pt-4 px-4 font-medium text-right">Total</th>
                    <th className="pb-3 pt-4 px-4 font-medium">Status</th>
                    <th className="pb-3 pt-4 px-4 font-medium text-right">Action</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-100 dark:divide-white/5">
                  {orders.map((order) => (
                    <tr key={order.id} className="hover:bg-gray-50/50 dark:hover:bg-white/5 transition-colors">
                      <td className="py-3 px-4 font-medium text-gray-900 dark:text-gray-100">{order.id}</td>
                      <td className="py-3 px-4 text-gray-600 dark:text-gray-400">{order.created_at || "Unknown"}</td>
                      <td className="py-3 px-4 text-gray-600 dark:text-gray-400">{order.customer_name || "Unknown"}</td>
                      <td className="py-3 px-4 text-right font-medium text-gray-900 dark:text-gray-100">{money(order.total_amount)}</td>
                      <td className="py-3 px-4">
                        <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${badgeTone(order.status)}`}>
                          {order.status || "Unknown"}
                        </span>
                      </td>
                      <td className="py-3 px-4 text-right">
                        <Link href={`/orders/${order.id}`}>
                          <Button variant="ghost" size="sm" className="h-8 text-[#0066FF] hover:text-[#0052CC] hover:bg-[#0066FF]/10">View</Button>
                        </Link>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </CardContent>
      </Card>
    </AppShell>
  );
}
