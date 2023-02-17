import { NavLink, Outlet } from "@remix-run/react";
import dayjs from "dayjs";

export default function Scoreboard() {
  const today = dayjs();
  const days: string[] = [];
  for (let i = 0; i <= 7; i++) {
    days.push(today.add(-7 + i, "day").format("YYYY-MM-DD"));
  }

  return (
    <div className="flex flex-col">
      <ul className="menu menu-compact flex flex-row py-3 px-3">
        {days.map((d) => (
          <li key={d}>
            <NavLink className="rounded" to={d}>
              {d}
            </NavLink>
          </li>
        ))}
      </ul>
      <Outlet />
    </div>
  );
}
