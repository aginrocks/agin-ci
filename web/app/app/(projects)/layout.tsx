import { ProjectSidebar } from '@components/project-sidebar';
import { SidebarInset } from '@components/ui/sidebar';

export default function Layout({ children }: { children: React.ReactNode }) {
    return (
        <>
            <ProjectSidebar />
            <SidebarInset>{children}</SidebarInset>
        </>
    );
}
