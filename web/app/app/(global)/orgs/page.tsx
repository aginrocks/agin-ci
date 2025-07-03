import { PageHeader } from '@components/page-header';
import { Button } from '@components/ui/button';
import { IconPlus } from '@tabler/icons-react';
import Link from 'next/link';

export default function Page() {
    return (
        <>
            <PageHeader
                path={[
                    {
                        label: 'Organizations',
                    },
                ]}
                rightSection={
                    <Button asChild>
                        <Link href="/app/orgs/new">
                            <IconPlus />
                            Create Organization
                        </Link>
                    </Button>
                }
            />
            <div className="flex flex-1 flex-col gap-4 p-4 pt-0">
                <div className="grid auto-rows-min gap-4 md:grid-cols-3">
                    <div className="bg-muted/50 aspect-video rounded-xl" />
                    <div className="bg-muted/50 aspect-video rounded-xl" />
                    <div className="bg-muted/50 aspect-video rounded-xl" />
                </div>
                <div className="bg-muted/50 min-h-[100vh] flex-1 rounded-xl md:min-h-min" />
            </div>
        </>
    );
}
