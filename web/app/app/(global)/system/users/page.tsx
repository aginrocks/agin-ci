'use client';
import { paths } from '@/types/api';
import { OrgRole } from '@/types/org-role';
import { DataTable } from '@components/data-table';
import { PageHeader } from '@components/page-header';
import { Avatar, AvatarFallback, AvatarImage } from '@components/ui/avatar';
import { Badge } from '@components/ui/badge';
import { Button } from '@components/ui/button';
import { useAvatar, useOrg } from '@lib/hooks';
import { useOrgRole } from '@lib/hooks/use-org-role';
import { useModals } from '@lib/modals/ModalsManager';
import { useChangeRoleMutation, useRemoveMemberMutation } from '@lib/mutations';
import { $api } from '@lib/providers/api';
import {
    Icon,
    IconCrown,
    IconEye,
    IconPencil,
    IconTrash,
    IconUser,
    IconUserPlus,
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
