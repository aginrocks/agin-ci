'use client';

import { ChevronRight } from 'lucide-react';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { useEffect, useState } from 'react';

import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '@/components/ui/collapsible';
import {
    SidebarMenuAction,
    SidebarMenuButton,
    SidebarMenuItem,
    SidebarMenuSub,
    SidebarMenuSubButton,
    SidebarMenuSubItem,
} from '@/components/ui/sidebar';

import { NavMainItem } from './nav-main';

export type NavMainItemProps = {
    item: NavMainItem;
};

export function NavMainItemComponent({ item }: NavMainItemProps) {
    const pathname = usePathname();

    // TODO: handle state correctly for nested navigation
    const isActive = item.isActive || pathname === item.url;

    const [open, setOpen] = useState(isActive || item.defaultOpen);

    useEffect(() => {
        console.log('active change');
        if (isActive) setOpen(true);
    }, [isActive]);

    return (
        <Collapsible asChild open={open} onOpenChange={setOpen}>
            <SidebarMenuItem>
                <SidebarMenuButton asChild tooltip={item.title} isActive={isActive}>
                    <Link href={item.url}>
                        <item.icon />
                        <span>{item.title}</span>
                    </Link>
                </SidebarMenuButton>
                {item.items?.length ? (
                    <>
                        <CollapsibleTrigger asChild>
                            <SidebarMenuAction className="data-[state=open]:rotate-90">
                                <ChevronRight />
                                <span className="sr-only">Toggle</span>
                            </SidebarMenuAction>
                        </CollapsibleTrigger>
                        <CollapsibleContent>
                            <SidebarMenuSub>
                                {item.items?.map((subItem) => (
                                    <SidebarMenuSubItem key={subItem.title}>
                                        <SidebarMenuSubButton asChild>
                                            <Link href={subItem.url}>
                                                <span>{subItem.title}</span>
                                            </Link>
                                        </SidebarMenuSubButton>
                                    </SidebarMenuSubItem>
                                ))}
                            </SidebarMenuSub>
                        </CollapsibleContent>
                    </>
                ) : null}
            </SidebarMenuItem>
        </Collapsible>
    );
}
