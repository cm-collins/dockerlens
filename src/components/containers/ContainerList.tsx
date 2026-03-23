import { useState } from "react";
import { useContainers } from "@/hooks/useContainers";
import { useContainersStore } from "@/store/containers.store";
import type { ContainerSummary } from "@/types/docker";

export function ContainerList() {
    const [search, setSearch] = useState("");
    const { data: containers = [], isLoading, isError } = useContainers();
    const { selectedId, setSelectedId } = useContainersStore();

    const filtered = containers.filter(
        (c) =>
            c.name.toLowerCase().includes(search.toLowerCase()) ||
            c.image.toLowerCase().includes(search.toLowerCase())
    );

    return (
        <div
            className="w-[320px] flex flex-col flex-shrink-0"
            style={{ borderRight: "1px solid var(--border)", background: "var(--bg)" }}
        >
            {/* Search */}
            <div className="p-3" style={{ borderBottom: "1px solid var(--border)" }}>
                <input
                    value={search}
                    onChange={(e) => setSearch(e.target.value)}
                    placeholder="Search containers…"
                    className="w-full px-3 py-2 rounded-[7px] text-[13px] outline-none transition-all"
                    style={{
                        background: "var(--surface)",
                        border: "1px solid var(--border)",
                        color: "var(--text-primary)",
                        fontFamily: "inherit",
                    }}
                    onFocus={(e) => (e.target.style.borderColor = "var(--blue)")}
                    onBlur={(e) => (e.target.style.borderColor = "var(--border)")}
                />
            </div>

            {/* List */}
            <div className="flex-1 overflow-auto">
                {isLoading && (
                    <div className="p-4 text-[13px]" style={{ color: "var(--text-muted)" }}>
                        Connecting to Docker…
                    </div>
                )}

                {isError && (
                    <div className="p-4 text-[13px]" style={{ color: "var(--red)" }}>
                        Failed to connect to Docker
                    </div>
                )}

                {!isLoading && filtered.length === 0 && (
                    <div className="p-4 text-[13px]" style={{ color: "var(--text-muted)" }}>
                        {search ? "No containers match your search" : "No containers found"}
                    </div>
                )}

                {filtered.map((c) => (
                    <ContainerRow
                        key={c.id}
                        container={c}
                        isSelected={selectedId === c.id}
                        onSelect={() => setSelectedId(c.id)}
                    />
                ))}
            </div>
        </div>
    );
}

function ContainerRow({ container, isSelected, onSelect }: {
    container: ContainerSummary;
    isSelected: boolean;
    onSelect: () => void;
}) {
    const isRunning = container.state.toLowerCase() === "running";

    return (
        <button
            onClick={onSelect}
            className="w-full text-left px-4 py-3 cursor-pointer transition-all"
            style={{
                background: isSelected ? "var(--blue-glow)" : "transparent",
                borderBottom: "1px solid var(--border)",
                borderLeft: `3px solid ${isSelected ? "var(--blue)" : "transparent"}`,
                fontFamily: "inherit",
            }}
        >
            {/* Name row */}
            <div className="flex items-center gap-2 mb-1">
                <StatusDot state={container.state} />
                <span className="text-[13px] font-semibold flex-1 truncate" style={{ color: "var(--text-primary)" }}>
                    {container.name}
                </span>
                <StateBadge state={container.state} />
            </div>

            {/* Image */}
            <div className="mono text-[11px] truncate pl-[15px]" style={{ color: "var(--text-muted)" }}>
                {container.image}
            </div>

            {/* Status for running containers */}
            {isRunning && (
                <div className="flex gap-4 mt-1 pl-[15px]">
                    <span className="text-[10px]" style={{ color: "var(--text-muted)" }}>
                        {container.status}
                    </span>
                </div>
            )}
        </button>
    );
}

function StatusDot({ state }: { state: string }) {
    const colors: Record<string, string> = {
        running: "var(--green)",
        exited: "var(--red)",
        paused: "var(--yellow)",
    };

    const color = colors[state.toLowerCase()] || "var(--text-muted)";
    const isRunning = state.toLowerCase() === "running";

    return (
        <span
            className="w-[8px] h-[8px] rounded-full flex-shrink-0"
            style={{
                background: color,
                animation: isRunning ? "pulse-dot 2s infinite" : "none",
            }}
        />
    );
}

function StateBadge({ state }: { state: string }) {
    const styles: Record<string, { bg: string; color: string }> = {
        running: { bg: "var(--green-dim)", color: "var(--green)" },
        exited: { bg: "var(--red-dim)", color: "var(--red)" },
        paused: { bg: "var(--yellow-dim)", color: "var(--yellow)" },
    };

    const style = styles[state.toLowerCase()] || { bg: "var(--border)", color: "var(--text-muted)" };

    return (
        <span
            className="text-[10px] font-semibold px-2 py-[2px] rounded-[10px] uppercase"
            style={style}
        >
            {state}
        </span>
    );
}
