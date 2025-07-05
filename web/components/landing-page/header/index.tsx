import { Logo } from '@components/logo';
import { HeaderLink } from './link';
import { Button } from '@components/ui/button';
import { DOCS_URL, REPO_URL } from '@lib/constants';

export function Header() {
    return (
        <div className="fixed top-0 left-0 right-0 p-6 flex justify-center items-center z-50">
            <div className="flex justify-between items-center w-full max-w-7xl">
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
