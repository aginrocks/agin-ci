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
    IconArrowLeft,
    IconBook,
    IconBox,
    IconBrandGithub,
    IconHistory,
    IconKey,
    IconLayoutDashboard,
    IconSettings,
    IconUsers,
} from '@tabler/icons-react';
import { NavMain, NavMainSubItem } from '@/components/nav-main';
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
import { useParams, usePathname, useRouter } from 'next/navigation';
import { useOrg, useProject } from '@lib/hooks';
import { Button } from './ui/button';

export function ProjectSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
    const router = useRouter();
    const { thisOrg } = useOrg();
    const { thisProject, projects } = useProject();
    const { org_slug, project_slug } = useParams<{ org_slug: string; project_slug: string }>();
    const pathname = usePathname();

    return (
        <Sidebar variant="inset" {...props}>
            <SidebarHeader className="gap-1">
                <div className="max-w-max">
                    <Button size="xs" variant="link" asChild>
                        <Link href={`/app/orgs/${org_slug}/projects`}>
                            <IconArrowLeft />
                            Back to {thisOrg?.name}
                        </Link>
                    </Button>
                </div>
                <OrgSwitcher
                    data={projects}
                    activeOrg={thisProject}
                    context="project"
                    onActiveChange={(p) => {
                        const path = pathname.split('/');
                        path[5] = p.slug;
                        router.push(path.join('/'));
                    }}
                />
            </SidebarHeader>
            <SidebarContent>
                <NavMain
                    items={[
                        {
                            title: 'Overview',
                            url: `/app/orgs/${org_slug}/projects/${project_slug}`,
                            icon: IconLayoutDashboard,
                        },
                        {
                            title: 'Secrets',
                            url: `/app/orgs/${org_slug}/secrets/${project_slug}`,
                            icon: IconKey,
                        },
                        {
                            title: 'Settings',
                            url: `/app/orgs/${org_slug}/settings/${project_slug}`,
                            icon: IconSettings,
                        },
                    ]}
                    title="Project"
                />
                <NavSecondary items={navSecondary} className="mt-auto" />
            </SidebarContent>
            <SidebarFooter>
                <NavUser />
            </SidebarFooter>
        </Sidebar>
    );
}
