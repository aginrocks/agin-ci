export * from './project';
export * from './org';
export * from './webhook-secret';
export * from './system';
export * from './god';
export * from './runners';

export type AdditionalParams = {
    onSuccess?: () => void;
    onError?: () => void;
};
