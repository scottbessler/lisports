import type { LoaderArgs } from "@remix-run/node";
import { redirect } from "@remix-run/node";
import { json } from "@remix-run/node";
import {
  NavLink,
  Outlet,
  useLoaderData,
  useLocation,
  useNavigate,
  useParams,
} from "@remix-run/react";
import classNames from "classnames";
import dayjs from "dayjs";
import type { ChangeEventHandler, MouseEventHandler } from "react";
import { useCallback, useRef, useState } from "react";
import { fetchTodaysScoreboard } from "../stores/scoreboard.server";
import { getCurrentBreakpoint } from "../utils";

export async function loader({ request, params }: LoaderArgs) {
  const todaysScoreboard = await fetchTodaysScoreboard();
  const today = todaysScoreboard.gameDate;

  const url = new URL(request.url);
  if (url.pathname === "/nba/scoreboard/today") {
    return redirect(today);
  }
  if (url.pathname === "/nba/scoreboard") {
    const isLiveOrCompletedGames = todaysScoreboard.games.some(
      (g) => g.gameStatus >= 2
    );
    if (isLiveOrCompletedGames) {
      return redirect(today);
    } else {
      const yesterday = dayjs(today).add(-1, "day").format(YMD_FORMAT);
      return redirect(yesterday);
    }
  }

  return json({ todaysScoreboard, today });
}

const YMD_FORMAT = "YYYY-MM-DD";
export default function Scoreboard() {
  const data = useLoaderData<typeof loader>();

  const params = useParams();
  const [currentDay, setCurrentDay] = useState(params.day ?? data.today);

  const currentDayJs = dayjs(currentDay);

  const days: { ymd: string; label: string }[] = [];
  for (let i = -3; i <= 3; i++) {
    const d = currentDayJs.add(i, "day");
    days.push({
      ymd: d.format(YMD_FORMAT),
      label: d.isSame(data.today) ? "Today" : d.format("ddd, MMM DD"),
    });
  }

  const datePickerRef = useRef<HTMLInputElement>(null);
  const navigate = useNavigate();
  const onChangeDate = useCallback<ChangeEventHandler<HTMLInputElement>>(
    (e) => {
      setCurrentDay(e.currentTarget.value);
      navigate(e.currentTarget.value);
    },
    [navigate]
  );

  const onPrev = useCallback(() => {
    setCurrentDay(
      currentDayJs.add(-1 * getNumDaysShowing(), "day").format(YMD_FORMAT)
    );
  }, [currentDayJs]);
  const onNext = useCallback(() => {
    // window.matchMedia("(orientation: portrait)");
    setCurrentDay(
      currentDayJs.add(getNumDaysShowing(), "day").format(YMD_FORMAT)
    );
  }, [currentDayJs]);

  return (
    <div className="flex flex-1 flex-col">
      <ul className="menu menu-compact flex flex-row flex-nowrap items-center justify-center pb-3 md:gap-2">
        <li className="mr-auto lg:ml-auto">
          <button onClick={onPrev}>❮</button>
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
        <li className="ml-auto">
          <button onClick={onNext}>❯</button>
        </li>
        <li tabIndex={0} className="lg:mr-auto">
          <span onClick={() => datePickerRef.current?.showPicker()}>📅</span>
          <input
            ref={datePickerRef}
            className="invisible absolute h-0 w-0"
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
function getNumDaysShowing() {
  switch (getCurrentBreakpoint()) {
    case "0":
    case "sm":
      return 3;
    case "md":
      return 5;
    case "lg":
    default:
      return 7;
  }
}
