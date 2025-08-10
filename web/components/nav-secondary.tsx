'use client';
import * as React from 'react';
import { type LucideIcon } from 'lucide-react';

import {
    SidebarGroup,
    SidebarGroupContent,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
} from '@/components/ui/sidebar';
import { Icon, IconCrown, IconSkull } from '@tabler/icons-react';
import Link from 'next/link';
import { Badge } from './ui/badge';
import { useQuery } from '@tanstack/react-query';
import { $api } from '@lib/providers/api';
import { useSkipConfirm } from '@lib/hooks';

export function NavSecondary({
    items,
    ...props
}: {
    items: {
        title: string;
        url: string;
        icon: LucideIcon | Icon;
    }[];
} & React.ComponentPropsWithoutRef<typeof SidebarGroup>) {
    const godMode = useQuery($api.queryOptions('get', '/api/god'));
    const skipConfirm = useSkipConfirm();

    return (
        <SidebarGroup {...props}>
            <SidebarGroupContent>
                <SidebarMenu>
                    {skipConfirm && (
                        <Badge variant="secondary" className="bg-red-400/20 text-red-400 mb-1 ml-2">
                            <IconSkull />
                            Deleting Without Confirmation
                        </Badge>
                    )}
                    {godMode.data?.enabled && (
                        <Badge variant="secondary" className="bg-amber-600 text-white mb-1 ml-2">
                            <IconCrown />
                            God Mode Enabled
                        </Badge>
                    )}
                    {items.map((item) => (
                        <SidebarMenuItem key={item.title}>
                            <SidebarMenuButton asChild size="sm">
                                <Link href={item.url} target="_blank">
                                    <item.icon />
                                    <span>{item.title}</span>
                                </Link>
                            </SidebarMenuButton>
                        </SidebarMenuItem>
                    ))}
                </SidebarMenu>
            </SidebarGroupContent>
        </SidebarGroup>
    );
}
