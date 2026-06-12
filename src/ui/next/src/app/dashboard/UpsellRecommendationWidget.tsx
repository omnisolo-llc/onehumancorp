"use client";

import React, { useState, useEffect } from "react";

export function UpsellRecommendationWidget() {
  const [upsell, setUpsell] = useState<{ title: string; recommendation: string; actionText: string } | null>(null);
  const [isGenerated, setIsGenerated] = useState(false);

  useEffect(() => {
    let currentTenant = "my-store";
    if (typeof localStorage !== "undefined") {
      currentTenant = localStorage.getItem("tenant") || "my-store";
    }

    fetch(`/api/v1/growth/upsell?tenant_id=${currentTenant}`)
      .then(res => res.json())
      .then(data => {
        if (!data.error) {
          setUpsell(data);
        }
      })
      .catch(err => console.error("Failed to fetch upsell recommendation", err));
  }, []);

  if (!upsell) return null;

  const handleGenerate = () => {
    setIsGenerated(true);
    setTimeout(() => setIsGenerated(false), 3000);
  };

  return (
    <section className="app-panel mb-6 border-2 border-orange-200 bg-gradient-to-br from-orange-50 to-white shadow-lg transform transition-all hover:scale-[1.01]">
      <div className="app-panel-header border-b border-orange-100 pb-4">
        <div>
          <h2 className="app-panel-title text-orange-900 flex items-center gap-2">
            <span>💡</span> {upsell.title}
          </h2>
          <div className="app-list-subtitle text-orange-700 font-medium mt-1">AI-driven Insight</div>
        </div>
      </div>
      <div className="app-panel-body pt-5">
        <div className="bg-white/60 backdrop-blur-sm p-4 rounded-xl border border-orange-50 mb-4">
          <p className="text-sm text-gray-700 italic">"{upsell.recommendation}"</p>
        </div>

        <div className="flex flex-col sm:flex-row gap-3">
          <button
            onClick={handleGenerate}
            className={`flex-1 py-3 px-4 rounded-xl font-bold font-outfit text-sm transition-all flex items-center justify-center gap-2 ${
              isGenerated
                ? "bg-green-500 text-white shadow-md shadow-green-200"
                : "bg-orange-600 text-white hover:bg-orange-700 shadow-md shadow-orange-200"
            }`}
          >
            {isGenerated ? (
              <><span>✓</span> Campaign Generated!</>
            ) : (
              <><span>⚡</span> {upsell.actionText}</>
            )}
          </button>
        </div>
      </div>
    </section>
  );
}
