import { useEffect, useState } from "react";
import { docker } from "@/lib/tauri";
import type { ContainerSummary } from "@/types/docker";

const STATE_COLORS: Record<string, string> = {
  running: "bg-green-500",
  exited: "bg-red-500",
  paused: "bg-yellow-500",
  restarting: "bg-blue-500",
};

const STATE_TEXT_COLORS: Record<string, string> = {
  running: "text-green-500",
  exited: "text-red-500",
  paused: "text-yellow-500",
  restarting: "text-blue-500",
};

export default function App() {
  const [containers, setContainers] = useState<ContainerSummary[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    docker
      .listContainers()
      .then(setContainers)
      .catch((e: unknown) => setError(String(e)))
      .finally(() => setLoading(false));
  }, []);

  return (
    <div className="w-screen h-screen bg-[#080B14] text-[#E8EDF8] font-['DM_Sans',system-ui,sans-serif] p-6 overflow-auto">
      <div className="flex items-center gap-3 mb-6 pb-4 border-b border-[#1E2535]">
        <span className="text-[28px]">🐳</span>
        <span className="text-lg font-bold">DockerLens</span>
        <span className="ml-auto bg-[#1E2535] px-2.5 py-0.5 rounded-[10px] text-xs text-[#8B96B0]">
          {containers.length} containers
        </span>
      </div>

      {loading && <p className="text-[#4A5568] text-[13px]">Connecting to Docker…</p>}

      {error && (
        <div className="bg-[#3D1515] border border-[#F0525230] rounded-[9px] p-3.5 text-[#F05252] text-[13px] mb-4">
          <strong>Docker connection failed</strong>
          <p>{error}</p>
        </div>
      )}

      {!loading && !error && containers.length === 0 && (
        <p className="text-[#4A5568] text-[13px]">No containers found.</p>
      )}

      <div className="flex flex-col gap-2">
        {containers.map((c) => (
          <div
            key={c.id}
            className="flex items-center gap-3 p-3 bg-[#0E1220] rounded-[9px] border border-[#1E2535]"
          >
            <span
              className={`w-2 h-2 rounded-full flex-shrink-0 ${
                STATE_COLORS[c.state] ?? "bg-gray-600"
              }`}
            />
            <span className="font-semibold text-[13px] flex-[0_0_180px]">{c.name}</span>
            <span className="text-[#4A5568] text-[11px] font-mono flex-1">{c.image}</span>
            <span className={`text-xs ${STATE_TEXT_COLORS[c.state] ?? "text-[#8B96B0]"}`}>
              {c.status}
            </span>
          </div>
        ))}
      </div>
    </div>
  );
}