"use client";
import { IconChartHistogram, IconFlame } from "@tabler/icons-react";
import { FeatureIcon } from "../FeatureIcon";
import {
  actions,
  blur,
  blurContainer,
  download,
  header,
  inline,
  subtitle,
  title,
  bold,
  subtitle2,
} from "./styles";
import { Button } from "../Button";
import { demoUrl } from "@/lib/config";
import { css } from "@/styled-system/css";
import { io, Socket } from "socket.io-client";
import { useEffect, useState } from "react";

export function DemoComponent() {
  const [socket, setSocket] = useState<Socket | null>(null);

  useEffect(() => {
    const socket = io("https://sensors.agin.rocks", {
      extraHeaders: {
        Authorization: `Organization token1`,
      },
      autoConnect: true,
    });
    setSocket(socket);

    return () => {
      socket?.disconnect();
    };
  }, []);

  type DataPacket = {
    bucket: string;
    measurement: string;
    ts: number;
    values: Record<string, number>;
  };

  function randomDataPacket(): DataPacket {
    return {
      bucket: "test-bucket",
      measurement: "test-measurement",
      ts: Date.now(),
      values: {
        x: Math.random() * 100,
        y: Math.random() * 100,
        z: Math.random() * 100,
      },
    };
  }

  function sendData() {
    if (socket) {
      socket.emit("measurement", randomDataPacket());
    }
  }

  return (
    <div className={download}>
      <div className={header}>
        <FeatureIcon icon={IconFlame} variant="gradient" />
        <div className={title}>Try Agin Sensors</div>
        <div className={subtitle2}>Credentials for demo:</div>
        <div className={inline}>
          <div className={bold}>User:</div>
          <div className={subtitle}>demo</div>
        </div>
        <div className={inline}>
          <div className={bold}>Password:</div>
          <div className={subtitle}>demo</div>
        </div>
        <div className={css({ height: "10px" })}></div>
        <div className={subtitle2}>
          You can spike the data in real time with button bellow
        </div>
      </div>
      <div className={actions}>
        <a href={demoUrl} target="_blank">
          <Button variant="primary" icon={IconChartHistogram}>
            Open demo
          </Button>
        </a>
        <Button onClick={sendData}>Spike data</Button>
      </div>
      <div className={blurContainer}>
        <div className={blur}></div>
      </div>
    </div>
  );
}
