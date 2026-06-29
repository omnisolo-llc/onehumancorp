"use client";

import React, { useState } from "react";
import { useParams } from "next/navigation";

export default function ClientPortalPage() {
  const { proposal_id } = useParams();
  const [signed, setSigned] = useState(false);

  // Mock data for client viewing. In real implementation, this fetches from the backend.
  const proposalData = {
    title: "Custom Project Proposal",
    scope: "Based on your inquiry, this proposal includes comprehensive design, implementation, and deployment.",
    price: "$500.00",
    legal: "Standard legal terms and conditions apply for this project. Deposit is required prior to commencement.",
    stripe_link: "https://checkout.stripe.mock/pay/" + proposal_id,
  };

  const handleSign = () => {
    setSigned(true);
  };

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#1D1D1F] p-4 sm:p-8 flex justify-center items-center font-sans">
      <div className="w-full max-w-2xl bg-white dark:bg-[#2C2C2E] rounded-[16px] shadow-lg p-6 sm:p-10 border border-gray-200 dark:border-gray-700">
        <h1 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">
          {proposalData.title}
        </h1>
        <p className="text-sm text-gray-500 mb-6 font-mono">ID: {proposal_id}</p>

        <section className="mb-8">
          <h2 className="text-xl font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Project Scope</h2>
          <div className="p-4 bg-gray-50 dark:bg-gray-800 rounded-[8px] text-gray-700 dark:text-gray-300">
            {proposalData.scope}
          </div>
        </section>

        <section className="mb-8">
          <h2 className="text-xl font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Investment</h2>
          <div className="p-4 bg-gray-50 dark:bg-gray-800 rounded-[8px] text-gray-900 dark:text-gray-100 font-bold text-2xl">
            {proposalData.price}
          </div>
        </section>

        <section className="mb-8">
          <h2 className="text-xl font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Contract Terms</h2>
          <div className="p-4 bg-gray-50 dark:bg-gray-800 rounded-[8px] text-gray-700 dark:text-gray-300 text-sm">
            {proposalData.legal}
          </div>
        </section>

        <div className="mt-10 border-t border-gray-200 dark:border-gray-700 pt-6 flex flex-col gap-4">
          {!signed ? (
            <button
              onClick={handleSign}
              className="w-full min-h-[44px] bg-[#0066FF] hover:bg-[#0052CC] text-white font-semibold rounded-[8px] transition-all flex items-center justify-center text-lg"
              data-testid="sign-contract-btn"
            >
              Sign Contract
            </button>
          ) : (
            <div className="flex flex-col gap-4">
              <div className="w-full p-4 bg-green-50 dark:bg-green-900/30 text-green-700 dark:text-green-300 rounded-[8px] text-center font-medium border border-green-200 dark:border-green-800">
                ✅ Contract Signed
              </div>
              <a
                href={proposalData.stripe_link}
                className="w-full min-h-[44px] bg-[#635BFF] hover:bg-[#4B45C6] text-white font-semibold rounded-[8px] transition-all flex items-center justify-center text-lg"
                data-testid="pay-deposit-btn"
              >
                Pay Deposit
              </a>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
