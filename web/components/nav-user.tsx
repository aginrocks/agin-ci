'use client';

import { ChevronsUpDown, LogOut } from 'lucide-react';

import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuGroup,
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
import { useAvatar } from '@lib/hooks';
import { useQuery } from '@tanstack/react-query';
import { $api } from '@lib/providers/api';
import { IconCheck, IconCrown, IconKey, IconSettings } from '@tabler/icons-react';
import clsx from 'clsx';
import { useGodModeMutation } from '@lib/mutations';
import { useCallback } from 'react';

export function NavUser() {
    const { isMobile } = useSidebar();

    const { data } = useQuery($api.queryOptions('get', '/api/user'));

    const godMode = useQuery($api.queryOptions('get', '/api/god'));
    const godModeMutation = useGodModeMutation({});
    const toggleGodMode = useCallback(() => {
        godModeMutation.mutate({
            body: {
                enable: !godMode.data?.enabled,
            },
        });
    }, [godMode.data?.enabled]);

    const avatar = useAvatar(data?.email);

    const avatarFallbackText = data?.name?.charAt(0)?.toUpperCase() ?? '';

    return (
        <SidebarMenu>
            <SidebarMenuItem>
                <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                        <SidebarMenuButton
                            size="lg"
                            className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
                        >
                            <Avatar className="h-8 w-8 rounded-lg">
                                <AvatarImage src={avatar} alt={data?.name} />
                                <AvatarFallback className="rounded-lg">
                                    {avatarFallbackText}
                                </AvatarFallback>
                            </Avatar>
                            <div className="grid flex-1 text-left text-sm leading-tight">
                                <span className="truncate font-medium">{data?.name}</span>
                                <span className="truncate text-xs">{data?.email}</span>
                            </div>
                            <ChevronsUpDown className="ml-auto size-4" />
                        </SidebarMenuButton>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent
                        className="w-(--radix-dropdown-menu-trigger-width) min-w-56 rounded-lg"
                        side={isMobile ? 'bottom' : 'right'}
                        align="end"
                        sideOffset={4}
                    >
                        <DropdownMenuLabel className="p-0 font-normal">
                            <div className="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
                                <Avatar className="h-8 w-8 rounded-lg">
                                    <AvatarImage src={avatar} alt={data?.name} />
                                    <AvatarFallback className="rounded-lg">
                                        {avatarFallbackText}
                                    </AvatarFallback>
                                </Avatar>
                                <div className="grid flex-1 text-left text-sm leading-tight">
                                    <span className="truncate font-medium">{data?.name}</span>
                                    <span className="truncate text-xs">{data?.email}</span>
                                </div>
                            </div>
                        </DropdownMenuLabel>
                        <DropdownMenuSeparator />
                        <DropdownMenuGroup>
                            <DropdownMenuItem>
                                <IconSettings />
                                Account Settings
                            </DropdownMenuItem>
                            <DropdownMenuItem>
                                <IconKey />
                                API Keys
                            </DropdownMenuItem>
                            {data?.role === 'admin' && (
                                <DropdownMenuItem onClick={toggleGodMode}>
                                    <IconCrown
                                        className={clsx({
                                            'text-amber-600': godMode.data?.enabled,
                                        })}
                                    />
                                    <span
                                        className={clsx({
                                            'text-amber-600': godMode.data?.enabled,
                                        })}
                                    >
                                        God Mode
                                    </span>
                                    <div className="flex-1" />
                                    {godMode.data?.enabled && (
                                        <IconCheck className="text-amber-600" />
                                    )}
                                </DropdownMenuItem>
                            )}
                        </DropdownMenuGroup>
                        <DropdownMenuSeparator />
                        <DropdownMenuItem>
                            <LogOut />
                            Log out
                        </DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>
            </SidebarMenuItem>
        </SidebarMenu>
    );
}
