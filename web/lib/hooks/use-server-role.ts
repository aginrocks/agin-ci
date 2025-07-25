'use client';
import { $api } from '@lib/providers/api';
import { useQuery } from '@tanstack/react-query';

export function useServerRole() {
    const { data } = useQuery($api.queryOptions('get', '/api/user'));

    return data?.role;
}
