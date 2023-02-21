import { useEffect } from "react";
import { themeChange } from "theme-change";

export const ThemePicker = () => {
  useEffect(() => {
    themeChange(false);
  }, []);

  return (
    <div title="Change Theme" className="dropdown-end dropdown ">
      <div tabIndex={0} className="btn-ghost btn gap-1 normal-case">
        <svg
          width="20"
          height="20"
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          className="inline-block h-5 w-5 stroke-current md:h-6 md:w-6"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth="2"
            d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01"
          ></path>
        </svg>{" "}
        <span className="hidden md:inline">Theme</span>{" "}
        <svg
          width="12px"
          height="12px"
          className="ml-1 hidden h-3 w-3 fill-current opacity-60 sm:inline-block"
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 2048 2048"
        >
          <path d="M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z"></path>
        </svg>
      </div>{" "}
      <div className="dropdown-content rounded-t-box rounded-b-box top-px  mt-16 h-[70vh] max-h-96 w-52 overflow-y-auto bg-base-200 text-base-content shadow-2xl ">
        <div className="grid grid-cols-1 gap-3 p-3" tabIndex={0}>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="light"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="light"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">light</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="dark"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="dark"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">dark</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="cupcake"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="cupcake"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">cupcake</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="bumblebee"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="bumblebee"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">bumblebee</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="emerald"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="emerald"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">emerald</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="corporate"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="corporate"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">corporate</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="synthwave"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="synthwave"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">synthwave</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="retro"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="retro"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">retro</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="cyberpunk"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="cyberpunk"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">cyberpunk</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="valentine"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="valentine"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">valentine</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="halloween"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="halloween"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">halloween</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="garden"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="garden"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">garden</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="forest"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="forest"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">forest</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="aqua"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="aqua"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">aqua</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="lofi"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="lofi"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">lofi</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="pastel"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="pastel"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">pastel</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="fantasy"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="fantasy"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">fantasy</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="wireframe"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="wireframe"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">wireframe</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="black"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="black"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">black</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="luxury"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="luxury"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">luxury</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="dracula"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="dracula"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">dracula</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="cmyk"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="cmyk"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">cmyk</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="autumn"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="autumn"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">autumn</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="business"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="business"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">business</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="acid"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="acid"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">acid</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="lemonade"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="lemonade"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">lemonade</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="night"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="night"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">night</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="coffee"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="coffee"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">coffee</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
          <button
            className="overflow-hidden rounded-lg text-left outline-base-content"
            data-set-theme="winter"
            data-act-classname="[&amp;_svg]:visible"
          >
            <div
              data-theme="winter"
              className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
            >
              <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex items-center gap-2 py-3 px-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    className="invisible h-3 w-3"
                  >
                    <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                  </svg>{" "}
                  <div className="flex-grow text-sm font-bold">winter</div>{" "}
                  <div className="flex h-full flex-shrink-0 flex-wrap gap-1">
                    <div className="w-2 rounded bg-primary"></div>{" "}
                    <div className="w-2 rounded bg-secondary"></div>{" "}
                    <div className="w-2 rounded bg-accent"></div>{" "}
                    <div className="w-2 rounded bg-neutral"></div>
                  </div>
                </div>
              </div>
            </div>
          </button>
        </div>
      </div>
    </div>
  );
};
