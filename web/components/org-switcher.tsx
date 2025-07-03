'use client';

import * as React from 'react';
import { ChevronsUpDown, Plus } from 'lucide-react';

import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuShortcut,
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

export type Org = {
    name: string;
    members: any[];
    slug: string;

    [key: string]: any;
};

export type OrgSwitcherProps = {
    data: Org[] | undefined;
    activeOrg?: Org;
    onActiveChange: (org: Org) => void;
};

export function OrgSwitcher({ data, activeOrg, onActiveChange }: OrgSwitcherProps) {
    const { isMobile } = useSidebar();

    const memberCount = activeOrg?.members?.length || 0;

    return (
        <SidebarMenu>
            <SidebarMenuItem>
                <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                        <SidebarMenuButton
                            size="lg"
                            className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
                        >
                            <div className="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg">
                                <div>{activeOrg?.name.charAt(0).toUpperCase()}</div>
                            </div>
                            <div className="grid flex-1 text-left text-sm leading-tight">
                                <span className="truncate font-medium">{activeOrg?.name}</span>
                                <span className="truncate text-xs text-muted-foreground">
                                    {memberCount} member{memberCount === 1 ? '' : 's'}
                                </span>
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
                            Organizations
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
                        <DropdownMenuSeparator />
                        <DropdownMenuItem className="gap-2 p-2" asChild>
                            <Link href="/app/orgs/new">
                                <div className="flex size-6 items-center justify-center rounded-md border bg-transparent">
                                    <Plus className="size-4" />
                                </div>
                                <div className="text-muted-foreground font-medium">
                                    Create Organization
                                </div>
                            </Link>
                        </DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>
            </SidebarMenuItem>
        </SidebarMenu>
    );
}
