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
    slug: z
        .string()
        .min(1, 'Slug is required')
        .max(32, 'Slug must be at most 32 characters long')
        .refine(
            isValidSlug,
            'Slug can only contain lowercase letters, numbers, hyphens, and underscores'
        )
        .refine((s) => !RESERVED_SLUGS.includes(s), 'This slug is reserved and cannot be used'),
    repository: z.object({
        url: z.string(),
        source: z.enum(['github', 'gitea', 'genericgit']),
    }),
}) satisfies z.ZodType<
    paths['/api/organizations/{org_slug}/projects']['post']['requestBody']['content']['application/json']
> satisfies z.ZodType<
    paths['/api/organizations/{org_slug}/projects/{project_slug}']['patch']['requestBody']['content']['application/json']
>;

export type FormSchema = z.infer<typeof formSchema>;
