import {
  IconActivity,
  IconBook,
  IconDownload,
  IconFlame,
} from "@tabler/icons-react";
import { FeatureIcon } from "../FeatureIcon";
import {
  actions,
  blur,
  blurContainer,
  download,
  header,
  subtitle,
  title,
} from "./styles";
import { Button } from "../Button";
import { demoUrl, repoUrl } from "@/lib/config";

export function Download() {
  return (
    <div className={download}>
      <div className={header}>
        <FeatureIcon icon={IconActivity} variant="gradient" />
        <div className={title}>Get Started with Agin Sensors</div>
        <div className={subtitle}>
          Look at the demo, read documentation, or set it up yourself.
        </div>
      </div>
      <div className={actions}>
        <a href={"/demo"}>
          <Button variant="primary" icon={IconDownload}>
            Try demo
          </Button>
        </a>
        <a href={repoUrl} target="_blank">
          <Button>Visit GitHub</Button>
        </a>
      </div>
      <div className={blurContainer}>
        <div className={blur}></div>
      </div>
    </div>
  );
}
