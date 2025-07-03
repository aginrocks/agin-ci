'use client';
import { PageHeader } from '@components/page-header';
import { useOrg } from '@lib/hooks';

export default function Page() {
    const { thisOrg } = useOrg();
    return (
        <>
            <PageHeader
                path={[
                    {
                        label: 'Organizations',
                        href: '/app/orgs',
                    },
                    {
                        label: thisOrg?.name,
                        href: `/app/orgs/${thisOrg?.slug}`,
                    },
                    {
                        label: 'Secrets',
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
