export type ModelWindowRow = {
  model: string;
  window: string;
};

export function modelWindowRowsFromProfile(modelList: string, modelWindows: string): ModelWindowRow[] {
  let map: Record<string, string> = {};
  try {
    map = JSON.parse(modelWindows || "{}") as Record<string, string>;
  } catch {
    map = {};
  }
  const rows = modelList
    .split("\n")
    .map((model) => model.trim())
    .filter(Boolean)
    .map((model) => ({ model, window: map[model] ?? "" }));
  return rows.length ? rows : [{ model: "", window: "" }];
}

export function serializeModelWindowRows(rows: ModelWindowRow[]): { modelList: string; modelWindows: string } {
  const modelList: string[] = [];
  const modelWindows: Record<string, string> = {};
  rows.forEach((row) => {
    const model = row.model.trim();
    if (!model) return;
    modelList.push(model);
    const window = row.window.trim();
    if (window) modelWindows[model] = window;
  });
  return {
    modelList: modelList.join("\n"),
    modelWindows: JSON.stringify(modelWindows),
  };
}
