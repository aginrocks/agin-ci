import { DocsLayout } from 'fumadocs-ui/layouts/docs';
import type { ReactNode } from 'react';
import { baseOptions } from '@/app/layout.config';
import { source } from '@/lib/source';
import { IconCircleDashedCheck, IconCode } from '@tabler/icons-react';

export default function Layout({ children }: { children: ReactNode }) {
    return (
        <DocsLayout
            tree={source.pageTree}
            sidebar={{
                tabs: [
                    {
                        title: 'Agin CI',
                        description: 'CI Platform',
                        url: '/platform',
                        icon: (
                            <div className="flex justify-center items-center h-full">
                                <IconCircleDashedCheck className="size-5" />
                            </div>
                        ),
                    },
                    {
                        title: 'API',
                        description: 'API Documentation',
                        url: '/api',
                        icon: (
                            <div className="flex justify-center items-center h-full">
                                <IconCode className="size-5" />
                            </div>
                        ),
                    },
                ],
            }}
            {...baseOptions}
        >
            {children}
        </DocsLayout>
    );
}
