'use client';
import { PageHeader } from '@components/page-header';
import { useOrg, useProject } from '@lib/hooks';

export default function Page() {
    const { thisOrg } = useOrg();
    const { thisProject } = useProject();

    return (
        <>
            <PageHeader
                path={[
                    {
                        label: thisOrg?.name,
                        href: `/app/orgs/${thisOrg?.slug}`,
                    },
                    {
                        label: thisProject?.name,
                        href: `/app/orgs/${thisOrg?.slug}/projects/${thisProject?.slug}`,
                    },
                    {
                        label: 'Overview',
                    },
                ]}
            />
            <div className="flex flex-1 flex-col gap-4 p-4 pt-0">
                <div className="grid auto-rows-min gap-4 md:grid-cols-3">
                    <div className="bg-muted/50 aspect-video rounded-xl" />
                    <div className="bg-muted/50 aspect-video rounded-xl" />
                    <div className="bg-muted/50 aspect-video rounded-xl" />
                </div>
                <div className="bg-muted/50 min-h-[100vh] flex-1 rounded-xl md:min-h-min" />
            </div>
        </>
    );
}
