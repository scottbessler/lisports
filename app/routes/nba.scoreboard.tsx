import type { LoaderArgs } from "@remix-run/node";
import { redirect } from "@remix-run/node";
import { json } from "@remix-run/node";
import { NavLink, Outlet, useLoaderData } from "@remix-run/react";
import classNames from "classnames";
import dayjs from "dayjs";
import type { ChangeEventHandler, MouseEventHandler } from "react";
import { useCallback, useRef, useState } from "react";
import { fetchTodaysScoreboard } from "../stores/scoreboard.server";

export async function loader({ request, params }: LoaderArgs) {
  const today = await fetchTodaysScoreboard();

  const url = new URL(request.url);
  if (url.pathname === "/nba/scoreboard") {
    const isLiveOrCompletedGames = today.games.some((g) => g.gameStatus >= 2);
    if (isLiveOrCompletedGames) {
      return redirect(`/nba/scoreboard/today`);
    } else {
      const yesterday = dayjs(today.gameDate).add(-1, "day").format(YMD_FORMAT);
      return redirect(`/nba/scoreboard/${yesterday}`);
    }
  }

  return json({ today: today.gameDate });
}

const YMD_FORMAT = "YYYY-MM-DD";
export default function Scoreboard() {
  const data = useLoaderData<typeof loader>();

  const [currentDay, setCurrentDay] = useState(data.today);

  const currentDayJs = dayjs(currentDay);

  const days: { ymd: string; label: string }[] = [];
  for (let i = -3; i <= 3; i++) {
    const d = currentDayJs.add(i, "day");
    if (d.isSame(data.today)) {
      days.push({ ymd: "today", label: "Today" });
    } else {
      days.push({
        ymd: d.format(YMD_FORMAT),
        label: d.format("ddd, MMM DD"),
      });
    }
  }

  const datePickerRef = useRef<HTMLInputElement>(null);
  const onChangeDate = useCallback<ChangeEventHandler<HTMLInputElement>>(
    (e) => {
      setCurrentDay(e.currentTarget.value);
    },
    []
  );
  return (
    <div className="flex flex-1 flex-col">
      <ul className="menu menu-compact flex flex-row flex-nowrap items-center justify-center gap-2 overflow-x-scroll pb-3">
        <li>
          <button
            className="hidden max-md:block"
            onClick={() =>
              setCurrentDay(currentDayJs.add(-3, "day").format(YMD_FORMAT))
            }
          >
            ❮
          </button>
          <button
            className="hidden md:max-lg:block"
            onClick={() =>
              setCurrentDay(currentDayJs.add(-5, "day").format(YMD_FORMAT))
            }
          >
            ❮
          </button>
          <button
            className="hidden lg:block"
            onClick={() =>
              setCurrentDay(currentDayJs.add(-7, "day").format(YMD_FORMAT))
            }
          >
            ❮
          </button>
        </li>
        {days.map((d, i) => (
          <li
            key={d.ymd}
            className={classNames({
              "hidden lg:block": i === 0 || i === 6,
              "hidden md:block": i === 1 || i === 5,
            })}
          >
            <NavLink to={d.ymd}>{d.label}</NavLink>
          </li>
        ))}
        <li>
          {" "}
          <button
            className="hidden max-md:block"
            onClick={() =>
              setCurrentDay(currentDayJs.add(3, "day").format(YMD_FORMAT))
            }
          >
            ❯
          </button>
          <button
            className="hidden md:max-lg:block"
            onClick={() =>
              setCurrentDay(currentDayJs.add(5, "day").format(YMD_FORMAT))
            }
          >
            ❯
          </button>
          <button
            className="hidden lg:block"
            onClick={() =>
              setCurrentDay(currentDayJs.add(7, "day").format(YMD_FORMAT))
            }
          >
            ❯
          </button>
        </li>
        <li tabIndex={0}>
          <span onClick={() => datePickerRef.current?.showPicker()}>📅</span>
          <input
            ref={datePickerRef}
            className="invisible absolute"
            type="date"
            value={currentDayJs.format(YMD_FORMAT)}
            onChange={onChangeDate}
          />
        </li>
      </ul>
      <Outlet />
    </div>
  );
}
