export type WorkItem = {
  id: string;
  scope: "/src" | "/tasks";
  owner: "devin" | "codex";
};

export function describeWorkItem(item: WorkItem): string {
  return `${item.owner} owns ${item.scope} for ${item.id}`;
}
