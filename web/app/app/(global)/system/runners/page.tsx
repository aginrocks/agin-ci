'use client';
import { paths } from '@/types/api';
import { DataTable } from '@components/data-table';
import { PageHeader } from '@components/page-header';
import { Badge } from '@components/ui/badge';
import { StatusBadge } from '@components/ui/status-badge';
import { useModals } from '@lib/modals/ModalsManager';
import { $api } from '@lib/providers/api';
import {
    Icon,
    IconBrandApple,
    IconBrandUbuntu,
    IconBrandWindows,
    IconPencil,
    IconPlus,
    IconTrash,
} from '@tabler/icons-react';
import { useQuery } from '@tanstack/react-query';
import { ColumnDef } from '@tanstack/react-table';
import { useCallback, useMemo } from 'react';
import moment from 'moment';
import { Button } from '@components/ui/button';
import { useCreateRunnerMutation, useEditRunnerMutation } from '@lib/mutations';

type Runner =
    paths['/api/system/runners']['get']['responses']['200']['content']['application/json'][number];

// TODO: Fix the tables

export default function Page() {
    const modals = useModals();

    const runners = useQuery($api.queryOptions('get', '/api/system/runners'));

    const createRunner = useCreateRunnerMutation({});
    const createRunnerAsk = useCallback(async () => {
        const data = await modals.show('EditRunner');
        if (!data) return;

        const runnnerData = await createRunner.mutateAsync({
            body: data,
        });

        await modals.show('OneTimeSecret', {
            title: 'Runner Token',
            description: 'Copy this token and use it to register your runner.',
            secret: runnnerData.token,
        });
    }, []);

    const editRunner = useEditRunnerMutation({});
    const editRunnerAsk = useCallback(async (runnerData: Runner) => {
        const data = await modals.show('EditRunner', {
            editData: {
                display_name: runnerData.display_name,
                host_os_type: runnerData.host_os_type || 'unknown',
            },
        });
        if (!data) return;

        editRunner.mutate({
            body: data,
            params: {
                path: {
                    runner_id: runnerData._id,
                },
            },
        });
    }, []);

    const columns: ColumnDef<Runner>[] = useMemo(
        () => [
            {
                // minSize: 150,
                size: 150,
                accessorKey: 'display_name',
                header: 'Name',
            },
            {
                header: 'OS',
                minSize: 300,
                cell: ({ row }) => {
                    const host_os_type = row.original.host_os_type || 'unknown';
                    const icons: Record<typeof host_os_type, Icon | null> = {
                        linux: IconBrandUbuntu,
                        macos: IconBrandApple,
                        windows: IconBrandWindows,
                        unknown: null,
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
                                {Icon && <Icon />}
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
                header: 'Status',
                cell: ({ row }) => {
                    const active = row.original.last_ping
                        ? new Date(row.original.last_ping) > new Date(Date.now() - 60 * 1000)
                        : false;

                    return (
                        <StatusBadge
                            variant={
                                row.original.last_ping ? (active ? 'success' : 'disabled') : 'error'
                            }
                        >
                            {row.original.last_ping
                                ? active
                                    ? 'Online'
                                    : `Last seen ${moment(row.original.last_ping).fromNow()}`
                                : 'Never Connected'}
                        </StatusBadge>
                    );
                },
            },
            {
                id: 'actions',
                maxSize: 80,
                size: 80,
                cell: ({ row }) => (
                    <div className="flex gap-1">
                        <Button
                            variant="ghost"
                            size="xsIcon"
                            onClick={() => {
                                editRunnerAsk(row.original);
                            }}
                        >
                            <IconPencil />
                        </Button>
                        <Button variant="ghostDestructive" size="xsIcon" onClick={() => {}}>
                            <IconTrash />
                        </Button>
                    </div>
                ),
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
                rightSection={
                    <Button onClick={createRunnerAsk}>
                        <IconPlus />
                        New Runner
                    </Button>
                }
            />
            <div className="p-4 pt-0">
                {runners.data && <DataTable columns={columns} data={runners.data} />}
            </div>
        </>
    );
}
