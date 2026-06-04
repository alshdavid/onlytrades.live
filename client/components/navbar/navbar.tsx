import { h } from "preact";
import { useInject } from "../../kit/mvvm/provider.ts";
import { useSubscribe } from "../../kit/mvvm/use-async.ts";
import {
  ProfilePermission,
  ProfileService,
} from "../../services/profile-service.ts";

type NavbarProps = {};

export function Navbar({}: NavbarProps) {
  const profileService = useInject(ProfileService);
  const [profile, profileError] = useSubscribe(profileService.profile);

  if (!profile || profileError) {
    return (
      <header className="top-nav app">
        <div class="content content-max-width l">
          <div class="title">
            <img
              height="32px"
              width="32px"
              src="/assets/icons/logo.efdeee9c033df29d5d83.svg"
              alt="OnlyTrades"
            />
            <span>OnlyTrades</span>
          </div>
          <nav />
          <div class="menu" />
        </div>
      </header>
    );
  }

  return (
    <header className="top-nav app">
      <div class="content content-max-width l">
        <a href="/dashboard" class="title">
          <img
            height="32px"
            width="32px"
            src="/assets/icons/logo.efdeee9c033df29d5d83.svg"
            alt="OnlyTrades"
          />
          <span>OnlyTrades</span>
        </a>
        <nav>
          <a href="/dashboard">DASHBOARD</a>
          <a href="/triggers">TRIGGERS</a>
          <a href="/bots">BOTS</a>
          <a href="/marketplace">MARKETPLACE</a>
          {profile.permissions.has(ProfilePermission.Admin) && (
            <a href="/admin">ADMIN</a>
          )}
        </nav>
        <div class="menu">
          {/* <button className="btn blue solid">Account</button> */}
          <a className="btn blue" href="/api/auth/logout">
            Logout
          </a>
        </div>
      </div>
    </header>
  );
}
