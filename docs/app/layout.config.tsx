import type { BaseLayoutProps } from 'fumadocs-ui/layouts/shared';
import Image from 'next/image';

/**
 * Shared layout configurations
 *
 * you can customise layouts individually from:
 * Home Layout: app/(home)/layout.tsx
 * Docs Layout: app/docs/layout.tsx
 */
export const baseOptions: BaseLayoutProps = {
    nav: {
        title: (
            <>
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
            </>
        ),
    },
    githubUrl: 'https://github.com/aginrocks/agin-ci',
    // see https://fumadocs.dev/docs/ui/navigation/links
    links: [],
};
