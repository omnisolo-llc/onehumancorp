"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";

export default function RequestProposalPage() {
  const router = useRouter();
  const [description, setDescription] = useState("");
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);

    try {
      const res = await fetch("/api/proposals/request", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          description,
          customer_name: name,
          customer_email: email,
        }),
      });
      const data = await res.json();
      if (data.success) {
        alert("Request submitted! You will receive a proposal shortly.");
        setDescription("");
        setName("");
        setEmail("");
        router.push(`/proposals/${data.inquiry_id}`); // Simulate redirecting to the generated proposal for MVP E2E
      } else {
        alert("Failed to submit request.");
      }
    } catch (err) {
      console.error(err);
      alert("Error submitting request.");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-900 flex items-center justify-center p-4">
      <div className="w-full max-w-md" style={{
          backdropFilter: "blur(20px) saturate(200%)",
          backgroundColor: "rgba(255, 255, 255, 0.05)",
          border: "1px solid rgba(255, 255, 255, 0.1)",
          borderRadius: "16px",
          padding: "24px",
      }}>
        <h1 className="text-2xl text-white font-semibold mb-6">Tell us what you need</h1>
        <form onSubmit={handleSubmit} className="flex flex-col gap-4">
          <input
            type="text"
            placeholder="Your Name"
            value={name}
            onChange={(e) => setName(e.target.value)}
            required
            className="p-3 rounded-lg bg-gray-800 text-white border border-gray-700"
          />
          <input
            type="email"
            placeholder="Your Email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            required
            className="p-3 rounded-lg bg-gray-800 text-white border border-gray-700"
          />
          <textarea
            placeholder="Describe your request (e.g. custom vegan wedding cake)"
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            required
            rows={5}
            className="p-3 rounded-lg bg-gray-800 text-white border border-gray-700"
          />
          <button
            type="submit"
            disabled={loading}
            className="mt-4 p-3 rounded-lg bg-blue-600 text-white font-semibold hover:bg-blue-700 disabled:opacity-50"
          >
            {loading ? "Submitting..." : "Get a Quote"}
          </button>
        </form>
      </div>
    </div>
  );
}
