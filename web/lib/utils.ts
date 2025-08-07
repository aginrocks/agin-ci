import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
}

export function slugify(input: string): string {
    return input
        .replace(/[^a-zA-Z0-9 ]/g, '') // Remove special characters
        .trim()
        .replace(/\s+/g, '-') // Replace spaces with -
        .toLowerCase(); // Convert to lowercase
}

export function formatDuration(seconds: number): string {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;

    let result = '';
    if (hours > 0) {
        result += `${hours}h `;
    }
    if (minutes > 0 || hours > 0) {
        result += `${minutes}m `;
    }
    result += `${secs}s`;

    return result.trim();
}

export function prependClassName<T extends HTMLElement>(element: T, newClassName: string): void {
    const classes: string[] = element.className.trim().split(/\s+/).filter(Boolean);
    element.className = [newClassName, ...classes.filter((cls) => cls !== newClassName)].join(' ');
}
