import { DocsLayout } from 'fumadocs-ui/layouts/docs';
import type { ReactNode } from 'react';
import { baseOptions } from '@/app/layout.config';
import { source } from '@/lib/source';

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
                    },
                    {
                        title: 'API',
                        description: 'API Documentation',
                        url: '/api',
                    },
                ],
            }}
            {...baseOptions}
        >
            {children}
        </DocsLayout>
    );
}
