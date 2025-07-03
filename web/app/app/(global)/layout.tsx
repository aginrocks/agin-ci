import { AppSidebar } from '@components/app-sidebar';
import { SidebarInset } from '@components/ui/sidebar';

export default function Layout({ children }: { children: React.ReactNode }) {
    return (
        <>
            <AppSidebar />
            <SidebarInset>{children}</SidebarInset>
        </>
    );
}
