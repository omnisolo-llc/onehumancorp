#!/usr/bin/env python3
"""Summarize this exact GitHub run attempt; cache labels are requests, not hits.

The first required-result step still owns correctness. This additional gate
measures elapsed CI wall time, including waits between dependent jobs. It never
substitutes a timeout value or a warm local invocation for actual CI evidence.
"""
from __future__ import annotations

import argparse
from datetime import datetime
import json
import math
from pathlib import Path
import sys
from typing import Any


def timestamp(value: Any) -> float:
    if not isinstance(value, str):
        raise ValueError("Missing job timestamp")
    try:
        parsed = datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError as error:
        raise ValueError("Invalid job timestamp") from error
    if parsed.tzinfo is None:
        raise ValueError("Job timestamps must include a timezone")
    return parsed.timestamp()


CORE_BUILD_JOBS = frozenset({
    "Native backend binaries", "Native Next production build", "Native Tauri compile",
})


def core_build_timing(rows: list[dict[str, Any]], first_start: float) -> dict[str, Any]:
    """Diagnostic build target, never a replacement for the full required gate."""
    selected = [row for row in rows if row["name"] in CORE_BUILD_JOBS]
    names = [row["name"] for row in selected]
    if len(names) != len(set(names)):
        raise ValueError("Duplicate core-build job names make timing ambiguous")
    missing = sorted(CORE_BUILD_JOBS - set(names))
    incomplete = missing or any(row["seconds"] is None for row in selected)
    elapsed = None if incomplete else max(timestamp(row["completed_at"]) for row in selected) - first_start
    return {
        "target_seconds": 600, "enforced": False,
        "elapsed_seconds": elapsed,
        "within_target": None if elapsed is None else elapsed <= 600,
        "all_builds_succeeded": not incomplete and all(row["conclusion"] == "success" for row in selected),
        "missing_jobs": missing,
    }


def summarize(pages: Any, run_id: int, attempt: int, budget: float, cold: bool,
              job_prefix: str = '') -> dict[str, Any]:
    if not math.isfinite(budget) or budget <= 0 or run_id <= 0 or attempt <= 0:
        raise ValueError("Positive run, attempt and budget are required")
    if not isinstance(pages, list) or not pages:
        raise ValueError("Expected all paginated job responses")
    # GitHub's documented example omits run_attempt on job objects. The caller
    # must fetch /runs/{run}/attempts/{attempt}/jobs, never the mixed-run endpoint.
    # Reject an explicit conflicting attempt when the API supplies that field.
    jobs: dict[int, dict[str, Any]] = {}
    totals: set[int] = set()
    for page in pages:
        if not isinstance(page, dict) or not isinstance(page.get("jobs"), list):
            raise ValueError("Malformed jobs response")
        total = page.get("total_count")
        if not isinstance(total, int) or total <= 0:
            raise ValueError("Missing total job count")
        totals.add(total)
        for job in page["jobs"]:
            if (not isinstance(job, dict) or not isinstance(job.get("id"), int)
                    or job.get("run_id") != run_id
                    or ("run_attempt" in job and job["run_attempt"] != attempt)):
                raise ValueError("Job does not belong to the requested run attempt")
            if job["id"] in jobs:
                raise ValueError("Duplicate job across result pages")
            jobs[job["id"]] = job
    if totals != {len(jobs)}:
        raise ValueError("Incomplete or inconsistent job pagination")
    # A reusable CI invocation shares the run with release packaging jobs.
    # Validate pagination first, then isolate the explicitly named CI invocation.
    # Never count those concurrently running release jobs as CI predecessors.
    if job_prefix:
        jobs = {key: {**job, 'name': job['name'][len(job_prefix):]}
                for key, job in jobs.items() if job.get('name', '').startswith(job_prefix)}
    own = [job for job in jobs.values() if job.get("name") == "CI Required"]
    if len(own) != 1:
        raise ValueError("Exactly one CI Required job is necessary")
    selected = [job for job in jobs.values() if job["id"] != own[0]["id"]]
    if not selected:
        raise ValueError("No preceding CI jobs were reported")
    rows = []
    for job in selected:
        if job.get("status") != "completed" or not job.get("conclusion"):
            raise ValueError("A preceding CI job has no final result")
        if job["conclusion"] == "skipped":
            rows.append({"name": job["name"], "conclusion": "skipped", "seconds": None})
            continue
        start, end = timestamp(job.get("started_at")), timestamp(job.get("completed_at"))
        if end < start:
            raise ValueError("Job completion precedes its start")
        rows.append({"name": job["name"], "conclusion": job["conclusion"],
                     "seconds": end - start, "started_at": job["started_at"],
                     "completed_at": job["completed_at"]})
    measured = [row for row in rows if row["seconds"] is not None]
    if not measured:
        raise ValueError("No executed jobs have timing evidence")
    first = min(timestamp(row["started_at"]) for row in measured)
    # Include scheduling delay before the final gate; exclude the gate's own
    # reporting/upload time and queue time before the first runner started.
    final_start = timestamp(own[0].get("started_at"))
    last = max(timestamp(row["completed_at"]) for row in measured)
    if final_start < last:
        raise ValueError("Final gate started before its predecessors completed")
    elapsed = final_start - first
    return {"schema_version": 1, "run_id": run_id, "run_attempt": attempt,
            "cache_mode_requested": "disabled" if cold else "enabled",
            "cache_hit_proven": False, "elapsed_seconds": elapsed,
            "core_build": core_build_timing(rows, first),
            "budget_seconds": budget * 60, "within_budget": elapsed <= budget * 60,
            "all_executed_jobs_succeeded": all(row["conclusion"] == "success" for row in measured),
            "exclusions": ["queue before first job", "final reporting/upload step"],
            "jobs": sorted(rows, key=lambda row: -(row["seconds"] or 0))}


def render(report: dict[str, Any]) -> str:
    def cell(value: str) -> str:
        return value.replace("|", "\\|").replace("\n", " ").replace("\r", " ")
    lines = ["## Native CI performance", "",
             f"Run {report['run_id']}, attempt {report['run_attempt']}; cache requested: {report['cache_mode_requested']} (not a cache-hit claim).",
             f"Elapsed to final gate: **{report['elapsed_seconds'] / 60:.2f} minutes**; budget: **{report['budget_seconds'] / 60:g} minutes**.",
             "Includes dependency/scheduling waits after the first job starts; excludes initial queue and final reporting/upload.", "",
             ]
    core = report["core_build"]
    if core["elapsed_seconds"] is None:
        lines.append("Core-build timing: **not measured** (missing or skipped backend, Next or Tauri build).")
    else:
        outcome = "passed" if core["all_builds_succeeded"] else "failed"
        lines.append(f"Core builds: **{core['elapsed_seconds'] / 60:.2f} minutes**, {outcome}; provisional target: **10 minutes** (diagnostic, not a replacement gate).")
    lines.extend(["", "| Job | Result | Execution minutes |", "|---|---|---:|"])
    for job in report["jobs"]:
        minutes = "—" if job["seconds"] is None else f"{job['seconds'] / 60:.2f}"
        lines.append(f"| {cell(job['name'])} | {cell(job['conclusion'])} | {minutes} |")
    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--jobs", type=Path, required=True)
    parser.add_argument("--run-id", type=int, required=True)
    parser.add_argument("--attempt", type=int, required=True)
    parser.add_argument("--budget-minutes", type=float, default=30)
    parser.add_argument("--cold", action="store_true")
    parser.add_argument("--job-prefix", default='')
    parser.add_argument("--output", type=Path, required=True)
    options = parser.parse_args()
    try:
        report = summarize(json.loads(options.jobs.read_text()), options.run_id,
                           options.attempt, options.budget_minutes, options.cold, options.job_prefix)
        options.output.mkdir(parents=True, exist_ok=True)
        (options.output / "performance.json").write_text(json.dumps(report, indent=2) + "\n")
        (options.output / "performance.md").write_text(render(report))
        print(render(report))
    except (ValueError, KeyError, TypeError, OSError) as error:
        print(f"CI performance evidence unavailable: {error}", file=sys.stderr)
        return 1
    return 0 if report["within_budget"] and report["all_executed_jobs_succeeded"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
