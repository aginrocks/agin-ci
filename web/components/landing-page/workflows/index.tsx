'use client';
import { ReactNode, useState } from 'react';
import { WorkflowsTabs } from './tabs';
import Image from 'next/image';
import { WorkflowCard } from './workflow-card';
import {
    IconBrandAndroid,
    IconBrandApple,
    IconBrandDocker,
    IconBrandReactNative,
    IconBrandRust,
    IconPackage,
} from '@tabler/icons-react';
import { AnimatePresence, motion } from 'motion/react';

export type WorkflowsProps = {
    codeBlocks: Record<string, ReactNode>;
};

export function Workflows({ codeBlocks }: WorkflowsProps) {
    const [workflow, setWorkflow] = useState('mobile');
    return (
        <div className="w-full flex flex-col items-center pt-4 relative">
            {/* <div className="text-4xl font-medium text-center mb-5">
                The Speed Your Workflow Deserves
            </div>
            <div className="text-center text-muted-foreground w-3xl leading-relaxed">
                We don't slow you down. Instant feedback loops and fast builds let you stay in flow
                and ship confidently — every time.
            </div> */}
            <div className="w-full max-w-6xl flex justify-between items-center gap-6">
                <div className="flex-1">
                    <div className="text-4xl font-medium mb-5">CI That Adapts to Your Stack</div>
                    <div className="text-muted-foreground w-3xl leading-relaxed">
                        Our system seamlessly supports the workflows you rely on — whether you're
                        building mobile apps with React Native and Expo, compiling high-performance
                        Rust code, or packaging applications into Docker containers.
                    </div>
                    <WorkflowsTabs tab={workflow} onTabChange={setWorkflow} />
                    <div className="mt-6 h-34.5 relative">
                        <AnimatePresence>
                            <motion.div
                                key={workflow}
                                className="absolute inset-0"
                                initial={{ opacity: 0, scale: 0.97 }}
                                animate={{ opacity: 1, scale: 1 }}
                                exit={{ opacity: 0, scale: 0.97 }}
                                transition={{ duration: 0.3 }}
                            >
                                {workflow === 'mobile' && (
                                    <WorkflowCard
                                        description="Build Android, iOS, and Expo apps with minimal setup. Our CI system includes smart caching for faster builds and first-party support for Expo projects, ensuring smooth integration with both managed and bare workflows."
                                        icons={[
                                            IconBrandAndroid,
                                            IconBrandApple,
                                            IconBrandReactNative,
                                        ]}
                                    />
                                )}
                                {workflow === 'rust' && (
                                    <WorkflowCard
                                        description="Build Rust projects with confidence. Our CI system supports native Rust tooling, including multi-crate workspaces and custom toolchains, with fast incremental builds powered by sccache and dependency caching."
                                        icons={[IconBrandRust]}
                                    />
                                )}
                                {workflow === 'docker' && (
                                    <WorkflowCard
                                        description="Build and package container images with ease. Our CI system supports Docker and OCI formats with efficient layer caching and clean build environments, ensuring fast, consistent builds for single or multi-platform targets."
                                        icons={[IconPackage, IconBrandDocker]}
                                    />
                                )}
                            </motion.div>
                        </AnimatePresence>
                    </div>
                    {/* <CodeBlock lang="yaml">- name: Build and Test Mobile App</CodeBlock> */}
                </div>
                <div>
                    <Image
                        src="/landing-page/mobile.png"
                        alt="iPhone with a mobile app opened"
                        width={300}
                        height={591.1}
                    />
                </div>
            </div>
        </div>
    );
}
