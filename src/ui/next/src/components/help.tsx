"use client";

import React, { createContext, useContext, useState, useEffect, useRef, ReactNode } from "react";
import DOMPurify from 'dompurify';
import { useRouter } from 'next/navigation';
import { WithTooltip } from './TooltipRegistry';
import { InteractiveWalkthrough, Step } from './Walkthrough';

// --- Walkthrough System ---

type HelpArticle = { title: string; desc: string; link?: string; category?: string };
type HelpVideo = { id: number; title: string; duration: string; video_url?: string; };
type HelpTab = "center" | "chat" | "videos" | "whatsnew";
type ChatMessage = { id: string; role: "bot" | "user"; text: string; linkUrl?: string; linkTitle?: string };

const helpTabs = [
  { id: "center", label: "Help" },
  { id: "chat", label: "Ask anything" },
  { id: "videos", label: "Videos" },
  { id: "whatsnew", label: "New" }
] as const;

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function isSafeLink(url: unknown): url is string {
  return typeof url === "string" && (url.startsWith("/") || url.startsWith("https://") || url.startsWith("http://"));
}

function normalizeArticles(data: unknown): HelpArticle[] {
  if (!Array.isArray(data)) return [];
  return data.flatMap((item) => {
    if (!isRecord(item) || typeof item.title !== "string" || typeof item.desc !== "string") return [];
    return [{ title: item.title, desc: item.desc, link: isSafeLink(item.link) ? item.link : undefined }];
  });
}

function normalizeVideos(data: unknown): HelpVideo[] {
  if (!Array.isArray(data)) return [];
  return data.flatMap((item) => {
    if (!isRecord(item) || typeof item.id !== "number" || typeof item.title !== "string" || typeof item.duration !== "string") return [];
    return [{ id: item.id, title: item.title, duration: item.duration }];
  });
}

function normalizeChatReply(data: unknown): Omit<ChatMessage, "id" | "role"> {
  if (!isRecord(data) || typeof data.reply !== "string" || !data.reply.trim()) {
    throw new Error("Invalid chat reply");
  }

  const link = isRecord(data.link) ? data.link : undefined;
  return {
    text: data.reply,
    linkUrl: isSafeLink(link?.url) ? link.url : undefined,
    linkTitle: typeof link?.title === "string" && link.title.trim() ? link.title : undefined
  };
}

type WalkthroughContextType = {
  startWalkthrough: (steps: Step[]) => void;
  nextStep: () => void;
  endWalkthrough: () => void;
};

const WalkthroughContext = createContext<WalkthroughContextType | undefined>(undefined);

export function WalkthroughProvider({ children }: { children: ReactNode }) {
  const [steps, setSteps] = useState<Step[]>([]);
  const [currentStepIndex, setCurrentStepIndex] = useState(-1);

  const startWalkthrough = (newSteps: Step[]) => {
    setSteps(newSteps);
    setCurrentStepIndex(0);
  };

  const nextStep = () => {
    if (currentStepIndex < steps.length - 1) {
      setCurrentStepIndex(prev => prev + 1);
    } else {
      endWalkthrough();
    }
  };

  const endWalkthrough = () => {
    setSteps([]);
    setCurrentStepIndex(-1);
  };

  useEffect(() => {
    if (currentStepIndex >= 0 && currentStepIndex < steps.length) {
      const step = steps[currentStepIndex];
      const el = document.getElementById(step.targetId);
      if (el) {
        el.scrollIntoView({ behavior: "smooth", block: "center" });
      }
    }
  }, [currentStepIndex, steps]);

  const activeStep = currentStepIndex >= 0 ? steps[currentStepIndex] : null;

  const [highlightStyle, setHighlightStyle] = useState({});

  useEffect(() => {
    if (activeStep) {
      const el = document.getElementById(activeStep.targetId);
      if (el) {
        const rect = el.getBoundingClientRect();
        setHighlightStyle({
          top: rect.top - 8,
          left: rect.left - 8,
          width: rect.width + 16,
          height: rect.height + 16,
        });
      }
    }
  }, [activeStep]);

  return (
    <WalkthroughContext.Provider value={{ startWalkthrough, nextStep, endWalkthrough }}>
      {children}
      {steps.length > 0 && (
        <InteractiveWalkthrough
          steps={steps.map(s => ({ targetId: s.targetId, title: s.title, content: s.content, position: "top" }))}
          isOpen={steps.length > 0}
          onClose={endWalkthrough}
          onComplete={endWalkthrough}
        />
      )}
    </WalkthroughContext.Provider>
  );
}

export function useWalkthrough() {
  const context = useContext(WalkthroughContext);
  if (!context) throw new Error("useWalkthrough must be used within WalkthroughProvider");
  return context;
}

// --- Help Widget System ---
export function HelpWidget() {
  return null;
}
