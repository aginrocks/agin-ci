'use client';
import { SidebarMenu, SidebarMenuButton, SidebarMenuItem } from '@components/ui/sidebar';
import { Icon } from '@tabler/icons-react';
import Link from 'next/link';
import { usePathname } from 'next/navigation';

export type SettingsTab = {
    label: string;
    url: string;
    icon: Icon;
};

export type SettingsTabsProps = {
    tabs: SettingsTab[];
    children?: React.ReactNode;
};

export function SettingsTabs({ tabs, children }: SettingsTabsProps) {
    const pathname = usePathname();

    return (
        <div className="flex-1 p-4 pt-0 flex justify-center max-w-full">
            <div className="w-full max-w-3xl lg:pt-4 flex gap-6">
                <SidebarMenu className="w-48">
                    {tabs.map((t) => {
                        const isActive = t.url === pathname;

                        return (
                            <SidebarMenuItem key={t.url}>
                                <SidebarMenuButton asChild tooltip={t.label} isActive={isActive}>
                                    <Link href={t.url}>
                                        <t.icon />
                                        <span>{t.label}</span>
                                    </Link>
                                </SidebarMenuButton>
                            </SidebarMenuItem>
                        );
                    })}
                </SidebarMenu>
                <div className="flex-1">{children}</div>
            </div>
        </div>
    );
}
