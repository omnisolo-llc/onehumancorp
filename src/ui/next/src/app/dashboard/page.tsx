"use client";

import { useState, useEffect } from "react";
import Link from "next/link";
import { WithTooltip } from "../../components/TooltipRegistry";

export default function Dashboard() {
  const [approvals, setApprovals] = useState<any[]>([]);
  const [isOffline, setIsOffline] = useState(false);
  const [offlineQueueCount, setOfflineQueueCount] = useState(0);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [isSendingCampaign, setIsSendingCampaign] = useState(false);
  const [campaignSuccess, setCampaignSuccess] = useState(false);

  useEffect(() => {
    const fetchOffers = async () => {
      try {
        const merchantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store';
        const res = await fetch(`/api/v1/capital/offers/${merchantId}`);
        if (res.ok) {
          const data = await res.json();
          if (data && data.length > 0) {
            setCapitalOffer(data[0]);
          }
        }
      } catch (e) {}
    };
    fetchOffers();
  }, []);

  useEffect(() => {
