'use client';

import * as React from 'react';
import {
    IconArrowLeft,
    IconBox,
    IconHistory,
    IconKey,
    IconLayoutDashboard,
    IconSettings,
    IconUsers,
} from '@tabler/icons-react';
import { NavMain, NavMainSubItem } from '@/components/nav-main';
import { NavSecondary } from '@/components/nav-secondary';
import { NavUser } from '@/components/nav-user';
import { Sidebar, SidebarContent, SidebarFooter, SidebarHeader } from '@/components/ui/sidebar';
import { navSecondary } from './sidebar-common';
import Link from 'next/link';
import { useQuery } from '@tanstack/react-query';
import { $api } from '@lib/providers/api';
import { OrgSwitcher } from './org-switcher';
import { useParams, usePathname, useRouter } from 'next/navigation';
import { useOrg } from '@lib/hooks';
import { Button } from './ui/button';

export function OrgSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
    const router = useRouter();
    const { thisOrg, orgs } = useOrg();
    const { org_slug } = useParams<{ org_slug: string }>();
    const pathname = usePathname();

    const projects = useQuery(
        $api.queryOptions('get', '/api/organizations/{org_slug}/projects', {
            params: {
                path: {
                    org_slug,
                },
            },
        })
    );

    const sidebarProjects = React.useMemo(
        () =>
            projects.data?.map(
                (p): NavMainSubItem => ({
                    title: p.name,
                    url: `/app/orgs/${org_slug}/projects/${p.slug}`,
                })
            ),
        [projects.data]
    );

    return (
        <Sidebar variant="inset" {...props}>
            <SidebarHeader className="gap-1">
                <div className="max-w-max mt-1">
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
                    context="org"
                    onActiveChange={(org) => {
                        const path = pathname.split('/');
                        path[3] = org.slug;

                        // If a project is selected, go back to the project list when switching organizations
                        if (path[4] === 'projects' && path[5] !== 'new') {
                            path.splice(5);
                        }

                        router.push(path.join('/'));
                    }}
                />
            </SidebarHeader>
            <SidebarContent>
                <NavMain
                    items={[
                        {
                            title: 'Overview',
                            url: `/app/orgs/${org_slug}`,
                            icon: IconLayoutDashboard,
                        },
                        {
                            title: 'Members & Permissions',
                            url: `/app/orgs/${org_slug}/members`,
                            icon: IconUsers,
                        },
                        {
                            title: 'Secrets',
                            url: `/app/orgs/${org_slug}/secrets`,
                            icon: IconKey,
                        },
                        {
                            title: 'Logs',
                            url: `/app/orgs/${org_slug}/logs`,
                            icon: IconHistory,
                        },
                        {
                            title: 'Projects',
                            url: `/app/orgs/${org_slug}/projects`,
                            icon: IconBox,
                            items: sidebarProjects,
                        },
                        {
                            title: 'Settings',
                            url: `/app/orgs/${org_slug}/settings`,
                            icon: IconSettings,
                        },
                    ]}
                    title="Organization"
                />
                <NavSecondary items={navSecondary} className="mt-auto" />
            </SidebarContent>
            <SidebarFooter>
                <NavUser />
            </SidebarFooter>
        </Sidebar>
    );
}
