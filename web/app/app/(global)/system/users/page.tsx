'use client';
import { paths } from '@/types/api';
import { ServerRole } from '@/types/server-role';
import { DataTable } from '@components/data-table';
import { PageHeader } from '@components/page-header';
import { Avatar, AvatarFallback, AvatarImage } from '@components/ui/avatar';
import { Badge } from '@components/ui/badge';
import { Button } from '@components/ui/button';
import { useAvatar } from '@lib/hooks';
import { useModals } from '@lib/modals/ModalsManager';
import { useChangeSystemRoleMutation } from '@lib/mutations/system';
import { $api } from '@lib/providers/api';
import {
    Icon,
    IconArrowRight,
    IconCrown,
    IconEye,
    IconPencil,
    IconUser,
} from '@tabler/icons-react';
import { useQuery } from '@tanstack/react-query';
import { ColumnDef } from '@tanstack/react-table';
import clsx from 'clsx';
import { useCallback, useMemo } from 'react';

type User =
    paths['/api/system/users']['get']['responses']['200']['content']['application/json'][number];

export default function Page() {
    const modals = useModals();

    const users = useQuery($api.queryOptions('get', '/api/system/users'));

    const changeRole = useChangeSystemRoleMutation({});

    const changeRoleAsk = useCallback(
        async (userId: string, userName: string, currentRole: ServerRole) => {
            const selectedRole = await modals.show('SelectServerRole', {
                selectedRole: currentRole,
                user: userName,
            });

            if (!selectedRole) return;

            changeRole.mutate({
                params: {
                    path: {
                        user_id: userId,
                    },
                },
                body: {
                    role: selectedRole,
                },
            });
        },
        []
    );

    const columns: ColumnDef<User>[] = useMemo(
        () => [
            {
                accessorKey: 'name',
                header: 'Name',
                cell: ({ row }) => {
                    const avatar = useAvatar(row.original.email);
                    const username = row.original.name || row.original.email;

                    return (
                        <div className="flex items-center gap-2">
                            <Avatar>
                                <AvatarImage src={avatar} />
                                <AvatarFallback>{username?.charAt(0).toUpperCase()}</AvatarFallback>
                            </Avatar>
                            <div>{username}</div>
                        </div>
                    );
                },
            },
            {
                accessorKey: 'email',
                header: 'Email',
            },
            {
                accessorKey: 'role',
                header: 'Role',
                cell: ({ row }) => {
                    const { role } = row.original;
                    const icons: Record<typeof role, Icon> = {
                        readonly: IconEye,
                        admin: IconCrown,
                        user: IconUser,
                    };
                    const Icon = icons[role];

                    return (
                        <Badge
                            variant="secondary"
                            className={clsx({
                                'bg-amber-600': role === 'admin',
                            })}
                        >
                            <Icon />
                            {role.charAt(0).toUpperCase() + role.slice(1)}
                        </Badge>
                    );
                },
            },
            {
                id: 'actions',
                size: 80,
                cell: ({ row }) => (
                    <div className="flex gap-1">
                        <Button
                            variant="ghost"
                            size="xsIcon"
                            onClick={() => {
                                changeRoleAsk(
                                    row.original._id,
                                    row.original.name,
                                    row.original.role
                                );
                            }}
                        >
                            <IconPencil />
                        </Button>
                        <Button variant="ghost" size="xsIcon">
                            <IconArrowRight />
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
                        label: 'Users',
                    },
                ]}
            />
            <div className="p-4 pt-0">
                {users.data && <DataTable columns={columns} data={users.data} />}
            </div>
        </>
    );
}
