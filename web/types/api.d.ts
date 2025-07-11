/**
 * This file was auto-generated by openapi-typescript.
 * Do not make direct changes to the file.
 */

export interface paths {
    "/api/health": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get health of the service (returns "ok") */
        get: operations["get_health"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/login": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Log in */
        get: operations["log_in"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/organizations": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get all organizations */
        get: operations["get_organizations"];
        put?: never;
        /** Create a new organization */
        post: operations["create_organization"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/organizations/{org_slug}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get org */
        get: operations["get_organization"];
        put?: never;
        post?: never;
        /**
         * Delete org
         * @description Dangerous!
         */
        delete: operations["delete_organization"];
        options?: never;
        head?: never;
        /** Edit org */
        patch: operations["edit_organization"];
        trace?: never;
    };
    "/api/organizations/{org_slug}/members": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get org members */
        get: operations["get_organization_members"];
        /** Add org member */
        put: operations["add_organization_member"];
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/organizations/{org_slug}/members/{member_id}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        /** Delete org member */
        delete: operations["delete_organization_member"];
        options?: never;
        head?: never;
        /** Edit org member's role */
        patch: operations["edit_organization_member"];
        trace?: never;
    };
    "/api/organizations/{org_slug}/projects": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get projects */
        get: operations["get_projects"];
        put?: never;
        /** Create project */
        post: operations["create_project"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/organizations/{org_slug}/projects/{project_slug}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get project */
        get: operations["get_project"];
        put?: never;
        post?: never;
        /** Delete project */
        delete: operations["delete_project"];
        options?: never;
        head?: never;
        /** Edit project */
        patch: operations["edit_project"];
        trace?: never;
    };
    "/api/organizations/{org_slug}/projects/{project_slug}/regenerate-keys": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * Regenerate project deploy keys
         * @description These keys are used to pull the repository. You can get the public key from the project details.
         */
        get: operations["regenerate_project_keys"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/organizations/{org_slug}/projects/{project_slug}/regenerate-webhook-secret": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * Regenerate webhook secret
         * @description This secret is used to verify the authenticity of webhooks sent by the repository service. You won't be able to view it again after this call.
         */
        get: operations["regenerate_webhook_secret"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/organizations/{org_slug}/secrets": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get org secrets */
        get: operations["get_organization_secrets"];
        put?: never;
        /** Create org secret */
        post: operations["create_organization_secret"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/organizations/{org_slug}/secrets/{secret_id}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        /** Delete org secret */
        delete: operations["delete_organization_secret"];
        options?: never;
        head?: never;
        /** Edit org secret */
        patch: operations["edit_organization_secret"];
        trace?: never;
    };
    "/api/user": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get user details */
        get: operations["get_user"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/webhook-handler/github": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /**
         * GitHub Webhook handler
         * @description Handles incoming GitHub webhooks
         */
        post: operations["github_webhook_handler"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
}
export type webhooks = Record<string, never>;
export interface components {
    schemas: {
        CreateOrgSecretBody: {
            name: string;
            secret: string;
        };
        CreateProjectBody: {
            name: string;
            repository: components["schemas"]["CreateProjectBodyRepository"];
            slug: string;
        };
        CreateProjectBodyRepository: {
            source: components["schemas"]["ProjectRepositorySource"];
            url: string;
        };
        /** @example {
         *       "success": true,
         *       "id": "60c72b2f9b1d8c001c8e4f5a"
         *     } */
        CreateSuccess: {
            id: string;
            success: boolean;
        };
        EditOrgSecretBody: {
            name?: string | null;
            secret?: string | null;
        };
        EditRoleBody: {
            role: components["schemas"]["OrganizationRole"];
        };
        /** @example {
         *       "error": "You do not have sufficient permissions to perform this action"
         *     } */
        ForbiddenError: {
            error: string;
        };
        Member: {
            _id: string;
            email: string;
            name: string;
            role: components["schemas"]["OrganizationRole"];
        };
        Membership: {
            role: components["schemas"]["OrganizationRole"];
            user_id: string;
        };
        /** @description MutableOrganization is used for creating or updating organization throught the API. */
        MutableOrganization: {
            avatar_email?: string | null;
            description: string;
            name: string;
            slug: string;
        };
        Organization: {
            _id?: string | null;
            avatar_email?: string | null;
            description: string;
            members: components["schemas"]["Membership"][];
            name: string;
            slug: string;
        };
        /** @enum {string} */
        OrganizationRole: "viewer" | "member" | "admin" | "owner";
        /** @enum {string} */
        ProjectRepositorySource: "github" | "gitea" | "genericgit";
        /** @description Project object that can be safely sent to the client */
        PublicProject: {
            _id: string;
            name: string;
            organization_id: string;
            repository: components["schemas"]["PublicProjectRepository"];
            slug: string;
        };
        /** @description ProjectRepository object that can be safely sent to the client */
        PublicProjectRepository: {
            deploy_key_generated: boolean;
            deploy_public_key?: string | null;
            source: components["schemas"]["ProjectRepositorySource"];
            url: string;
            webhook_secret_generated: boolean;
        };
        PublicSecret: {
            _id: string;
            name: string;
            organization_id: string;
            project_id?: string | null;
            scope: components["schemas"]["SecretScope"];
        };
        RegenerateSecretResponse: {
            webhook_secret: string;
        };
        /** @enum {string} */
        SecretScope: "organization" | "project";
        /** @example {
         *       "error": "Unauthorized"
         *     } */
        UnauthorizedError: {
            error: string;
        };
        User: {
            _id: string;
            email: string;
            name: string;
            subject: string;
        };
        /** @example {
         *       "success": true
         *     } */
        WebhookHandlerSuccess: {
            success: boolean;
        };
    };
    responses: never;
    parameters: never;
    requestBodies: never;
    headers: never;
    pathItems: never;
}
export type $defs = Record<string, never>;
export interface operations {
    get_health: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Success */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "text/plain": string;
                };
            };
        };
    };
    log_in: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: never;
    };
    get_organizations: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Success */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Organization"][];
                };
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["UnauthorizedError"];
                };
            };
        };
    };
    create_organization: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["MutableOrganization"];
            };
        };
        responses: {
            /** @description Success */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["CreateSuccess"];
                };
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["UnauthorizedError"];
                };
            };
        };
    };
    get_organization: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Organization slug */
                org_slug: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Success */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Organization"];
                };
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["UnauthorizedError"];
                };
            };
            /** @description Forbidden */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ForbiddenError"];
                };
            };
        };
    };
    delete_organization: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Organization slug */
                org_slug: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Success */
            204: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["UnauthorizedError"];
                };
            };
            /** @description Forbidden */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ForbiddenError"];
                };
            };
        };
    };
    edit_organization: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Organization slug */
                org_slug: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["MutableOrganization"];
            };
        };
        responses: {
            /** @description Success */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["CreateSuccess"];
                };
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["UnauthorizedError"];
                };
            };
            /** @description Forbidden */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ForbiddenError"];
                };
            };
        };
    };
    get_organization_members: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Organization slug */
                org_slug: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Success */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Member"][];
                };
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["UnauthorizedError"];
                };
            };
            /** @description Forbidden */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ForbiddenError"];
                };
            };
        };
    };
    add_organization_member: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Organization slug */
                org_slug: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["Membership"];
            };
        };
        responses: {
            /** @description Success */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["CreateSuccess"];
                };
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["UnauthorizedError"];
                };
            };
            /** @description Forbidden */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ForbiddenError"];
                };
            };
        };
    };
    delete_organization_member: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Organization slug */
                org_slug: string;
                /** @description Member ID */
                member_id: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Success */
            204: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["UnauthorizedError"];
                };
            };
            /** @description Forbidden */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ForbiddenError"];
                };
            };
        };
    };
    edit_organization_member: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Organization slug */
                org_slug: string;
                /** @description Member ID */
                member_id: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["EditRoleBody"];
            };
        };
        responses: {
            /** @description Success */
            204: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["UnauthorizedError"];
                };
            };
            /** @description Forbidden */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ForbiddenError"];
                };
            };
        };
    };
    get_projects: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Organization slug */
                org_slug: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Success */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["PublicProject"][];
                };
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["UnauthorizedError"];
                };
            };
            /** @description Forbidden */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ForbiddenError"];
                };
            };
        };
    };
    create_project: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Organization slug */
                org_slug: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CreateProjectBody"];
            };
        };
        responses: {
            /** @description Success */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["CreateSuccess"];
                };
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["UnauthorizedError"];
                };
            };
            /** @description Forbidden */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ForbiddenError"];
                };
            };
        };
    };
    get_project: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Organization slug */
                org_slug: string;
                /** @description Project slug */
                project_slug: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Success */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["PublicProject"];
                };
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["UnauthorizedError"];
                };
            };
            /** @description Forbidden */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ForbiddenError"];
                };
            };
        };
    };
    delete_project: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Organization slug */
                org_slug: string;
                /** @description Project slug */
                project_slug: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Success */
            204: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["UnauthorizedError"];
                };
            };
            /** @description Forbidden */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ForbiddenError"];
                };
            };
        };
    };
    edit_project: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Organization slug */
                org_slug: string;
                /** @description Project slug */
                project_slug: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CreateProjectBody"];
            };
        };
        responses: {
            /** @description Success */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["CreateSuccess"];
                };
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["UnauthorizedError"];
                };
            };
            /** @description Forbidden */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ForbiddenError"];
                };
            };
        };
    };
    regenerate_project_keys: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Organization slug */
                org_slug: string;
                /** @description Project slug */
                project_slug: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Success */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["PublicProject"];
                };
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["UnauthorizedError"];
                };
            };
            /** @description Forbidden */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ForbiddenError"];
                };
            };
        };
    };
    regenerate_webhook_secret: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Organization slug */
                org_slug: string;
                /** @description Project slug */
                project_slug: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Success */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["RegenerateSecretResponse"];
                };
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["UnauthorizedError"];
                };
            };
            /** @description Forbidden */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ForbiddenError"];
                };
            };
        };
    };
    get_organization_secrets: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Organization slug */
                org_slug: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Success */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["PublicSecret"][];
                };
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["UnauthorizedError"];
                };
            };
            /** @description Forbidden */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ForbiddenError"];
                };
            };
        };
    };
    create_organization_secret: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Organization slug */
                org_slug: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CreateOrgSecretBody"];
            };
        };
        responses: {
            /** @description Success */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["CreateSuccess"];
                };
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["UnauthorizedError"];
                };
            };
            /** @description Forbidden */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ForbiddenError"];
                };
            };
        };
    };
    delete_organization_secret: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Organization slug */
                org_slug: string;
                /** @description Secret ID */
                secret_id: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Success */
            204: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["UnauthorizedError"];
                };
            };
            /** @description Forbidden */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ForbiddenError"];
                };
            };
        };
    };
    edit_organization_secret: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Organization slug */
                org_slug: string;
                /** @description Secret ID */
                secret_id: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["EditOrgSecretBody"];
            };
        };
        responses: {
            /** @description Success */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["CreateSuccess"];
                };
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["UnauthorizedError"];
                };
            };
            /** @description Forbidden */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ForbiddenError"];
                };
            };
        };
    };
    get_user: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Success */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["User"];
                };
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["UnauthorizedError"];
                };
            };
        };
    };
    github_webhook_handler: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "text/plain": string;
            };
        };
        responses: {
            /** @description Success */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["WebhookHandlerSuccess"];
                };
            };
        };
    };
}
