'use client';
import { Button } from '@components/ui/button';
import { $api } from '@lib/providers/api';
import { useQuery } from '@tanstack/react-query';

export default function Home() {
    const { data } = useQuery($api.queryOptions('get', '/api/user'));

    return <div>{data?.email}</div>;
}
