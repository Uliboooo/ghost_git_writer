export type SemVerPart = "major" | "minor" | "patch";

const PARTS: { key: SemVerPart; label: string }[] = [
  { key: "major", label: "MAJOR" },
  { key: "minor", label: "MINOR" },
  { key: "patch", label: "PATCH" },
];

export function parseSemVerPart(s: string): SemVerPart | null {
  const lower = s.trim().toLowerCase();
  if (lower === "major" || lower === "minor" || lower === "patch") {
    return lower as SemVerPart;
  }
  return null;
}

export function semVerSelector(selected: SemVerPart): string {
  let top = "";
  let mid = "";
  let bot = "";

  for (let i = 0; i < PARTS.length; i++) {
    const { label } = PARTS[i];
    const w = label.length;
    const isSel = PARTS[i].key === selected;
    const prevIsSel = i > 0 && PARTS[i - 1].key === selected;

    // Add separator before this item (skip if adjacent to selected)
    if (i > 0 && !prevIsSel && !isSel) {
      top += " ";
      mid += "│";
      bot += " ";
    }

    if (isSel) {
      top += `╭${"─".repeat(w + 2)}╮`;
      mid += `│ ${label} │`;
      bot += `╰${"─".repeat(w + 2)}╯`;
    } else {
      top += " ".repeat(w + 2);
      mid += ` ${label} `;
      bot += " ".repeat(w + 2);
    }
  }

  return `${top}\n${mid}\n${bot}`;
}
