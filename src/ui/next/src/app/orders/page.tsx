"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { AppShell } from "../components/AppShell";
import { WithTooltip } from "../../components/TooltipRegistry";

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
            <WithTooltip id="orders-order-list" defaultText="All orders placed by customers across your sales channels."><div className="app-panel-title">Order List</div></WithTooltip>
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
  );
}
