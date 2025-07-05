import { Button } from '@components/ui/button';
import { cn } from '@lib/utils';
import clsx from 'clsx';
import Link from 'next/link';

export type HeaderLinkProps = {
    href: string;
    active?: boolean;
    children?: React.ReactNode;
};

export function HeaderLink({ href, active = false, children }: HeaderLinkProps) {
    return (
        <Button asChild className={cn('rounded-full')} variant={active ? 'secondary' : 'ghost'}>
            <Link href={href}>{children}</Link>
        </Button>
    );
}
