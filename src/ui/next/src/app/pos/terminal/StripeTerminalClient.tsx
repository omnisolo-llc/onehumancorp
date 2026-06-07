"use client";

import React, { useState, useEffect } from "react";
import { loadStripeTerminal } from "@stripe/terminal-js";

export default function StripeTerminalClient({
  amount,
  productId,
  tenantId,
}: {
  amount: number;
  productId: string;
  tenantId: string;
}) {
  const [terminal, setTerminal] = useState<any>(null);
  const [status, setStatus] = useState<string>("Initializing...");
  const [discoveredReaders, setDiscoveredReaders] = useState<any[]>([]);
  const [connectedReader, setConnectedReader] = useState<any>(null);
  const [reserving, setReserving] = useState<boolean>(false);

  useEffect(() => {
    async function initTerminal() {
      const StripeTerminal = await loadStripeTerminal();
      if (!StripeTerminal) {
        setStatus("Failed to load Stripe Terminal SDK.");
        return;
      }

      const term = StripeTerminal.create({
        onFetchConnectionToken: async () => {
          const res = await fetch("/api/terminal/connection_token", {
            method: "POST",
          });
          const data = await res.json();
          return data.secret;
        },
        onUnexpectedReaderDisconnect: () => {
          setStatus("Reader disconnected unexpectedly.");
          setConnectedReader(null);
        },
      });
      setTerminal(term);
      setStatus("Terminal initialized. Ready to discover readers.");
    }
    initTerminal();
  }, []);

  const discoverReaders = async () => {
    if (!terminal) return;
    setStatus("Discovering readers...");
    const result = await terminal.discoverReaders({ simulated: true });
    if (result.error) {
      setStatus("Discovery failed: " + result.error.message);
    } else {
      setDiscoveredReaders(result.discoveredReaders);
      setStatus("Discovered " + result.discoveredReaders.length + " readers.");
    }
  };

  const connectReader = async (reader: any) => {
    if (!terminal) return;
    setStatus("Connecting to reader...");
    const result = await terminal.connectReader(reader);
    if (result.error) {
      setStatus("Connection failed: " + result.error.message);
    } else {
      setConnectedReader(result.reader);
      setStatus("Connected to reader: " + result.reader.label);
    }
  };

  const processPayment = async () => {
    if (!terminal || !connectedReader) return;

    setReserving(true);
    setStatus("Reserving inventory...");

    let lockId = "";
    try {
      const reserveRes = await fetch("/api/v1/payments/terminal/reserve", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          tenant_id: tenantId,
          product_id: productId,
          quantity: 1,
          ttl_seconds: 15,
        }),
      });
      const reserveData = await reserveRes.json();

      if (!reserveData.success) {
        setStatus(
          "Reservation failed: " +
            (reserveData.error_message ||
              "Item is currently being purchased elsewhere"),
        );
        setReserving(false);
        return;
      }
      lockId = reserveData.lock_id;
    } catch (e: any) {
      setStatus("Reservation error: " + e.message);
      setReserving(false);
      return;
    }

    setStatus("Creating payment intent...");
    try {
      const res = await fetch("/api/terminal/create_payment_intent", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ amount, currency: "usd" }),
      });
      const data = await res.json();

      setStatus("Collecting payment method...");
      const collectResult = await terminal.collectPaymentMethod(
        data.client_secret,
      );
      if (collectResult.error) {
        setStatus("Payment collection failed: " + collectResult.error.message);
        setReserving(false);
        return;
      }

      setStatus("Processing payment...");
      const processResult = await terminal.processPayment(
        collectResult.paymentIntent,
      );
      if (processResult.error) {
        setStatus("Payment processing failed: " + processResult.error.message);
      } else {
        setStatus("Payment successful. Committing inventory...");

        try {
          const commitRes = await fetch("/api/v1/payments/terminal/commit", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
              tenant_id: tenantId,
              product_id: productId,
              quantity: 1,
              lock_id: lockId,
            }),
          });
          const commitData = await commitRes.json();
          if (commitData.success) {
            setStatus("Payment successful!");
          } else {
            setStatus(
              "Payment successful, but inventory commit failed: " +
                commitData.error_message,
            );
          }
        } catch (commitErr: any) {
          setStatus(
            "Payment successful, but inventory commit error: " +
              commitErr.message,
          );
        }
      }
    } catch (e: any) {
      setStatus("Error: " + e.message);
    } finally {
      setReserving(false);
    }
  };

  return (
    <div className="p-4 border rounded shadow bg-[rgba(255,255,255,0.6)] backdrop-blur-[30px] saturate-[210%] border-[rgba(255,255,255,0.4)]">
      <h2 className="text-xl font-bold mb-4">Stripe Terminal</h2>
      <p className="mb-4 text-gray-700">Status: {status}</p>

      {!connectedReader && (
        <div className="mb-4">
          <button
            onClick={discoverReaders}
            className="bg-blue-600 text-white px-4 py-2 rounded"
          >
            Discover Readers
          </button>
          <ul className="mt-2">
            {discoveredReaders.map((reader) => (
              <li
                key={reader.id}
                className="flex justify-between items-center my-2 p-2 border rounded"
              >
                <span>{reader.label || reader.id}</span>
                <button
                  onClick={() => connectReader(reader)}
                  className="bg-green-600 text-white px-3 py-1 rounded"
                >
                  Connect
                </button>
              </li>
            ))}
          </ul>
        </div>
      )}

      {connectedReader && (
        <div>
          <button
            onClick={processPayment}
            disabled={reserving}
            className={`bg-indigo-600 text-white px-4 py-2 rounded w-full ${reserving ? "opacity-50" : ""}`}
          >
            {reserving
              ? "Processing..."
              : `Charge $${(amount / 100).toFixed(2)}`}
          </button>
        </div>
      )}
    </div>
  );
}
