import { $api } from '@lib/providers/api';
import { useQuery } from '@tanstack/react-query';
import { useParams } from 'next/navigation';
import { useMemo } from 'react';

export function useOrg() {
    const orgs = useQuery($api.queryOptions('get', '/api/organizations'));
    const { org_slug } = useParams<{ org_slug: string }>();

    const thisOrg = useMemo(
        () => orgs.data?.find((org) => org.slug === org_slug),
        [org_slug, orgs.data]
    );

    return { thisOrg, orgs: orgs.data };
}
