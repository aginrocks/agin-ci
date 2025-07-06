import { IconCircleCheckFilled, IconCloudUpload, IconGitBranch } from '@tabler/icons-react';
import { CardAction } from './card-action';
import { HeroCard } from './hero-card';

export function HeroGraphics() {
    return (
        <div className="">
            <HeroCard className="w-xs">
                <CardAction title="Pushed">
                    <div className="flex mt-1 text-muted-foreground items-center gap-1">
                        <div className="text-xs font-medium">Branch:</div>
                        <div className="text-xs">production</div>
                    </div>
                </CardAction>
            </HeroCard>
            <HeroCard className="w-xs">
                <CardAction
                    title="Build Android"
                    icon={<IconCircleCheckFilled className="text-green-600" />}
                    timing="49s"
                ></CardAction>
            </HeroCard>
            <HeroCard className="w-xs">
                <CardAction
                    title="Build iOS"
                    icon={<IconCircleCheckFilled className="text-green-600" />}
                    timing="1m 20s"
                ></CardAction>
            </HeroCard>
            <HeroCard className="w-xs">
                <CardAction
                    title="Build Server"
                    icon={<IconCircleCheckFilled className="text-green-600" />}
                    timing="34s"
                ></CardAction>
            </HeroCard>
        </div>
    );
}
