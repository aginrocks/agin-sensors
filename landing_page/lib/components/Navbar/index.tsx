import Logo from "../Logo";
import NavbarLink from "./NavbarLink";
import { container, linksContainer, menu, menuLogo } from "./styles";
import Link from "next/link";
import { demoUrl, repoUrl } from "@/lib/config";

export function Navbar() {
  return (
    <div className={container}>
      <div className={menu}>
        <div className={menuLogo}>
          <Logo size="sm" />
        </div>
        <div className={linksContainer}>
          <Link href="/">
            <NavbarLink label="Home" active />
          </Link>
          <a href={repoUrl} target="_blank">
            <NavbarLink label="GitHub" />
          </a>
        </div>
        <Link href={demoUrl} target="_blank">
          <NavbarLink label="Try demo" primary />
        </Link>
      </div>
    </div>
  );
}
