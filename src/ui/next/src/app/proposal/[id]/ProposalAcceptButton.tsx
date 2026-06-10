"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { FiCheck } from "react-icons/fi";

export default function ProposalAcceptButton({ id }: { id: string }) {
  const router = useRouter();
  const [approving, setApproving] = useState(false);

  const handleAccept = async () => {
    setApproving(true);
    try {
      const res = await fetch(`/api/proposals/${id}/accept`, {
        method: 'POST'
      });
      const data = await res.json();
      if (data.checkoutUrl) {
        window.location.href = data.checkoutUrl;
      } else {
        alert("Proposal accepted! You will be contacted shortly.");
        router.push("/dashboard");
      }
    } catch (e) {
      alert("Failed to accept proposal");
      setApproving(false);
    }
  };

  return (
    <button
      onClick={handleAccept}
      disabled={approving}
      data-testid="accept-proposal-btn"
      className="w-full py-4 px-4 font-semibold rounded-xl bg-blue-600 hover:bg-blue-700 text-white transition-all shadow-md flex justify-center items-center space-x-2 text-lg active:scale-[0.98]"
    >
      {approving ? (
        <span>Processing...</span>
      ) : (
        <>
          <FiCheck className="text-xl" />
          <span>Accept & Pay Deposit</span>
        </>
      )}
    </button>
  );
}
