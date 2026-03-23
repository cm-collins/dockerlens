import { ContainerList } from "@/components/containers/ContainerList";
import { ContainerDetail } from "@/components/containers/ContainerDetail";

export function ContainersPage() {
    return (
        <div className="flex flex-1 overflow-hidden">
            <ContainerList />
            <ContainerDetail />
        </div>
    );
}