'use client';
import { IconCircleCheckFilled } from '@tabler/icons-react';
import { ArcherContainer, ArcherElement } from 'react-archer';
import { CardAction } from './card-action';
import { HeroCard } from './hero-card';

export function HeroGraphics() {
    return (
        <div className="w-full">
            <ArcherContainer strokeColor="gray">
                <div className="flex items-center justify-center gap-16">
                    <ArcherElement
                        id="root"
                        relations={[
                            {
                                targetId: 'android',
                                targetAnchor: 'left',
                                sourceAnchor: 'right',
                                style: {
                                    endMarker: false,
                                    strokeColor: 'var(--color-neutral-800)',
                                },
                            },
                            {
                                targetId: 'ios',
                                targetAnchor: 'left',
                                sourceAnchor: 'right',
                                style: {
                                    endMarker: false,
                                    strokeColor: 'var(--color-neutral-800)',
                                },
                            },
                            {
                                targetId: 'web',
                                targetAnchor: 'left',
                                sourceAnchor: 'right',
                                style: {
                                    endMarker: false,
                                    strokeColor: 'var(--color-neutral-800)',
                                },
                            },
                            {
                                targetId: 'server',
                                targetAnchor: 'left',
                                sourceAnchor: 'right',
                                style: {
                                    endMarker: false,
                                    strokeColor: 'var(--color-neutral-800)',
                                },
                            },
                        ]}
                    >
                        <div>
                            <HeroCard className="w-xs">
                                <CardAction title="Pushed">
                                    <div className="flex items-center gap-1 mt-1 text-muted-foreground">
                                        <div className="text-xs font-medium">Branch:</div>
                                        <div className="text-xs">production</div>
                                    </div>
                                </CardAction>
                            </HeroCard>
                        </div>
                    </ArcherElement>

                    <div className="flex flex-col gap-4">
                        <ArcherElement
                            id="android"
                            relations={[
                                {
                                    targetId: 'playstore',
                                    targetAnchor: 'left',
                                    sourceAnchor: 'right',
                                    style: {
                                        endMarker: false,
                                        strokeColor: 'var(--color-neutral-800)',
                                    },
                                },
                            ]}
                        >
                            <div>
                                <HeroCard className="w-xs">
                                    <CardAction
                                        title="Build Android"
                                        icon={<IconCircleCheckFilled className="text-green-600" />}
                                        timing="49s"
                                    />
                                </HeroCard>
                            </div>
                        </ArcherElement>

                        <ArcherElement
                            id="ios"
                            relations={[
                                {
                                    targetId: 'appstore',
                                    targetAnchor: 'left',
                                    sourceAnchor: 'right',
                                    style: {
                                        endMarker: false,
                                        strokeColor: 'var(--color-neutral-800)',
                                    },
                                },
                            ]}
                        >
                            <div>
                                <HeroCard className="w-xs">
                                    <CardAction
                                        title="Build iOS"
                                        icon={<IconCircleCheckFilled className="text-green-600" />}
                                        timing="1m 20s"
                                    />
                                </HeroCard>
                            </div>
                        </ArcherElement>

                        <ArcherElement
                            id="web"
                            relations={[
                                {
                                    targetId: 'deploy',
                                    targetAnchor: 'left',
                                    sourceAnchor: 'right',
                                    style: {
                                        endMarker: false,
                                        strokeColor: 'var(--color-neutral-800)',
                                    },
                                },
                            ]}
                        >
                            <div>
                                <HeroCard className="w-xs">
                                    <CardAction
                                        title="Build Web"
                                        icon={<IconCircleCheckFilled className="text-green-600" />}
                                        timing="26s"
                                    />
                                </HeroCard>
                            </div>
                        </ArcherElement>

                        <ArcherElement
                            id="server"
                            relations={[
                                {
                                    targetId: 'deploy',
                                    targetAnchor: 'left',
                                    sourceAnchor: 'right',
                                    style: {
                                        endMarker: false,
                                        strokeColor: 'var(--color-neutral-800)',
                                    },
                                },
                            ]}
                        >
                            <div>
                                <HeroCard className="w-xs">
                                    <CardAction
                                        title="Build Server"
                                        icon={<IconCircleCheckFilled className="text-green-600" />}
                                        timing="34s"
                                    />
                                </HeroCard>
                            </div>
                        </ArcherElement>
                    </div>
                    <div className="flex flex-col gap-4">
                        <ArcherElement id="playstore">
                            <div>
                                <HeroCard className="w-xs">
                                    <CardAction
                                        title="Submit to Play Store"
                                        icon={<IconCircleCheckFilled className="text-green-600" />}
                                        timing="17s"
                                    />
                                </HeroCard>
                            </div>
                        </ArcherElement>

                        <ArcherElement id="appstore">
                            <div>
                                <HeroCard className="w-xs">
                                    <CardAction
                                        title="Submit to App Store"
                                        icon={<IconCircleCheckFilled className="text-green-600" />}
                                        timing="35s"
                                    />
                                </HeroCard>
                            </div>
                        </ArcherElement>

                        <ArcherElement id="deploy">
                            <div>
                                <HeroCard className="w-xs">
                                    <CardAction
                                        title="Deploy Code"
                                        icon={<IconCircleCheckFilled className="text-green-600" />}
                                        timing="12s"
                                    />
                                </HeroCard>
                            </div>
                        </ArcherElement>
                    </div>
                </div>
            </ArcherContainer>
        </div>
    );
}
