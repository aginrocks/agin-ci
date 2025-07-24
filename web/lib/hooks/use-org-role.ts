'use client';
import { $api } from '@lib/providers/api';
import { useQuery } from '@tanstack/react-query';
import { useParams } from 'next/navigation';
import { useMemo } from 'react';

export function useOrgRole() {
    const { org_slug } = useParams<{ org_slug: string }>();

    const userData = useQuery($api.queryOptions('get', '/api/user'));

    const orgs = useQuery(
        $api.queryOptions('get', '/api/organizations/{org_slug}/members', {
            params: {
                path: {
                    org_slug,
                },
            },
        })
    );

    const thisMember = useMemo(
        () => orgs.data?.find((member) => member._id === userData.data?._id),
        [org_slug, orgs.data, userData.data?._id]
    );

    return { thisMember: thisMember, role: thisMember?.role };
}
