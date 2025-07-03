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
import { IconArrowLeft, IconBook, IconBrandGithub } from '@tabler/icons-react';
import { NavMain } from '@/components/nav-main';
import { NavProjects } from '@/components/nav-projects';
import { NavSecondary } from '@/components/nav-secondary';
import { NavUser } from '@/components/nav-user';
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
import { navSecondary } from './sidebar-common';
import Link from 'next/link';
import { useQuery } from '@tanstack/react-query';
import { $api } from '@lib/providers/api';
import { OrgSwitcher } from './org-switcher';
import { useMemo } from 'react';
import { useParams, useRouter } from 'next/navigation';
import { useOrg } from '@lib/hooks';
import { Button } from './ui/button';

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

export function OrgSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
    const router = useRouter();
    const { thisOrg, orgs } = useOrg();

    return (
        <Sidebar variant="inset" {...props}>
            <SidebarHeader className="gap-1">
                <div className="max-w-max">
                    <Button size="xs" variant="link" asChild>
                        <Link href="/app/orgs">
                            <IconArrowLeft />
                            Back
                        </Link>
                    </Button>
                </div>
                <OrgSwitcher
                    data={orgs}
                    activeOrg={thisOrg}
                    onActiveChange={(org) => router.push(`/app/orgs/${org.slug}`)}
                />
            </SidebarHeader>
            <SidebarContent>
                <NavMain items={data.navMain} title="Platform" />
                <NavProjects projects={data.projects} />
                <NavSecondary items={navSecondary} className="mt-auto" />
            </SidebarContent>
            <SidebarFooter>
                <NavUser />
            </SidebarFooter>
        </Sidebar>
    );
}
