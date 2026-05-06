import Link from "next/link"

export default function Home() {
  return (
    <div className="flex min-h-screen flex-col items-center justify-center bg-gradient-to-br from-slate-50 to-slate-100 dark:from-slate-950 dark:to-slate-900">
      <div className="container flex max-w-2xl flex-col items-center gap-8 px-4 py-16 text-center">
        <h1 className="text-5xl font-bold tracking-tight text-slate-900 dark:text-slate-100">
          Clone App Builder
        </h1>
        <p className="text-xl text-slate-600 dark:text-slate-400">
          Transform any website into a customizable Next.js project with interactive
          real-time UI components and live progress tracking.
        </p>
        <Link
          href="/dashboard"
          className="inline-flex h-12 items-center justify-center rounded-lg bg-slate-900 px-8 text-base font-medium text-white transition-colors hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200"
        >
          Get Started
        </Link>
      </div>
    </div>
  )
}