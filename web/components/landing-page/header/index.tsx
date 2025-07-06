'use client';
import { Logo } from '@components/logo';
import { HeaderLink } from './link';
import { Button } from '@components/ui/button';
import { DOCS_URL, REPO_URL } from '@lib/constants';
import { useWindowScroll } from '@mantine/hooks';
import { cn } from '@lib/utils';
import clsx from 'clsx';

export function Header() {
    const [{ y }] = useWindowScroll();

    const hasBackground = y > 100;

    return (
        <div className="fixed top-0 left-0 right-0 p-3 flex justify-center items-center z-50">
            <div
                className={cn(
                    'flex justify-between items-center w-full max-w-7xl p-3 rounded-full border border-transparent transition-all duration-500',
                    clsx({
                        'bg-neutral-100 dark:bg-neutral-800/20 backdrop-blur-xl border-border':
                            hasBackground,
                    })
                )}
            >
                <div className="flex gap-4 items-center">
                    <Logo className="p-0" href="/" />
                    <div className="flex items-center mb-[2px] gap-0.5">
                        <HeaderLink href="/" isActive={true}>
                            About
                        </HeaderLink>
                        <HeaderLink href={DOCS_URL} isExternal>
                            Documentation
                        </HeaderLink>
                        <HeaderLink href={REPO_URL} isExternal>
                            GitHub
                        </HeaderLink>
                    </div>
                </div>
                <Button className="rounded-full" size="lg" asChild>
                    <a href="/api/login">Log in</a>
                </Button>
            </div>
        </div>
    );
}
