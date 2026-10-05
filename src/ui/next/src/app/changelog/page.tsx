"use client";

import React,{ useEffect,useState } from "react";
import { motion } from "framer-motion";

type ChangelogSection = {
  version: string;
  contentLines: string[];
  screenshot_url?: string;
};

type ChangelogState =
  | { status: "loading" }
  | { status: "error"; message: string }
  | { status: "loaded"; sections: ChangelogSection[] };

function isChangelog(value: unknown): value is ChangelogSection[] {
  return Array.isArray(value) && value.every((section) =>
    section !== null && typeof section === "object" &&
    typeof section.version === "string" &&
    Array.isArray(section.contentLines) &&
    section.contentLines.every((line: unknown) => typeof line === "string") &&
    (section.screenshot_url === undefined || typeof section.screenshot_url === "string")
  );
}

function parseLinks(text: string): React.ReactNode {
  const linkRegex = /\[(.*?)\]\((.*?)\)/g;
  let match;
  const parts: React.ReactNode[] = [];
  let lastIndex = 0;
  let key = 0;

  while ((match = linkRegex.exec(text)) !== null) {
    if (match.index > lastIndex) {
      parts.push(<span key={key++}>{text.substring(lastIndex, match.index)}</span>);
    }
    parts.push(
      <a key={key++} href={match[2]} target="_blank" rel="noopener noreferrer" className="text-blue-600 hover:underline">
        {match[1]}
      </a>
    );
    lastIndex = linkRegex.lastIndex;
  }

  if (lastIndex < text.length) {
    parts.push(<span key={key}>{text.substring(lastIndex)}</span>);
  }

  return parts.length > 0 ? <>{parts}</> : text;
}

export default function ChangelogPage() {
  const [state, setState] = useState<ChangelogState>({ status: "loading" });
  const [attempt, setAttempt] = useState(0);

  useEffect(() => {
    let active = true;
    setState({ status: "loading" });
    fetch("/api/v1/changelog")
      .then(async (res) => {
        if (!res.ok) {
          const problem = await res.json().catch(() => null);
          const detail = typeof problem?.error === "string" ? `: ${problem.error}` : "";
          throw new Error(`HTTP ${res.status}${detail}`);
        }
        return res.json();
      })
      .then((data) => {
        if (!isChangelog(data)) throw new Error("The server returned an invalid changelog response.");
        if (active) setState({ status: "loaded", sections: data });
      })
      .catch((error: unknown) => {
        if (active) setState({
          status: "error",
          message: error instanceof Error ? error.message : "The request failed.",
        });
      });
    return () => { active = false; };
  }, [attempt]);

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-black py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-3xl mx-auto">
        <h2 data-testid="changelog-title" className="text-3xl sm:text-4xl font-extrabold font-outfit text-gray-900 dark:text-gray-100 mb-8 text-center tracking-tight">
          Changelog Updates
        </h2>
        <div className="space-y-8">
          {state.status === "loading" ? (
            <div role="status" className="flex justify-center items-center gap-3 py-12">
              <div aria-hidden="true" className="animate-spin rounded-full h-8 w-8 border-b-2 border-[#0071E3]"></div>
              <span>Loading changelog…</span>
            </div>
          ) : state.status === "error" ? (
            <div className="rounded-3xl border border-red-200 bg-red-50 p-6 text-red-900">
              <p role="alert">Unable to load changelog. {state.message}</p>
              <button type="button" onClick={() => setAttempt((current) => current + 1)} className="mt-4 min-h-[44px] rounded-xl border border-red-300 px-5 py-2 font-semibold focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2">
                Retry
              </button>
            </div>
          ) : state.sections.length === 0 ? (
            <p className="text-center text-gray-500 font-medium py-8 backdrop-blur-[40px] saturate-[210%] bg-white/70 dark:bg-[#1C1C1E]/70 border border-white/40 dark:border-white/10 shadow-[0_4px_24px_rgba(0,0,0,0.04)] rounded-3xl">
              No changelog available.
            </p>
          ) : (
            state.sections.map((section, idx) => (
              <motion.div
                key={idx}
                data-testid="changelog-section"
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: idx * 0.1, duration: 0.4 }}
                className="backdrop-blur-[40px] saturate-[210%] bg-white/70 dark:bg-[#1C1C1E]/70 border border-white/40 dark:border-white/10 shadow-[0_4px_24px_rgba(0,0,0,0.04)] p-6 sm:p-8 rounded-3xl transition-all hover:-translate-y-1 hover:shadow-[0_4px_24px_rgba(0,0,0,0.04)] hover:border-blue-300 dark:hover:border-blue-700"
              >
                <h2 className="text-xl sm:text-2xl font-bold text-[#0071E3] dark:text-blue-400 mb-4 font-outfit">
                  {section.version}
                </h2>
                <div className="space-y-3">
                  {section.contentLines.map((line, lidx) => {
                    if (line.startsWith("### ")) {
                      return (
                        <h3
                          key={lidx}
                          className="text-lg font-semibold text-gray-800 dark:text-gray-200 mt-6 mb-2 font-outfit tracking-tight"
                        >
                          {line.replace("### ", "")}
                        </h3>
                      );
                    }
                    if (line.startsWith("- ")) {
                      return (
                        <li key={lidx} className="text-gray-600 dark:text-gray-300 ml-5 list-disc pl-1 marker:text-[#0066FF]">
                          {parseLinks(line.replace("- ", ""))}
                        </li>
                      );
                    }
                    return (
                      <p key={lidx} className="text-gray-600 dark:text-gray-300 leading-relaxed">
                        {parseLinks(line)}
                      </p>
                    );
                  })}
                </div>
                {section.screenshot_url && (
                  <img
                    src={section.screenshot_url}
                    alt={`${section.version} Screenshot`}
                    loading="lazy"
                    className="rounded-2xl mt-6 w-full shadow-lg border border-gray-200/50 dark:border-gray-700/50 object-cover"
                  />
                )}
              </motion.div>
            ))
          )}

          <div className="mt-10 text-center">
            <a
              href="https://cloud.omnisolo.co/changelog"
              target="_blank"
              rel="noopener noreferrer"
              className="text-[#0071E3] dark:text-blue-400 font-bold hover:text-blue-700 dark:hover:text-blue-300 bg-blue-50/80 dark:bg-blue-900/20 px-8 py-4 rounded-full border border-blue-100 dark:border-blue-800/50 inline-block shadow-sm backdrop-blur-xl saturate-[210%] transition-all hover:shadow-md hover:-translate-y-0.5"
            >
              Read the full technical changelog on our website →
            </a>
          </div>
        </div>
      </div>
    </div>
  );
}
