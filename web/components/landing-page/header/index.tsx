import { Logo } from '@components/logo';
import { HeaderLink } from './link';
import { Button } from '@components/ui/button';

export function Header() {
    return (
        <div className="fixed top-0 left-0 right-0 p-6 flex justify-center items-center">
            <div className="flex justify-between items-center w-full max-w-7xl">
                <div className="flex gap-4 items-center">
                    <Logo className="p-0" />
                    <div className="flex items-center mb-[2px] gap-0.5">
                        <HeaderLink href="/" active={true}>
                            About
                        </HeaderLink>
                        <HeaderLink href="/">Documentation</HeaderLink>
                        <HeaderLink href="/">GitHub</HeaderLink>
                    </div>
                </div>
                <Button className="rounded-full" size="lg">
                    Log in
                </Button>
            </div>
        </div>
    );
}
