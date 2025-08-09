'use client';

import * as React from 'react';
import { ChevronsUpDown, Plus } from 'lucide-react';

import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import {
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
    useSidebar,
} from '@/components/ui/sidebar';
import { IconCheck } from '@tabler/icons-react';
import Link from 'next/link';
import { useParams } from 'next/navigation';
import { useAvatar, useOrgRole, useServerRole } from '@lib/hooks';
import { Avatar, AvatarFallback, AvatarImage } from './ui/avatar';

export type Org = {
    name: string;
    members?: any[];
    slug: string;
    gravatar_email?: string;

    [key: string]: any;
};

export type OrgSwitcherProps = {
    data: Org[] | undefined;
    activeOrg?: Org;
    onActiveChange: (org: Org) => void;
    context: 'org' | 'project';
};

export function OrgSwitcher({ data, activeOrg, onActiveChange, context }: OrgSwitcherProps) {
    const { isMobile } = useSidebar();

    const memberCount = activeOrg?.members?.length;

    const { org_slug } = useParams<{ org_slug: string }>();

    const role = useServerRole();
    const orgRole = useOrgRole();

    const avatar = useAvatar(activeOrg?.avatar_email);

    return (
        <SidebarMenu>
            <SidebarMenuItem>
                <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                        <SidebarMenuButton
                            size="lg"
                            className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
                        >
                            <Avatar className="rounded-lg">
                                <AvatarImage src={avatar} />
                                <AvatarFallback className="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg">
                                    {activeOrg?.name.charAt(0).toUpperCase()}
                                </AvatarFallback>
                            </Avatar>
                            <div className="grid flex-1 text-left text-sm leading-tight">
                                <span className="truncate font-medium">{activeOrg?.name}</span>
                                {memberCount !== undefined && (
                                    <span className="truncate text-xs text-muted-foreground">
                                        {memberCount} member{memberCount === 1 ? '' : 's'}
                                    </span>
                                )}
                            </div>
                            <ChevronsUpDown className="ml-auto" />
                        </SidebarMenuButton>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent
                        className="w-(--radix-dropdown-menu-trigger-width) min-w-56 rounded-lg"
                        align="start"
                        side={isMobile ? 'bottom' : 'right'}
                        sideOffset={4}
                    >
                        <DropdownMenuLabel className="text-muted-foreground text-xs">
                            {context === 'org' ? 'Organizations' : 'Projects'}
                        </DropdownMenuLabel>
                        {data?.map((org, index) => (
                            <DropdownMenuItem
                                key={org.name}
                                onClick={() => onActiveChange(org)}
                                className="gap-2 p-2"
                            >
                                <div className="flex size-6 items-center justify-center rounded-md border">
                                    <div className="text-muted-foreground">
                                        {org.name.charAt(0).toUpperCase()}
                                    </div>
                                </div>
                                {org.name}
                                {org.slug === activeOrg?.slug && <IconCheck className="ml-auto" />}
                            </DropdownMenuItem>
                        ))}
                        {((context === 'org' && role !== 'readonly') ||
                            (context === 'project' && orgRole.role !== 'viewer')) && (
                            <>
                                <DropdownMenuSeparator />
                                <DropdownMenuItem className="gap-2 p-2" asChild>
                                    <Link
                                        href={
                                            context === 'org'
                                                ? '/app/orgs/new'
                                                : `/app/orgs/${org_slug}/projects/new`
                                        }
                                    >
                                        <div className="flex size-6 items-center justify-center rounded-md border bg-transparent">
                                            <Plus className="size-4" />
                                        </div>
                                        <div className="text-muted-foreground font-medium">
                                            Create {context === 'org' ? 'Organization' : 'Project'}
                                        </div>
                                    </Link>
                                </DropdownMenuItem>
                            </>
                        )}
                    </DropdownMenuContent>
                </DropdownMenu>
            </SidebarMenuItem>
        </SidebarMenu>
    );
}
