import { ExternalLink, X } from "lucide-react";
import { useContainerDetail } from "@/hooks/useContainers";
import { useContainersStore } from "@/store/containers.store";

function DetailSection({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) {
  return (
    <section className="space-y-3">
      <h3 className="text-[11px] font-semibold uppercase tracking-[0.22em] text-[rgb(var(--workspace-muted))]">
        {title}
      </h3>
      {children}
    </section>
  );
}

function InlineTag({ children }: { children: React.ReactNode }) {
  return (
    <span className="inline-flex items-center rounded-full border border-[rgb(var(--workspace-border))] bg-[rgb(var(--workspace-card))] px-3 py-1 text-[12px] text-[rgb(var(--workspace-soft))]">
      {children}
    </span>
  );
}

export function ContainerDetail() {
  const { selectedId, setSelectedId } = useContainersStore();
  const { data, isLoading } = useContainerDetail(selectedId);

  if (!selectedId) {
    return null;
  }

  return (
    <aside className="border-t border-[rgb(var(--workspace-border))] bg-[rgb(var(--panel-bg))] px-4 py-5 sm:px-6 lg:px-10 lg:py-6">
      <div className="mb-5 flex flex-wrap items-start justify-between gap-4">
        <div>
          <div className="flex items-center gap-3">
            <h2 className="text-[24px] font-semibold tracking-[-0.04em] text-[rgb(var(--workspace-foreground))]">
              {data?.name ?? "Loading container"}
            </h2>
            {data?.platform.architecture ? (
              <InlineTag>{data.platform.architecture.toUpperCase()}</InlineTag>
            ) : null}
          </div>

          <p className="mt-2 max-w-3xl text-[14px] text-[rgb(var(--workspace-muted))]">
            {isLoading
              ? "Loading container details..."
              : data
                ? `${data.image} • ${data.status}`
                : "Unable to load selected container details."}
          </p>
        </div>

        <button
          type="button"
          onClick={() => setSelectedId(null)}
          className="flex h-10 w-10 items-center justify-center rounded-2xl border border-[rgb(var(--workspace-border))] bg-[rgb(var(--workspace-card))] text-[rgb(var(--workspace-soft))]"
          title="Close details"
        >
          <X className="h-4 w-4" />
        </button>
      </div>

      {data ? (
        <div className="grid grid-cols-1 gap-6 md:grid-cols-2 xl:grid-cols-4 xl:gap-8">
          <DetailSection title="Image">
            <div className="space-y-2 text-[14px] text-[rgb(var(--workspace-soft))]">
              <div>{data.image}</div>
              <div className="font-mono text-[12px] text-[rgb(var(--workspace-muted))]">
                {data.image_id}
              </div>
            </div>
          </DetailSection>

          <DetailSection title="Ports">
            <div className="flex flex-wrap gap-2">
              {data.ports.length ? (
                data.ports.map((port) => (
                  <a
                    key={`${port.host_port}-${port.container_port}-${port.protocol}`}
                    href={port.browser_url ?? "#"}
                    target="_blank"
                    rel="noreferrer"
                    className="inline-flex items-center gap-2 rounded-full border border-[rgb(var(--link))/0.24] bg-[rgb(var(--workspace-card))] px-3 py-1 text-[12px] text-[rgb(var(--link))]"
                  >
                    {port.host_port || port.container_port}/{port.protocol}
                    {port.browser_url ? <ExternalLink className="h-3.5 w-3.5" /> : null}
                  </a>
                ))
              ) : (
                <span className="text-[14px] text-[rgb(var(--workspace-muted))]">
                  No public ports
                </span>
              )}
            </div>
          </DetailSection>

          <DetailSection title="Networks">
            <div className="space-y-2 text-[14px] text-[rgb(var(--workspace-soft))]">
              {data.networks.length ? (
                data.networks.map((network) => (
                  <div key={network.name}>
                    <div>{network.name}</div>
                    <div className="text-[12px] text-[rgb(var(--workspace-muted))]">
                      {network.ip_address ?? "No IP assigned"}
                    </div>
                  </div>
                ))
              ) : (
                <span className="text-[14px] text-[rgb(var(--workspace-muted))]">
                  No networks attached
                </span>
              )}
            </div>
          </DetailSection>

          <DetailSection title="Runtime">
            <div className="flex flex-wrap gap-2">
              {data.restart_policy ? <InlineTag>{data.restart_policy}</InlineTag> : null}
              {data.health ? <InlineTag>{data.health}</InlineTag> : null}
              <InlineTag>{data.state}</InlineTag>
            </div>
          </DetailSection>
        </div>
      ) : null}
    </aside>
  );
}
