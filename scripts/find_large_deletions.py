#!/usr/bin/env python3
"""Find large file deletions in recent git history and optionally restore them."""

from __future__ import annotations

import argparse
import subprocess
from dataclasses import dataclass
from pathlib import Path


@dataclass
class DeletedFile:
    commit: str
    subject: str
    lines: int
    path: str
    scan_index: int
    commit_deleted_lines: int


def git(args: list[str], *, text: bool = True) -> str | bytes:
    result = subprocess.run(
        ["git", *args],
        check=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=text,
    )
    return result.stdout


def recent_commits(limit: int | None, since: str | None) -> list[str]:
    args = ["rev-list", "HEAD"]
    if limit is not None:
        args.insert(1, f"--max-count={limit}")
    if since is not None:
        args.insert(1, f"--since={since}")
    output = git(args)
    assert isinstance(output, str)
    return [line for line in output.splitlines() if line]


def subject_for(commit: str) -> str:
    output = git(["show", "-s", "--format=%s", commit])
    assert isinstance(output, str)
    return output.strip()


def deleted_files_for(
    commit: str,
    min_file_lines: int,
    min_commit_deleted_lines: int,
    scan_index: int,
) -> list[DeletedFile]:
    subject = subject_for(commit)
    output = git(["diff-tree", "-r", "--numstat", "--diff-filter=D", f"{commit}^", commit])
    assert isinstance(output, str)
    all_deleted: list[tuple[int, str]] = []
    for line in output.splitlines():
        parts = line.split("\t", 2)
        if len(parts) != 3:
            continue
        _added, removed, path = parts
        if removed == "-":
            lines = 0
        else:
            lines = int(removed)
        all_deleted.append((lines, path))

    commit_deleted_lines = sum(lines for lines, _path in all_deleted)
    deleted: list[DeletedFile] = []
    for lines, path in all_deleted:
        if lines >= min_file_lines or commit_deleted_lines >= min_commit_deleted_lines:
            deleted.append(
                DeletedFile(
                    commit=commit,
                    subject=subject,
                    lines=lines,
                    path=path,
                    scan_index=scan_index,
                    commit_deleted_lines=commit_deleted_lines,
                )
            )
    return deleted


def all_deleted_files_for(commit: str, scan_index: int) -> list[DeletedFile]:
    subject = subject_for(commit)
    output = git(["diff-tree", "-r", "--numstat", "--diff-filter=D", f"{commit}^", commit])
    assert isinstance(output, str)
    rows: list[tuple[int, str]] = []
    for line in output.splitlines():
        parts = line.split("\t", 2)
        if len(parts) != 3:
            continue
        _added, removed, path = parts
        lines = 0 if removed == "-" else int(removed)
        rows.append((lines, path))
    commit_deleted_lines = sum(lines for lines, _path in rows)

    deleted: list[DeletedFile] = []
    for lines, path in rows:
        deleted.append(
            DeletedFile(
                commit=commit,
                subject=subject,
                lines=lines,
                path=path,
                scan_index=scan_index,
                commit_deleted_lines=commit_deleted_lines,
            )
        )
    return deleted


def restore_deleted_file(item: DeletedFile, repo_root: Path, dry_run: bool) -> str:
    target = repo_root / item.path
    if target.exists():
        return "exists"
    if dry_run:
        return "would_restore"

    target.parent.mkdir(parents=True, exist_ok=True)
    content = git(["show", f"{item.commit}^:{item.path}"], text=False)
    assert isinstance(content, bytes)
    target.write_bytes(content)
    return "restored"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--limit", type=int, default=None)
    parser.add_argument("--since", default=None)
    parser.add_argument(
        "--min-lines",
        type=int,
        default=50,
        help="Match a commit when any deleted file has at least this many deleted lines.",
    )
    parser.add_argument(
        "--min-commit-deleted-lines",
        type=int,
        default=500,
        help="Match a commit when total deleted-file lines in that commit reach this threshold.",
    )
    parser.add_argument("--restore", action="store_true")
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument(
        "--all-deletions-in-matching-commits",
        action="store_true",
        help="After a commit matches --min-lines, include every deleted file from that commit.",
    )
    parser.add_argument(
        "--latest-per-path",
        action=argparse.BooleanOptionalAction,
        default=True,
        help="When restoring, only use the most recent deletion for each path.",
    )
    args = parser.parse_args()

    repo_root_output = git(["rev-parse", "--show-toplevel"])
    assert isinstance(repo_root_output, str)
    repo_root = Path(repo_root_output.strip())

    items: list[DeletedFile] = []
    for scan_index, commit in enumerate(recent_commits(args.limit, args.since)):
        try:
            matching_items = deleted_files_for(
                commit,
                args.min_lines,
                args.min_commit_deleted_lines,
                scan_index,
            )
            if args.all_deletions_in_matching_commits and matching_items:
                items.extend(all_deleted_files_for(commit, scan_index))
            else:
                items.extend(matching_items)
        except subprocess.CalledProcessError:
            continue

    if args.restore and args.latest_per_path:
        latest_by_path: dict[str, DeletedFile] = {}
        for item in items:
            if item.path not in latest_by_path:
                latest_by_path[item.path] = item
        items = list(latest_by_path.values())

    items.sort(key=lambda item: (item.lines, item.commit, item.path), reverse=True)

    current_commit = ""
    for item in items:
        if item.commit != current_commit:
            current_commit = item.commit
            commit_total = item.commit_deleted_lines
            print(f"\n{item.commit[:9]} deleted_lines={commit_total} {item.subject}")
        status = ""
        if args.restore:
            status = f" [{restore_deleted_file(item, repo_root, args.dry_run)}]"
        print(f"  {item.lines:5d}  {item.path}{status}")

    print(
        "\nFound "
        f"{len(items)} deleted files matching file >= {args.min_lines} lines "
        f"or commit >= {args.min_commit_deleted_lines} deleted-file lines."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
