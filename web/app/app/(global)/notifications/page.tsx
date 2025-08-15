'use client';
import { PageHeader } from '@components/page-header';
import { $api } from '@lib/providers/api';
import { useQuery } from '@tanstack/react-query';

export default function Page() {
    const notifications = useQuery($api.queryOptions('get', '/api/notifications'));

    return (
        <>
            <PageHeader
                path={[
                    {
                        label: 'Notifications',
                    },
                ]}
            />
            <div className="flex flex-1 flex-col gap-2 p-4 pt-0">Test</div>
        </>
    );
}
