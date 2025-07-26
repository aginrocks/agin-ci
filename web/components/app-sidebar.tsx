'use client';

import * as React from 'react';
import {
    IconBell,
    IconBuildings,
    IconHistory,
    IconHome,
    IconServer,
    IconSettings,
    IconUsers,
} from '@tabler/icons-react';
import { NavMain, NavMainSubItem } from '@/components/nav-main';
import { NavSecondary } from '@/components/nav-secondary';
import { NavUser } from '@/components/nav-user';
import {
    Sidebar,
    SidebarContent,
    SidebarFooter,
    SidebarHeader,
    SidebarMenu,
    SidebarMenuItem,
} from '@/components/ui/sidebar';
import { useQuery } from '@tanstack/react-query';
import { $api } from '@lib/providers/api';
import { navSecondary } from './sidebar-common';
import { Logo } from './logo';
import { useServerRole } from '@lib/hooks';

export function AppSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
    const organizations = useQuery($api.queryOptions('get', '/api/organizations'));
    const role = useServerRole();

    const sidebarOrgs = React.useMemo(
        () =>
            organizations.data?.map(
                (org): NavMainSubItem => ({
                    title: org.name,
                    url: `/app/orgs/${org.slug}`,
                })
            ),
        [organizations.data]
    );

    return (
        <Sidebar variant="inset" {...props}>
            <SidebarHeader>
                <SidebarMenu>
                    <SidebarMenuItem>
                        <Logo />
                    </SidebarMenuItem>
                </SidebarMenu>
            </SidebarHeader>
            <SidebarContent>
                <NavMain
                    items={[
                        {
                            icon: IconHome,
                            title: 'Home',
                            url: '/app',
                        },
                        {
                            icon: IconBell,
                            title: 'Notifications',
                            url: '/app/notifications',
                        },
                        {
                            icon: IconBuildings,
                            title: 'Organizations',
                            url: '/app/orgs',
                            items: sidebarOrgs,
                            // defaultOpen: true,
                        },
                    ]}
                    title="General"
                />
                {role === 'admin' && (
                    <NavMain
                        items={[
                            {
                                icon: IconServer,
                                title: 'Runners',
                                url: '/app/system/runners',
                            },
                            {
                                icon: IconUsers,
                                title: 'Users',
                                url: '/app/system/users',
                            },
                            {
                                icon: IconHistory,
                                title: 'Logs',
                                url: '/app/system/logs',
                            },
                            {
                                icon: IconSettings,
                                title: 'Settings',
                                url: '/app/system/settings',
                            },
                        ]}
                        title="System"
                    />
                )}
                <NavSecondary items={navSecondary} className="mt-auto" />
            </SidebarContent>
            <SidebarFooter>
                <NavUser />
            </SidebarFooter>
        </Sidebar>
    );
}
