"use client";
import {
  IconAdjustments,
  IconAutomation,
  IconBrandGit,
  IconDatabase,
  IconFileUpload,
  IconFilter,
  IconGitMerge,
  IconLock,
  IconPlus,
} from "@tabler/icons-react";
import { FeatureIcon } from "../FeatureIcon";
import {
  privacy,
  privacyHeader,
  privacySubtitle,
  privacyTitle,
} from "./styles";
import { FeatureCard } from "../FeatureCard";
import { FeaturesGrid } from "../FeaturesGrid";

export function Privacy() {
  return (
    <div className={privacy}>
      <div className={privacyHeader}>
        <FeatureIcon icon={IconPlus} variant="gradient" />
        <div className={privacyTitle}>There&apos;s more to that</div>
        <div className={privacySubtitle}>
          Agin Sensors is tailored to your needs. Flexible, extensible, and open
          source.
        </div>
      </div>
      <FeaturesGrid>
        <FeatureCard
          label="Database Flexibility"
          description="Support for multiple time-series databases"
          icon={IconDatabase}
        />
        <FeatureCard
          label="Organization-Based Filtering"
          description="Route sensor data to different organizations based on MAC addresses, tokens, or other metadata"
          icon={IconFilter}
        />
        <FeatureCard
          label="Data Processing Pipeline"
          description="Apply custom modifiers to transform and process sensor data in real-time"
          icon={IconAutomation}
        />
        <FeatureCard
          label="Configuration-Driven"
          description="YAML-based configuration for connectors, databases, and organizations"
          icon={IconAdjustments}
        />
      </FeaturesGrid>
    </div>
  );
}
