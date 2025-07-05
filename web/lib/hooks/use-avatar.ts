import { useEffect, useState } from 'react';

let cache: Record<string, string> = {};

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

            const url = `https://www.gravatar.com/avatar/${gravatarHash}?d=404`;
            cache[email] = url;

            setAvatarUrl(url);
        })();
    }, [email, defaultAvatar]);

    const cached = email && cache[email];
    if (cached) return cached;

    return defaultAvatar || avatarUrl;
}
