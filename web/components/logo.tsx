import Link from 'next/link';
import Image from 'next/image';
import { cn } from '@lib/utils';

export type LogoProps = React.ComponentProps<'div'> & {
    href?: string;
};

export function Logo({ className, href = '/app', ...props }: LogoProps) {
    return (
        <Link href={href}>
            <div className={cn('px-2 pt-2', className)} {...props}>
                <Image
                    src="/logo.svg"
                    alt="Logo"
                    width={100}
                    height={30.6167}
                    className="hidden dark:block"
                />
                <Image
                    src="/logo-light.svg"
                    alt="Logo"
                    width={100}
                    height={30.6167}
                    className="dark:hidden"
                />
            </div>
        </Link>
    );
}
