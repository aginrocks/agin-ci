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
                        label: 'Settings',
                    },
                ]}
            />
            <div className="flex-1 p-4 pt-0">
                <div className="text-2xl font-semibold">Settings</div>
                <div className="text-muted-foreground">
                    Manage your organization settings, including members, billing, and more.
                </div>
                {/* Add your settings components here */}
            </div>
        </>
    );
}
