"use client";
import Logo from "../Logo";
import NavbarLink from "./NavbarLink";
import { container, linksContainer, menu, menuLogo } from "./styles";
import Link from "next/link";
import { repoUrl } from "@/lib/config";
import { usePathname } from "next/navigation";

export function Navbar() {
  const path = usePathname();

  return (
    <div className={container}>
      <div className={menu}>
        <div className={menuLogo}>
          <Logo size="sm" />
        </div>
        <div className={linksContainer}>
          <Link href="/">
            <NavbarLink label="Home" active={path == "/"} />
          </Link>
          <a href={repoUrl} target="_blank">
            <NavbarLink label="GitHub" />
          </a>
        </div>
        <Link href={"/demo"}>
          <NavbarLink label="Try demo" primary active={path == "/demo"} />
        </Link>
      </div>
    </div>
  );
}
