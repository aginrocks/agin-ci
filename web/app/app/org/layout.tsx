import { OrgSidebar } from '@components/org-sidebar';
import { SidebarInset } from '@components/ui/sidebar';

export default function Layout({ children }: { children: React.ReactNode }) {
    return (
        <>
            <OrgSidebar />
            <SidebarInset>{children}</SidebarInset>
        </>
    );
}
