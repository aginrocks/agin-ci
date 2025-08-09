'use client';
import { Organization } from '@components/organization';
import { PageHeader } from '@components/page-header';
import { Button } from '@components/ui/button';
import { useServerRole } from '@lib/hooks';
import { $api } from '@lib/providers/api';
import { IconPlus } from '@tabler/icons-react';
import { useQuery } from '@tanstack/react-query';
import Link from 'next/link';

export default function Page() {
    const role = useServerRole();

    const organizations = useQuery($api.queryOptions('get', '/api/organizations'));

    return (
        <>
            <PageHeader
                path={[
                    {
                        label: 'Organizations',
                    },
                ]}
                rightSection={
                    role !== 'readonly' && (
                        <Button asChild>
                            <Link href="/app/orgs/new">
                                <IconPlus />
                                Create Organization
                            </Link>
                        </Button>
                    )
                }
            />
            <div className="flex flex-1 flex-col gap-2 p-4 pt-0">
                {organizations.data?.map((org) => (
                    <Organization key={org.slug} {...org} own_role="admin" />
                ))}
            </div>
        </>
    );
}
