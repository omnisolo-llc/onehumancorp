"use client";

import { useState, useEffect } from "react";

export default function PaymentLedger() {
  const [revenue, setRevenue] = useState(0);
  const [amount, setAmount] = useState(50);
  const [status, setStatus] = useState("idle");

  useEffect(() => {
    fetchBalance();
  }, []);

  const fetchBalance = async () => {
    try {
      const res = await fetch("/api/ledger/balance");
      if (res.ok) {
        const data = await res.json();
        setRevenue(data.total_revenue);
      }
    } catch (e) {
      console.warn(e);
    }
  };

  const handleRequestPayment = async () => {
    setStatus("Processing...");
    try {
      const intentRes = await fetch("/api/payments/intent", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          amount: amount,
          currency: "USD",
          source: "tap_to_pay"
        })
      });

      if (!intentRes.ok) {
        setStatus("Failed to initialize");
        return;
      }

      const intentData = await intentRes.json();

      // Simulate Stripe Webhook
      const webhookRes = await fetch("/api/payments/webhook", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          type_field: "payment_intent.succeeded",
          data: {
            object: {
              id: "pi_mock_" + Date.now(),
              metadata: {
                tenant_id: "e2e-tenant", // Using default E2E tenant
                idempotency_key: intentData.idempotency_key
              }
            }
          }
        })
      });

      if (webhookRes.ok) {
        setStatus("Approved");
        fetchBalance();
      } else {
        setStatus("Failed");
      }
    } catch (e) {
      console.warn(e);
      setStatus("Error");
    }
  };

  return (
    <div style={{ maxWidth: "375px", margin: "0 auto", padding: "20px", fontFamily: "sans-serif" }}>
      <h2>Dashboard</h2>
      <div style={{ background: "#f0f0f0", padding: "20px", borderRadius: "10px", marginBottom: "20px" }}>
        <p style={{ margin: 0, color: "#666" }}>Today's Revenue</p>
        <h1 style={{ margin: "10px 0" }} data-testid="total-revenue">${revenue.toFixed(2)}</h1>
      </div>

      <div style={{ background: "#fafafa", padding: "20px", borderRadius: "10px" }}>
        <h3>Request Payment</h3>
        <input
          type="number"
          value={amount}
          onChange={(e) => setAmount(Number(e.target.value))}
          style={{ width: "100%", padding: "10px", marginBottom: "10px", boxSizing: "border-box" }}
          data-testid="payment-amount-input"
        />
        <button
          onClick={handleRequestPayment}
          disabled={status === "Processing..."}
          data-testid="request-payment-button"
          style={{
            width: "100%",
            padding: "15px",
            background: status === "Processing..." ? "#ccc" : "#0070f3",
            color: "white",
            border: "none",
            borderRadius: "5px",
            fontWeight: "bold",
            cursor: "pointer"
          }}
        >
          {status === "Processing..." ? "Waiting for card..." : "Tap to Pay"}
        </button>
        {status !== "idle" && status !== "Processing..." && (
          <p data-testid="payment-status" style={{ textAlign: "center", marginTop: "10px", color: status === "Approved" ? "green" : "red" }}>
            {status}
          </p>
        )}
      </div>
    </div>
  );
}
