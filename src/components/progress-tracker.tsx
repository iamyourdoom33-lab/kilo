"use client"

import * as React from "react"
import { Progress } from "@/components/ui/progress"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { StatusIndicator } from "@/components/ui/status-indicator"
import { cn } from "@/lib/utils"

interface Step {
  id: string
  label: string
  status: "idle" | "pending" | "success" | "error" | "processing"
}

interface ProgressTrackerProps {
  steps: Step[]
  className?: string
}

export function ProgressTracker({ steps, className }: ProgressTrackerProps) {
  const completedSteps = steps.filter(s => s.status === "success").length
  const progress = (completedSteps / steps.length) * 100

  return (
    <Card className={cn(className)}>
      <CardHeader>
        <CardTitle>Progress Tracker</CardTitle>
        <CardDescription>
          {completedSteps} of {steps.length} steps completed
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <Progress value={progress} className="h-2" />
        <div className="space-y-3">
          {steps.map((step) => (
            <div key={step.id} className="flex items-center justify-between">
              <span className="text-sm font-medium">{step.label}</span>
              <StatusIndicator status={step.status} />
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  )
}