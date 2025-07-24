import { paths } from './api';

export type OrgRole =
    paths['/api/organizations/{org_slug}/members']['get']['responses']['200']['content']['application/json'][number]['role'];
