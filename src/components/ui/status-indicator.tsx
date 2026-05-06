"use client"

import * as React from "react"
import { cn } from "@/lib/utils"

interface StatusIndicatorProps {
  status: "idle" | "pending" | "success" | "error" | "processing"
  label?: string
  className?: string
}

const statusConfig = {
  idle: { color: "bg-gray-300", text: "text-gray-600", label: "Idle" },
  pending: { color: "bg-yellow-400 animate-pulse", text: "text-yellow-700", label: "Pending" },
  processing: { color: "bg-blue-400 animate-pulse", text: "text-blue-700", label: "Processing" },
  success: { color: "bg-green-400", text: "text-green-700", label: "Complete" },
  error: { color: "bg-red-400", text: "text-red-700", label: "Error" },
}

export function StatusIndicator({ status, label, className }: StatusIndicatorProps) {
  const config = statusConfig[status]
  
  return (
    <div className={cn("flex items-center gap-2", className)}>
      <div className={cn("h-2.5 w-2.5 rounded-full", config.color)} />
      <span className={cn("text-sm font-medium", config.text)}>
        {label || config.label}
      </span>
    </div>
  )
}