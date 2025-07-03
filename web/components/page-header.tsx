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

export type PathSegment = {
    label: string;
    href?: string;
};

export type PageHeaderProps = {
    path: PathSegment[];
};

export function PageHeader({ path }: PageHeaderProps) {
    const lastSegment = path[path.length - 1];
    path.pop();

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
                        {path.map((segment, i) => (
                            <div
                                className="flex flex-wrap items-center gap-1.5 text-sm break-words sm:gap-2.5"
                                key={i}
                            >
                                <BreadcrumbItem className="hidden md:block">
                                    <BreadcrumbLink asChild>
                                        <Link href={segment.href ?? '#'}>{segment.label}</Link>
                                    </BreadcrumbLink>
                                </BreadcrumbItem>
                                <BreadcrumbSeparator className="hidden md:block" />
                            </div>
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
