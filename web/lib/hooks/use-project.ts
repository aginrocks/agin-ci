'use client';
import { $api } from '@lib/providers/api';
import { useQuery } from '@tanstack/react-query';
import { useParams } from 'next/navigation';
import { useMemo } from 'react';

export function useProject() {
    const { org_slug, project_slug } = useParams<{ org_slug: string; project_slug: string }>();
    const projects = useQuery(
        $api.queryOptions('get', '/api/organizations/{org_slug}/projects', {
            params: {
                path: {
                    org_slug,
                },
            },
        })
    );

    const thisProject = useMemo(
        () => projects.data?.find((p) => p.slug === project_slug),
        [project_slug, projects.data]
    );

    return { thisProject, projects: projects.data, thisProjectSlug: project_slug };
}
