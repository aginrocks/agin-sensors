import { DemoComponent } from "@/lib/components/Demo";
import { demoPageStyle } from "./styles";

export default function Demo() {
  return (
    <div className={demoPageStyle}>
      <DemoComponent />
    </div>
  );
}
