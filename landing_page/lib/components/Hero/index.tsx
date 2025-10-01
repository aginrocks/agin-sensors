"use client";
import { useEffect, useState } from "react";
import { Title } from "../Title";
import {
  content,
  hero,
  heroSubtitle,
  heroTextBox,
  heroTitle,
  screenshot,
  screenshotContainer,
  secondaryGrid,
} from "./styles";
import { useMouse } from "@mantine/hooks";

export function Hero() {
  const { ref, x, y } = useMouse();

  return (
    <div className={hero} ref={ref}>
      <div
        className={secondaryGrid}
        style={{
          maskPosition: `${x - 200}px ${y - 200}px`,
        }}></div>
      <div className={content}>
        <div className={heroTextBox}>
          <div className={heroTitle}>Highly Customizable Sensor System</div>
          <div className={heroSubtitle}>
            Effortlessly collect and manage data from various sensors with Agin
            Sensors, a flexible and easy-to-use solution for your projects.
          </div>
        </div>
        <div className={screenshotContainer}>
          <img src="/images/demo.png" className={screenshot} />
        </div>
      </div>
    </div>
  );
}
