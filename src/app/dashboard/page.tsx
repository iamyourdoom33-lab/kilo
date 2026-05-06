"use client"

import { DashboardLayout } from "@/components/dashboard-layout"
import { ProgressTracker } from "@/components/progress-tracker"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"

const initialSteps = [
  { id: "scraping", label: "Web Scraping", status: "idle" as const },
  { id: "analysis", label: "AI Analysis", status: "idle" as const },
  { id: "generation", label: "Code Generation", status: "idle" as const },
  { id: "export", label: "Export Project", status: "idle" as const },
]

export default function DashboardPage() {
  return (
    <DashboardLayout
      title="Clone App Builder"
      description="Transform any website into a customizable Next.js project"
    >
      <div className="grid gap-6 md:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle>Website URL</CardTitle>
            <CardDescription>Enter the URL of the website you want to clone</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              <input
                type="url"
                placeholder="https://example.com"
                className="w-full rounded-md border px-3 py-2"
              />
              <Button className="w-full">Start Cloning</Button>
            </div>
          </CardContent>
        </Card>

        <ProgressTracker steps={initialSteps} />
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Generated Components</CardTitle>
          <CardDescription>Live preview of detected components</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="text-center py-12 text-muted-foreground">
            Components will appear here after analysis
          </div>
        </CardContent>
      </Card>
    </DashboardLayout>
  )
}