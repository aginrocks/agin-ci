'use client';
import { paths } from '@/types/api';
import { OrgRole } from '@/types/org-role';
import { ServerRole } from '@/types/server-role';
import { DataTable } from '@components/data-table';
import { PageHeader } from '@components/page-header';
import { Avatar, AvatarFallback, AvatarImage } from '@components/ui/avatar';
import { Badge } from '@components/ui/badge';
import { Button } from '@components/ui/button';
import { useAvatar, useOrg } from '@lib/hooks';
import { useOrgRole } from '@lib/hooks/use-org-role';
import { useModals } from '@lib/modals/ModalsManager';
import { useChangeRoleMutation, useRemoveMemberMutation } from '@lib/mutations';
import { useChangeSystemRoleMutation } from '@lib/mutations/system';
import { $api } from '@lib/providers/api';
import {
    Icon,
    IconArrowRight,
    IconBrandApple,
    IconBrandUbuntu,
    IconBrandWindows,
    IconCrown,
    IconEye,
    IconPencil,
    IconQuestionMark,
    IconUser,
} from '@tabler/icons-react';
import { useQuery } from '@tanstack/react-query';
import { ColumnDef } from '@tanstack/react-table';
import clsx from 'clsx';
import { useCallback, useMemo } from 'react';

type Runner =
    paths['/api/system/runners']['get']['responses']['200']['content']['application/json'][number];

export default function Page() {
    const modals = useModals();

    const runners = useQuery($api.queryOptions('get', '/api/system/runners'));

    const columns: ColumnDef<Runner>[] = useMemo(
        () => [
            {
                minSize: 200,
                accessorKey: 'display_name',
                header: 'Name',
            },
            {
                header: 'OS',
                minSize: 300,
                cell: ({ row }) => {
                    const host_os_type = row.original.host_os_type || 'unknown';
                    const icons: Record<typeof host_os_type, Icon> = {
                        linux: IconBrandUbuntu,
                        macos: IconBrandApple,
                        windows: IconBrandWindows,
                        unknown: IconQuestionMark,
                    };
                    const osLabels: Record<typeof host_os_type, string> = {
                        linux: 'Linux',
                        macos: 'macOS',
                        windows: 'Windows',
                        unknown: 'Unknown',
                    };
                    const Icon = icons[host_os_type];

                    return (
                        <div className="flex gap-1.5">
                            <Badge variant="secondary">
                                <Icon />
                                {osLabels[host_os_type]}
                            </Badge>
                            {row.original.host_os && (
                                <Badge variant="secondary">{row.original.host_os}</Badge>
                            )}
                            {row.original.host_os_version && (
                                <Badge variant="secondary">{row.original.host_os_version}</Badge>
                            )}
                        </div>
                    );
                },
            },
            {
                header: 'Runner Version',
                minSize: 140,
                maxSize: 140,
                cell: ({ row }) => (
                    <Badge variant="secondary">{row.original.runner_version ?? 'Unknown'}</Badge>
                ),
            },
            {
                minSize: 180,
                maxSize: 180,
                accessorKey: 'last_ping',
                header: 'Status',
            },
        ],
        []
    );

    return (
        <>
            <PageHeader
                path={[
                    {
                        label: 'System',
                        href: '/app',
                    },
                    {
                        label: 'Runners',
                    },
                ]}
            />
            <div className="p-4 pt-0">
                {runners.data && <DataTable columns={columns} data={runners.data} />}
            </div>
        </>
    );
}
