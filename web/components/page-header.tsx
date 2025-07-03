import { SidebarTrigger } from './ui/sidebar';
import {
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbList,
    BreadcrumbPage,
    BreadcrumbSeparator,
} from './ui/breadcrumb';
import Link from 'next/link';
import { Separator } from './ui/separator';
import React from 'react';

export type PathSegment = {
    label: string | undefined;
    href?: string;
};

export type PageHeaderProps = {
    path: PathSegment[];
};

export function PageHeader({ path }: PageHeaderProps) {
    const lastSegment = path[path.length - 1];
    const otherSegments = path.slice(0, -1);

    return (
        <header className="flex h-16 shrink-0 items-center gap-2">
            <div className="flex items-center gap-2 px-4">
                <SidebarTrigger className="-ml-1" />
                <Separator
                    orientation="vertical"
                    className="mr-2 data-[orientation=vertical]:h-4"
                />
                <Breadcrumb>
                    <BreadcrumbList>
                        {otherSegments.map((segment, i) => (
                            <React.Fragment key={i}>
                                <BreadcrumbItem className="hidden md:block">
                                    <BreadcrumbLink asChild>
                                        <Link href={segment.href ?? '#'}>{segment.label}</Link>
                                    </BreadcrumbLink>
                                </BreadcrumbItem>
                                <BreadcrumbSeparator className="hidden md:block" />
                            </React.Fragment>
                        ))}
                        <BreadcrumbItem>
                            <BreadcrumbPage>{lastSegment?.label}</BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            </div>
        </header>
    );
}
