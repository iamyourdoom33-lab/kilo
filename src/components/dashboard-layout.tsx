"use client"

import * as React from "react"
import { cn } from "@/lib/utils"

interface DashboardLayoutProps {
  title?: string
  description?: string
  children: React.ReactNode
  className?: string
}

export function DashboardLayout({
  title = "Dashboard",
  description,
  children,
  className,
}: DashboardLayoutProps) {
  return (
    <div className="flex min-h-screen flex-col bg-background">
      <header className="sticky top-0 z-40 w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
        <div className="container flex h-16 items-center justify-between">
          <div className="flex flex-col">
            <h1 className="text-2xl font-bold tracking-tight">{title}</h1>
            {description && (
              <p className="text-sm text-muted-foreground">{description}</p>
            )}
          </div>
        </div>
      </header>
      <main className="flex-1 container py-8">
        <div className={cn("space-y-8", className)}>{children}</div>
      </main>
    </div>
  )
}