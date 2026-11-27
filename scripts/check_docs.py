#!/usr/bin/env python3
"""Check the documentation set: required files exist and relative links resolve."""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

REQUIRED = [
    "README.md", "AGENTS.md", "LICENSE", "CONTRIBUTING.md", "CODE_OF_CONDUCT.md",
    "SECURITY.md", "CHANGELOG.md", "THIRD-PARTY.md", "SUPPORT.md",
    "docs/PRD.md", "docs/PLAN.md", "docs/TOOLING.md", "docs/VERIFICATION.md",
    "contract/README.md", ".github/CODEOWNERS", ".github/PULL_REQUEST_TEMPLATE.md",
    ".github/ISSUE_TEMPLATE/bug_report.md", ".github/ISSUE_TEMPLATE/feature_request.md",
]

PATTERNS = [
    "README.md", "AGENTS.md", "CONTRIBUTING.md", "CODE_OF_CONDUCT.md",
    "SECURITY.md", "CHANGELOG.md", "THIRD-PARTY.md", "SUPPORT.md",
    "docs/*.md", "contract/*.md", ".github/**/*.md",
]

FENCE = re.compile(r"```.*?```", re.S)
LINK = re.compile(r"\[[^\]]*\]\(([^)\s]+)")

failures = []
for rel in REQUIRED:
    if not (ROOT / rel).is_file():
        failures.append(f"missing required file: {rel}")

paths = {p for pattern in PATTERNS for p in ROOT.glob(pattern) if p.is_file()}
for path in sorted(paths):
    text = FENCE.sub("", path.read_text(encoding="utf-8"))
    for raw in LINK.findall(text):
        target = raw.split("#", 1)[0]
        if not target or target.startswith(("http://", "https://", "mailto:", "#", "/")):
            continue
        if not (path.parent / target).resolve().exists():
            failures.append(f"{path.relative_to(ROOT)}: broken link {raw}")

if failures:
    print("\n".join(failures))
    sys.exit(1)
print(f"docs ok: {len(paths)} files, relative links resolve")
