'use client';
import { PageHeader } from '@components/page-header';
import { SettingsTabs } from '@components/settings/tabs';
import { useOrg, useProject } from '@lib/hooks';
import { IconGitMerge, IconSettings } from '@tabler/icons-react';

export default function Layout({ children }: { children: React.ReactNode }) {
    const { thisOrg, thisOrgSlug } = useOrg();
    const { thisProject, thisProjectSlug } = useProject();

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
                        label: 'Settings',
                    },
                ]}
            />
            <SettingsTabs
                tabs={[
                    {
                        icon: IconSettings,
                        label: 'General',
                        url: `/app/orgs/${thisOrgSlug}/projects/${thisProjectSlug}/settings`,
                    },
                    {
                        icon: IconGitMerge,
                        label: 'Repository',
                        url: `/app/orgs/${thisOrgSlug}/projects/${thisProjectSlug}/settings/repository`,
                    },
                ]}
            >
                {children}
            </SettingsTabs>
        </>
    );
}
