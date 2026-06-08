import React, { useEffect, useState } from "react";
import { WithTooltip } from "../../../components/TooltipRegistry";
import { LedgerStatementModal } from "./LedgerStatementModal";

type TaxObligation = {
  jurisdiction: string;
  amount: number;
  currency: string;
};

type StatementEntry = {
  id: string;
  date: string;
  description: string;
  amount: number;
  type: string;
};

type LedgerData = {
  balance: number;
  currency: string;
  taxObligations: TaxObligation[];
  statement: StatementEntry[];
};

export const FinancialsWidget = () => {
  const [data, setData] = useState<LedgerData | null>(null);
  const [loading, setLoading] = useState(true);
  const [isModalOpen, setIsModalOpen] = useState(false);

  useEffect(() => {
    const fetchFinancials = async () => {
      try {
        const response = await fetch('/api/ledger');
        if (response.ok) {
          const json = await response.json();
          setData(json);
        } else {
          setData({ balance: 0, currency: "USD", taxObligations: [], statement: [] });
        }
      } catch (err) {
        console.error("Error fetching financials", err);
        setData({ balance: 0, currency: "USD", taxObligations: [], statement: [] });
      } finally {
        setLoading(false);
      }
    };

    fetchFinancials();
  }, []);

  if (loading || !data) {
    return <div className="p-4 bg-white/65 dark:bg-zinc-900/70 backdrop-blur-[30px] rounded-[16px] border border-white/40 dark:border-white/10 animate-pulse h-32"></div>;
  }

  const { balance = 0, currency = "USD", taxObligations = [], statement = [] } = data;
  const safeCurrency = currency || "USD";
  const totalTaxes = Array.isArray(taxObligations) ? taxObligations.reduce((acc, curr) => acc + (curr.amount || 0), 0) : 0;

  return (
    <>
      <div className="p-5 bg-white/65 dark:bg-zinc-900/70 backdrop-blur-[30px] rounded-[16px] border border-white/40 dark:border-white/10 shadow-sm flex flex-col gap-4">
        <div className="flex justify-between items-center">
          <h3 className="font-outfit text-lg font-semibold text-zinc-900 dark:text-zinc-50">Financials</h3>
          <WithTooltip tooltip="View full ledger statement">
            <button
              onClick={() => setIsModalOpen(true)}
              className="text-sm font-medium text-blue-600 dark:text-blue-400 hover:text-blue-800 transition-colors">
              Recent Activity
            </button>
          </WithTooltip>
        </div>

        <div className="grid grid-cols-2 gap-4">
          <div className="flex flex-col">
            <span className="text-sm text-zinc-500 dark:text-zinc-400 mb-1">Total Balance</span>
            <span className="text-3xl font-outfit font-bold tracking-tight text-zinc-900 dark:text-white">
              {new Intl.NumberFormat('en-US', { style: 'currency', currency: safeCurrency }).format(balance)}
            </span>
          </div>
          <div className="flex flex-col">
            <span className="text-sm text-zinc-500 dark:text-zinc-400 mb-1">Estimated Taxes Saved</span>
            <span className="text-3xl font-outfit font-bold tracking-tight text-zinc-900 dark:text-white">
              {new Intl.NumberFormat('en-US', { style: 'currency', currency: safeCurrency }).format(totalTaxes)}
            </span>
          </div>
        </div>

        {totalTaxes > 0 && (
           <div className="mt-2 p-3 bg-blue-50/50 dark:bg-blue-900/20 rounded-[8px] border border-blue-100 dark:border-blue-800/30 flex items-start justify-between">
              <div className="flex flex-col">
                 <span className="text-sm font-medium text-blue-900 dark:text-blue-100">Advisory: Tax Savings</span>
                 <span className="text-xs text-blue-700 dark:text-blue-300 mt-0.5">You have collected ${totalTaxes} in estimated taxes. Move to tax savings?</span>
              </div>
              <button className="h-[44px] min-w-[44px] px-4 rounded-[8px] bg-blue-600 hover:bg-blue-700 text-white font-medium text-sm transition-colors shadow-sm whitespace-nowrap">
                 Approve
              </button>
           </div>
        )}
      </div>

      <LedgerStatementModal
        isOpen={isModalOpen}
        onClose={() => setIsModalOpen(false)}
        statement={statement}
      />
    </>
  );
};
