'use client';

import * as React from 'react';
import {
    BookOpen,
    Bot,
    Command,
    Frame,
    LifeBuoy,
    Map,
    PieChart,
    Send,
    Settings2,
    SquareTerminal,
} from 'lucide-react';
import {
    IconBell,
    IconBook,
    IconBrandGithub,
    IconBuildings,
    IconHistory,
    IconHome,
    IconServer,
    IconSettings,
} from '@tabler/icons-react';
import { NavMain, NavMainSubItem } from '@/components/nav-main';
import { NavProjects } from '@/components/nav-projects';
import { NavSecondary } from '@/components/nav-secondary';
import { NavUser } from '@/components/nav-user';
import Image from 'next/image';
import {
    Sidebar,
    SidebarContent,
    SidebarFooter,
    SidebarHeader,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
} from '@/components/ui/sidebar';
import { REPO_URL } from '@lib/constants';
import Link from 'next/link';
import { useQuery } from '@tanstack/react-query';
import { $api } from '@lib/providers/api';
import { navSecondary } from './sidebar-common';

const data = {
    navMain: [
        {
            title: 'Playground',
            url: '#',
            icon: SquareTerminal,
            isActive: true,
            items: [
                {
                    title: 'History',
                    url: '#',
                },
                {
                    title: 'Starred',
                    url: '#',
                },
                {
                    title: 'Settings',
                    url: '#',
                },
            ],
        },
        {
            title: 'Models',
            url: '#',
            icon: Bot,
            items: [
                {
                    title: 'Genesis',
                    url: '#',
                },
                {
                    title: 'Explorer',
                    url: '#',
                },
                {
                    title: 'Quantum',
                    url: '#',
                },
            ],
        },
        {
            title: 'Documentation',
            url: '#',
            icon: BookOpen,
            items: [
                {
                    title: 'Introduction',
                    url: '#',
                },
                {
                    title: 'Get Started',
                    url: '#',
                },
                {
                    title: 'Tutorials',
                    url: '#',
                },
                {
                    title: 'Changelog',
                    url: '#',
                },
            ],
        },
        {
            title: 'Settings',
            url: '#',
            icon: Settings2,
            items: [
                {
                    title: 'General',
                    url: '#',
                },
                {
                    title: 'Team',
                    url: '#',
                },
                {
                    title: 'Billing',
                    url: '#',
                },
                {
                    title: 'Limits',
                    url: '#',
                },
            ],
        },
    ],
    projects: [
        {
            name: 'Design Engineering',
            url: '#',
            icon: Frame,
        },
        {
            name: 'Sales & Marketing',
            url: '#',
            icon: PieChart,
        },
        {
            name: 'Travel',
            url: '#',
            icon: Map,
        },
    ],
};

export function AppSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
    const organizations = useQuery($api.queryOptions('get', '/api/organizations'));

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
                        <Link href="/app">
                            <div className="px-2 pt-2">
                                <Image
                                    src="/logo.svg"
                                    alt="Logo"
                                    width={100}
                                    height={30.6167}
                                    className="hidden dark:block"
                                />
                                <Image
                                    src="/logo-light.svg"
                                    alt="Logo"
                                    width={100}
                                    height={30.6167}
                                    className="dark:hidden"
                                />
                            </div>
                        </Link>
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
                            defaultOpen: true,
                        },
                    ]}
                    title="General"
                />
                <NavMain
                    items={[
                        {
                            icon: IconServer,
                            title: 'Runners',
                            url: '/app/system/runners',
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
                <NavSecondary items={navSecondary} className="mt-auto" />
            </SidebarContent>
            <SidebarFooter>
                <NavUser />
            </SidebarFooter>
        </Sidebar>
    );
}
