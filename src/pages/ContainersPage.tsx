import { ContainerDetail } from "@/components/containers/ContainerDetail";
import { ContainerList } from "@/components/containers/ContainerList";

export function ContainersPage() {
  return (
    <div className="flex h-full min-h-0 flex-col overflow-hidden">
      <ContainerList />
      <ContainerDetail />
    </div>
  );
}
