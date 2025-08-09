import { OrgRole } from '@/types/org-role';
import { useAvatar } from '@lib/hooks';
import { Avatar, AvatarFallback, AvatarImage } from './ui/avatar';
import Link from 'next/link';

export type OrganizationProps = {
    name: string;
    slug: string;
    avatar_email?: string | null;
    description?: string;
    own_role: OrgRole;
};

export function Organization({
    name,
    slug,
    avatar_email,
    description,
    own_role,
}: OrganizationProps) {
    const avatar = useAvatar(avatar_email || undefined);

    return (
        <Link href={`/app/orgs/${slug}`}>
            <div className="rounded-lg border px-3.5 py-3 flex items-center gap-2.5 cursor-pointer hover:bg-muted/50 transition-colors h-16">
                <Avatar className="rounded-lg">
                    <AvatarImage src={avatar} />
                    <AvatarFallback className="rounded-lg">
                        {name.charAt(0).toUpperCase()}
                    </AvatarFallback>
                </Avatar>
                <div className="flex flex-col gap-0.5">
                    <div className="font-medium text-sm">{name}</div>
                    <div className="text-xs text-muted-foreground">{description}</div>
                </div>
            </div>
        </Link>
    );
}
