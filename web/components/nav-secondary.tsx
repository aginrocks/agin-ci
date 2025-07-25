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
import { Icon, IconCrown } from '@tabler/icons-react';
import Link from 'next/link';
import { Badge } from './ui/badge';
import { useQuery } from '@tanstack/react-query';
import { $api } from '@lib/providers/api';

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

    return (
        <SidebarGroup {...props}>
            <SidebarGroupContent>
                <SidebarMenu>
                    {godMode.data?.enabled && (
                        <Badge variant="secondary" className="bg-amber-600 mb-1 ml-2">
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
