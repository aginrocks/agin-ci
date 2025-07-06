'use client';
import { ArcherContainer, ArcherElement } from 'react-archer';
import { CardAction } from './card-action';
import { HeroCard } from './hero-card';
import { useSimulatedLoading } from './use-simulated-loading';

export function HeroGraphics() {
    const androidTiming = useSimulatedLoading({ delay: 0.8, startSeconds: 49 });
    const iosTiming = useSimulatedLoading({ delay: 1.5, startSeconds: 80 });
    const webTiming = useSimulatedLoading({ delay: 0.6, startSeconds: 26 });
    const serverTiming = useSimulatedLoading({ delay: 1.7, startSeconds: 34 });
    const playStoreTiming = useSimulatedLoading({ delay: 2, startSeconds: 17 });
    const appStoreTiming = useSimulatedLoading({ delay: 2.3, startSeconds: 35 });
    const deployTiming = useSimulatedLoading({ delay: 2.2, startSeconds: 12 });

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
                                        status={androidTiming.status}
                                        timing={androidTiming.formattedSeconds}
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
                                        status={iosTiming.status}
                                        timing={iosTiming.formattedSeconds}
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
                                        status={webTiming.status}
                                        timing={webTiming.formattedSeconds}
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
                                        status={serverTiming.status}
                                        timing={serverTiming.formattedSeconds}
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
                                        status={playStoreTiming.status}
                                        timing={playStoreTiming.formattedSeconds}
                                    />
                                </HeroCard>
                            </div>
                        </ArcherElement>

                        <ArcherElement id="appstore">
                            <div>
                                <HeroCard className="w-xs">
                                    <CardAction
                                        title="Submit to App Store"
                                        status={appStoreTiming.status}
                                        timing={appStoreTiming.formattedSeconds}
                                    />
                                </HeroCard>
                            </div>
                        </ArcherElement>

                        <ArcherElement id="deploy">
                            <div>
                                <HeroCard className="w-xs">
                                    <CardAction
                                        title="Deploy Code"
                                        status={deployTiming.status}
                                        timing={deployTiming.formattedSeconds}
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
