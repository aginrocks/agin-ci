import { paths } from '@/types/api';
import z from 'zod';

const RESERVED_SLUGS = ['new', 'edit', 'delete', 'create'];

const isValidSlug = (s: string): boolean => {
    return s
        .split('')
        .every((c) => (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') || c === '-' || c === '_');
};

export const formSchema = z.object({
    name: z.string().min(1, 'Name is required').max(32, 'Name must be at most 32 characters long'),
    description: z.string().max(2048, 'Description must be at most 2048 characters long'),
    slug: z
        .string()
        .min(1, 'Slug is required')
        .max(32, 'Slug must be at most 32 characters long')
        .refine(
            isValidSlug,
            'Slug can only contain lowercase letters, numbers, hyphens, and underscores'
        )
        .refine((s) => !RESERVED_SLUGS.includes(s), 'This slug is reserved and cannot be used'),
    avatar_email: z
        .string()
        .email('Invalid email address')
        .max(64, 'Email must be at most 64 characters long')
        .or(z.literal(''))
        .optional(),
}) satisfies z.ZodType<
    paths['/api/organizations']['post']['requestBody']['content']['application/json']
> satisfies z.ZodType<
    paths['/api/organizations/{org_slug}']['patch']['requestBody']['content']['application/json']
>;

export type FormSchema = z.infer<typeof formSchema>;
