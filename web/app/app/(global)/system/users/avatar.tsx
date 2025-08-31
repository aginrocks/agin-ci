import { Avatar, AvatarFallback, AvatarImage } from '@components/ui/avatar';
import { useAvatar } from '@lib/hooks';

export function InlineAvatar({ username, email }: { username: string; email?: string }) {
    const avatar = useAvatar(email);
    return (
        <Avatar>
            <AvatarImage src={avatar} />
            <AvatarFallback>{username?.charAt(0).toUpperCase()}</AvatarFallback>
        </Avatar>
    );
}
