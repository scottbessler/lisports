import classNames from 'classnames';
import { useCallback, useEffect, useRef, useState } from 'react';

const THEMES = [
	'light',
	'dark',
	'cupcake',
	'bumblebee',
	'emerald',
	'corporate',
	'synthwave',
	'retro',
	'cyberpunk',
	'valentine',
	'halloween',
	'garden',
	'forest',
	'aqua',
	'lofi',
	'pastel',
	'fantasy',
	'wireframe',
	'black',
	'luxury',
	'dracula',
	'cmyk',
	'autumn',
	'business',
	'acid',
	'lemonade',
	'night',
	'coffee',
	'winter',
] as const;

type ThemeName = (typeof THEMES)[number];

function applyTheme(theme: ThemeName) {
	document.documentElement.setAttribute('data-theme', theme);
	window.localStorage.setItem('theme', theme);
}

export const ThemePicker = () => {
	const [currentTheme, setCurrentTheme] = useState<ThemeName>('light');
	const [isOpen, setIsOpen] = useState(false);
	const rootRef = useRef<HTMLDivElement>(null);

	useEffect(() => {
		const storedTheme = window.localStorage.getItem('theme') as ThemeName | null;
		const initialTheme =
			storedTheme && THEMES.includes(storedTheme)
				? storedTheme
				: ((document.documentElement.getAttribute('data-theme') as ThemeName | null) ?? 'light');
		setCurrentTheme(initialTheme);
		applyTheme(initialTheme);
	}, []);

	useEffect(() => {
		const onPointerDown = (event: MouseEvent) => {
			if (rootRef.current?.contains(event.target as Node)) return;
			setIsOpen(false);
		};

		const onKeyDown = (event: KeyboardEvent) => {
			if (event.key === 'Escape') setIsOpen(false);
		};

		document.addEventListener('mousedown', onPointerDown);
		document.addEventListener('keydown', onKeyDown);
		return () => {
			document.removeEventListener('mousedown', onPointerDown);
			document.removeEventListener('keydown', onKeyDown);
		};
	}, []);

	const onSelectTheme = useCallback((theme: ThemeName) => {
		setCurrentTheme(theme);
		applyTheme(theme);
		setIsOpen(false);
	}, []);

	return (
		<div ref={rootRef} title="Change Theme" className="relative">
			<button
				type="button"
				className={classNames('btn btn-ghost btn-sm gap-1 normal-case', isOpen && 'btn-active')}
				aria-expanded={isOpen}
				aria-haspopup="true"
				onClick={() => setIsOpen((open) => !open)}
			>
				<svg
					width="20"
					height="20"
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					className="h-5 w-5 stroke-current"
				>
					<path
						strokeLinecap="round"
						strokeLinejoin="round"
						strokeWidth="2"
						d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01"
					/>
				</svg>
				<span className="hidden md:inline">Theme</span>
				<svg
					width="12"
					height="12"
					className="hidden h-3 w-3 fill-current opacity-60 sm:inline-block"
					xmlns="http://www.w3.org/2000/svg"
					viewBox="0 0 2048 2048"
				>
					<path d="M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z" />
				</svg>
			</button>
			<div
				className={classNames(
					'absolute top-full right-0 z-[60] mt-2 h-[70vh] max-h-96 w-56 overflow-y-auto rounded-box border border-base-300 bg-base-200 p-2 text-base-content shadow-2xl',
					!isOpen && 'hidden',
				)}
			>
				<div className="grid grid-cols-1 gap-2">
					{THEMES.map((theme) => {
						const isActive = currentTheme === theme;
						return (
							<button
								key={theme}
								type="button"
								onClick={() => onSelectTheme(theme)}
								className={classNames(
									'overflow-hidden rounded-box text-left outline-offset-2 transition-colors',
									isActive && 'ring-2 ring-primary',
								)}
							>
								<div
									data-theme={theme}
									className="w-full cursor-pointer bg-base-100 font-sans text-base-content"
								>
									<div className="flex items-center gap-2 px-4 py-3">
										<svg
											xmlns="http://www.w3.org/2000/svg"
											width="16"
											height="16"
											viewBox="0 0 24 24"
											fill="currentColor"
											className={classNames('h-3 w-3', !isActive && 'invisible')}
										>
											<path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z" />
										</svg>
										<div className="flex-1 text-sm font-bold">{theme}</div>
										<div className="flex flex-shrink-0 flex-wrap gap-1">
											<div className="w-2 rounded bg-primary" />
											<div className="w-2 rounded bg-secondary" />
											<div className="w-2 rounded bg-accent" />
											<div className="w-2 rounded bg-neutral" />
										</div>
									</div>
								</div>
							</button>
						);
					})}
				</div>
			</div>
		</div>
	);
};
