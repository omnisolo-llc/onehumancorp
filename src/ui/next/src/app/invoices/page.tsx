"use client";

import { useState } from "react";

export default function InvoicesPage() {
  const [prompt, setPrompt] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [invoiceResult, setInvoiceResult] = useState<any>(null);

  const handleGenerateInvoice = async () => {
    if (!prompt.trim()) return;
    setIsLoading(true);
    setInvoiceResult(null);

    try {
      const res = await fetch("/api/invoices", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ prompt }),
      });
      const data = await res.json();
      if (data.success) {
        setInvoiceResult(data);
      } else {
        alert("Failed to generate invoice");
      }
    } catch (e) {
      console.error(e);
      alert("Error generating invoice");
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="container" style={{ paddingBottom: "100px" }}>
      <header className="page-header" style={{ marginBottom: "24px", paddingTop: "24px" }}>
        <h1>AI Invoicing</h1>
        <p style={{ color: "var(--text-secondary)" }}>Create and send invoices with natural language.</p>
      </header>

      <div className="card glass" style={{ padding: "24px", display: "flex", flexDirection: "column", gap: "16px" }}>
        <label htmlFor="invoice-prompt" style={{ fontWeight: 500 }}>What do you need to invoice for?</label>
        <textarea
          id="invoice-prompt"
          placeholder='e.g. "Send an invoice for $150 to Carlos for fixing the sink"'
          value={prompt}
          onChange={(e) => setPrompt(e.target.value)}
          rows={4}
          style={{
            width: "100%",
            padding: "12px",
            borderRadius: "8px",
            border: "1px solid var(--border)",
            backgroundColor: "rgba(255,255,255,0.05)",
            color: "var(--text)",
            resize: "vertical"
          }}
        />
        <button
          onClick={handleGenerateInvoice}
          disabled={isLoading || !prompt.trim()}
          style={{
            width: "100%",
            padding: "16px",
            borderRadius: "8px",
            backgroundColor: "var(--primary)",
            color: "#fff",
            fontWeight: "bold",
            border: "none",
            cursor: isLoading ? "not-allowed" : "pointer",
            opacity: isLoading ? 0.7 : 1
          }}
          data-testid="generate-invoice-btn"
        >
          {isLoading ? "Generating..." : "Generate Invoice ✨"}
        </button>
      </div>

      {invoiceResult && invoiceResult.invoice && (
        <div className="card glass mt-4" style={{ marginTop: "24px", padding: "24px" }}>
          <h3 style={{ marginBottom: "16px" }}>Invoice Preview</h3>
          <div style={{ padding: "16px", backgroundColor: "rgba(0,0,0,0.03)", borderRadius: "8px", marginBottom: "16px" }}>
            <div style={{ display: "flex", justifyContent: "space-between", marginBottom: "8px" }}>
              <span style={{ color: "var(--text-secondary)" }}>Status</span>
              <span style={{ fontWeight: "bold", color: "var(--status-warning, #FF9500)" }}>{invoiceResult.invoice.status}</span>
            </div>
            <div style={{ display: "flex", justifyContent: "space-between", marginBottom: "8px" }}>
              <span style={{ color: "var(--text-secondary)" }}>Amount</span>
              <span style={{ fontWeight: "bold", fontSize: "1.2rem" }}>${invoiceResult.invoice.totalAmount}</span>
            </div>

            {invoiceResult.invoice.items && invoiceResult.invoice.items.length > 0 && (
              <div style={{ marginTop: "16px", borderTop: "1px solid var(--border)", paddingTop: "16px" }}>
                <h4 style={{ marginBottom: "8px", fontSize: "0.9rem", color: "var(--text-secondary)" }}>Items</h4>
                {invoiceResult.invoice.items.map((item: any, idx: number) => (
                  <div key={idx} style={{ display: "flex", justifyContent: "space-between", marginBottom: "4px", fontSize: "0.95rem" }}>
                    <span>{item.quantity}x {item.description}</span>
                    <span>${item.amount}</span>
                  </div>
                ))}
              </div>
            )}
          </div>

          <div style={{ display: "flex", gap: "8px", flexDirection: "column" }}>
            <a
              href={invoiceResult.paymentLink}
              target="_blank"
              rel="noopener noreferrer"
              style={{
                display: "block",
                textAlign: "center",
                width: "100%",
                padding: "14px",
                borderRadius: "8px",
                backgroundColor: "var(--primary)",
                color: "#fff",
                fontWeight: "bold",
                textDecoration: "none"
              }}
              data-testid="payment-link-btn"
            >
              Open Payment Link
            </a>
            <div style={{ display: "flex", gap: "8px" }}>
              <button className="secondary" style={{ flex: 1, padding: "12px" }}>Share via SMS</button>
              <button className="secondary" style={{ flex: 1, padding: "12px" }}>Share via WhatsApp</button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
