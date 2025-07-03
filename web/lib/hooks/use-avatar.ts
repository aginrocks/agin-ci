import { useEffect, useState } from 'react';

export function useAvatar(email: string | undefined, defaultAvatar?: string): string | undefined {
    const [avatarUrl, setAvatarUrl] = useState<string | undefined>(defaultAvatar);

    useEffect(() => {
        (async () => {
            if (!email || defaultAvatar) return;

            const encoder = new TextEncoder();
            const data = encoder.encode(email.toLowerCase().trim() || '');
            const hashBuffer = await crypto.subtle.digest('SHA-256', data);
            const hashArray = Array.from(new Uint8Array(hashBuffer));
            const gravatarHash = hashArray.map((b) => b.toString(16).padStart(2, '0')).join('');

            setAvatarUrl(`https://www.gravatar.com/avatar/${gravatarHash}?d=identicon`);
        })();
    }, [email, defaultAvatar]);

    return defaultAvatar || avatarUrl;
}
