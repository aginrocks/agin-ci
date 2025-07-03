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
