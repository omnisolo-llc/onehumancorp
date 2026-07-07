import React, { useState, useEffect } from 'react';
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { AlertCircle, CheckCircle2, ChevronRight, CreditCard, PieChart, Database, Zap } from "lucide-react";
import { Badge } from "@/components/ui/badge";

interface PlanMetrics {
  current_tier: string;
  monthly_ai_actions: number;
  ai_actions_limit: number;
  storage_used_mb: number;
  storage_limit_mb: number;
  estimated_next_bill: number;
  projected_cost: number;
  budget_alert?: boolean;
}

const formatCurrency = (amount: number) => {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD',
  }).format(amount / 100);
};

export default function PlanPage() {
  const [metrics, setMetrics] = useState<PlanMetrics | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    // In a real app, this would be an API call to our backend
    // which integrates with Stripe Billing and our internal metrics DB
    const fetchMetrics = async () => {
      try {
        // Simulate API delay
        await new Promise(resolve => setTimeout(resolve, 800));

        setMetrics({
          current_tier: 'Professional',
          monthly_ai_actions: 845,
          ai_actions_limit: 1000,
          storage_used_mb: 4200,
          storage_limit_mb: 5000,
          estimated_next_bill: 4900, // $49.00
          projected_cost: 5500, // $55.00
          budget_alert: true,
        });
      } catch (error) {
        console.error("Failed to fetch plan metrics:", error);
      } finally {
        setIsLoading(false);
      }
    };

    fetchMetrics();
  }, []);

  if (isLoading) {
    return (
      <div className="flex h-[50vh] items-center justify-center">
        <div className="animate-pulse space-y-4 text-center">
          <div className="h-12 w-12 rounded-full bg-slate-200 mx-auto"></div>
          <div className="text-slate-500">Loading your plan details...</div>
        </div>
      </div>
    );
  }

  return (
    <div className="max-w-4xl mx-auto space-y-8">
      <div>
        <h1 className="text-3xl font-semibold tracking-tight text-slate-900">My Plan</h1>
        <p className="mt-2 text-slate-600">
          Manage your subscription, view current usage, and estimate your next bill.
        </p>
      </div>

      {metrics?.budget_alert && (
        <div
          className="flex items-center gap-3 p-4 bg-amber-50 text-amber-900 rounded-lg border border-amber-200"
          role="alert"
        >
          <AlertCircle className="h-5 w-5 text-amber-600 flex-shrink-0" />
          <div className="text-sm">
            <span className="font-semibold">Budget health warning.</span> Your projected cost for this month ({formatCurrency(metrics.projected_cost)}) is nearing or exceeding your set threshold.
          </div>
        </div>
      )}

      <div className="grid gap-6 md:grid-cols-2">
        <Card className="border-slate-200 shadow-sm">
          <CardHeader>
            <CardTitle className="flex items-center justify-between">
              Current Plan
              <Badge variant="secondary" className="bg-blue-50 text-blue-700 hover:bg-blue-100">
                {metrics?.current_tier}
              </Badge>
            </CardTitle>
            <CardDescription>
              Your workspace is currently on the {metrics?.current_tier} tier.
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex items-center justify-between py-2 border-b border-slate-100">
              <div className="flex items-center gap-2 text-slate-600">
                <Zap className="h-4 w-4" />
                <span className="text-sm">AI Actions</span>
              </div>
              <div className="text-sm font-medium">
                {metrics?.monthly_ai_actions} / {metrics?.ai_actions_limit}
              </div>
            </div>
            <div className="flex items-center justify-between py-2 border-b border-slate-100">
              <div className="flex items-center gap-2 text-slate-600">
                <Database className="h-4 w-4" />
                <span className="text-sm">Storage Used</span>
              </div>
              <div className="text-sm font-medium">
                {(metrics?.storage_used_mb! / 1000).toFixed(1)} GB / {(metrics?.storage_limit_mb! / 1000).toFixed(1)} GB
              </div>
            </div>
            <div className="flex items-center justify-between py-2 border-b border-slate-100">
              <div className="flex items-center gap-2 text-slate-600">
                <CreditCard className="h-4 w-4" />
                <span className="text-sm">Estimated Next Bill</span>
              </div>
              <div className="text-sm font-medium">
                {formatCurrency(metrics?.estimated_next_bill || 0)}
              </div>
            </div>
          </CardContent>
          <CardFooter>
            <Button className="w-full bg-slate-900 hover:bg-slate-800 text-white">
              Upgrade Plan
            </Button>
          </CardFooter>
        </Card>

        <div className="space-y-6">
          <Card className="border-slate-200 shadow-sm bg-slate-50/50">
            <CardHeader>
              <CardTitle className="text-lg">Need more capacity?</CardTitle>
              <CardDescription>
                Upgrade to Enterprise for unlimited AI actions, 1TB storage, and dedicated support.
              </CardDescription>
            </CardHeader>
            <CardContent>
              <ul className="space-y-2 mb-6">
                <li className="flex items-start gap-2 text-sm text-slate-600">
                  <CheckCircle2 className="h-4 w-4 text-emerald-500 mt-0.5" />
                  Unlimited AI workspace actions
                </li>
                <li className="flex items-start gap-2 text-sm text-slate-600">
                  <CheckCircle2 className="h-4 w-4 text-emerald-500 mt-0.5" />
                  1TB secure media storage
                </li>
                <li className="flex items-start gap-2 text-sm text-slate-600">
                  <CheckCircle2 className="h-4 w-4 text-emerald-500 mt-0.5" />
                  Custom reporting dashboards
                </li>
              </ul>
              <Button variant="outline" className="w-full justify-between bg-white">
                View All Plans
                <ChevronRight className="h-4 w-4 text-slate-400" />
              </Button>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}
