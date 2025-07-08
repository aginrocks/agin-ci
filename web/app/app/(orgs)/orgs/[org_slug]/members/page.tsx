'use client';
import { paths } from '@/types/api';
import { DataTable } from '@components/data-table';
import { PageHeader } from '@components/page-header';
import { Avatar, AvatarFallback, AvatarImage } from '@components/ui/avatar';
import { Badge } from '@components/ui/badge';
import { Button } from '@components/ui/button';
import { useAvatar, useOrg } from '@lib/hooks';
import { useModals } from '@lib/modals/ModalsManager';
import { useRemoveMemberMutation } from '@lib/mutations';
import { $api } from '@lib/providers/api';
import { Icon, IconCrown, IconEye, IconPencil, IconTrash, IconUser } from '@tabler/icons-react';
import { useQuery } from '@tanstack/react-query';
import { ColumnDef } from '@tanstack/react-table';
import clsx from 'clsx';
import { useCallback, useMemo } from 'react';

type Member =
    paths['/api/organizations/{org_slug}/members']['get']['responses']['200']['content']['application/json'][number];

export default function Page() {
    const { thisOrg, thisOrgSlug } = useOrg();
    const modals = useModals();
    const removeMember = useRemoveMemberMutation({});

    const members = useQuery(
        $api.queryOptions('get', '/api/organizations/{org_slug}/members', {
            params: {
                path: {
                    org_slug: thisOrgSlug,
                },
            },
        })
    );

    const removeMemeberConfirm = useCallback(
        async (memberId: string, memberName: string) => {
            const confirmed = await modals.show('Confirm', {
                title: 'Remove Member',
                description: `Are you sure you want to remove ${memberName} from the organization?`,
                cancelText: 'Cancel',
                confirmText: 'Remove',
                destructive: true,
            });

            if (!confirmed) return;

            removeMember.mutate({
                params: {
                    path: {
                        org_slug: thisOrgSlug,
                        member_id: memberId,
                    },
                },
            });
        },
        [thisOrgSlug]
    );

    const columns: ColumnDef<Member>[] = useMemo(
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
                        viewer: IconEye,
                        owner: IconCrown,
                        admin: IconCrown,
                        member: IconUser,
                    };
                    const Icon = icons[role];

                    return (
                        <Badge
                            variant="secondary"
                            className={clsx({
                                'bg-amber-600': role === 'owner',
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
                        <Button variant="ghost" size="xsIcon">
                            <IconPencil />
                        </Button>
                        <Button
                            variant="ghostDestructive"
                            size="xsIcon"
                            onClick={() =>
                                removeMemeberConfirm(row.original._id, row.original.name)
                            }
                        >
                            <IconTrash />
                        </Button>
                    </div>
                ),
            },
        ],
        [removeMemeberConfirm]
    );

    return (
        <>
            <PageHeader
                path={[
                    {
                        label: thisOrg?.name,
                        href: `/app/orgs/${thisOrg?.slug}`,
                    },
                    {
                        label: 'Members & Permissions',
                    },
                ]}
            />
            <div className="p-4 pt-0">
                {members.data && <DataTable columns={columns} data={members.data} />}
            </div>
        </>
    );
}
