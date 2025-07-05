import { Button } from '@components/ui/button';
import { cn } from '@lib/utils';
import Link from 'next/link';

export type HeaderLinkProps = {
    href: string;
    isActive?: boolean;
    children?: React.ReactNode;
    isExternal?: boolean;
};

export function HeaderLink({
    href,
    isActive = false,
    isExternal = false,
    children,
}: HeaderLinkProps) {
    const Comp = isExternal ? 'a' : Link;

    return (
        <Button asChild className={cn('rounded-full')} variant={isActive ? 'secondary' : 'ghost'}>
            <Comp href={href} target={isExternal ? '_blank' : undefined}>
                {children}
            </Comp>
        </Button>
    );
}
